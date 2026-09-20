//! AIS parser-level tests: frame filtering, fragment reassembly, reset.
#![cfg(feature = "ais")]

use nmea_0183_rs::ais::transmit::{AisChannel, AisEncodable, AisTransmitOptions, SafetyBroadcast};
use nmea_0183_rs::ais::{AisDecodeError, AisDecodeOutcome, AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn ignores_nmea_dollar_frames() {
    let mut parser = AisParser::new();
    let frame = parse_frame("$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*77")
        .expect("valid NMEA sentence");
    assert!(
        matches!(parser.decode(&frame), Ok(AisDecodeOutcome::Ignored)),
        "parser should ignore $ NMEA frames"
    );
}

#[test]
fn type8_now_decoded() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,85Mv070j2d>=<e<<=PQhhg`59P00,0*26").expect("valid Type 8 frame");
    match parser.decode(&frame) {
        Ok(AisDecodeOutcome::Message(AisMessage::BinaryBroadcast(bb))) => assert!(bb.mmsi > 0),
        other => panic!("expected BinaryBroadcast (type 8), got {other:?}"),
    }
}

#[test]
fn truncated_payloads_return_errors_no_panic() {
    let mut parser = AisParser::new();
    // Each payload is far shorter than the message type's minimum bit length,
    // so the public decoder must report InvalidMessage (never panic).
    for frame_str in [
        "!AIVDM,1,1,,A,1,0*17", // type 1 (needs >= 144 bits) - 6 bits
        "!AIVDM,1,1,,A,5,0*13", // type 5 (needs >= 424) - 6 bits
        "!AIVDM,1,1,,A,H,0*6E", // type 24 (needs >= 160) - 6 bits
    ] {
        let frame = parse_frame(frame_str).expect("frame parses");
        assert!(matches!(
            parser.decode(&frame),
            Err(AisDecodeError::InvalidMessage { .. })
        ));
    }
}

#[test]
fn separates_interleaved_vdm_and_vdo_fragments() {
    let vdm = SafetyBroadcast {
        repeat_indicator: 0,
        mmsi: 111_111_111,
        text: "A".repeat(100),
    }
    .to_sentences(AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(0))
    .expect("encode VDM");
    let vdo = SafetyBroadcast {
        repeat_indicator: 0,
        mmsi: 222_222_222,
        text: "B".repeat(100),
    }
    .to_sentences(AisTransmitOptions::vdo(AisChannel::A).with_sequence_id(0))
    .expect("encode VDO");
    assert_eq!(vdm.len(), 2);
    assert_eq!(vdo.len(), 2);

    let mut parser = AisParser::new();
    assert!(matches!(
        parser.decode(&parse_frame(&vdm[0]).expect("parse VDM fragment one")),
        Ok(AisDecodeOutcome::Pending)
    ));
    assert!(matches!(
        parser.decode(&parse_frame(&vdo[0]).expect("parse VDO fragment one")),
        Ok(AisDecodeOutcome::Pending)
    ));
    assert!(matches!(
        parser.decode(&parse_frame(&vdm[1]).expect("parse VDM fragment two")),
        Ok(AisDecodeOutcome::Message(AisMessage::Safety(message)))
            if message.mmsi == 111_111_111 && message.text == "A".repeat(100)
    ));
    assert!(matches!(
        parser.decode(&parse_frame(&vdo[1]).expect("parse VDO fragment two")),
        Ok(AisDecodeOutcome::Message(AisMessage::Safety(message)))
            if message.mmsi == 222_222_222 && message.text == "B".repeat(100)
    ));
}

#[test]
fn reset_clears_vdm_and_vdo_fragments() {
    let vdm = SafetyBroadcast {
        repeat_indicator: 0,
        mmsi: 111_111_111,
        text: "A".repeat(100),
    }
    .to_sentences(AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(0))
    .expect("encode VDM");
    let vdo = SafetyBroadcast {
        repeat_indicator: 0,
        mmsi: 222_222_222,
        text: "B".repeat(100),
    }
    .to_sentences(AisTransmitOptions::vdo(AisChannel::A).with_sequence_id(0))
    .expect("encode VDO");

    let mut parser = AisParser::new();
    assert!(matches!(
        parser.decode(&parse_frame(&vdm[0]).expect("parse VDM")),
        Ok(AisDecodeOutcome::Pending)
    ));
    assert!(matches!(
        parser.decode(&parse_frame(&vdo[0]).expect("parse VDO")),
        Ok(AisDecodeOutcome::Pending)
    ));
    parser.reset();
    assert!(matches!(
        parser.decode(&parse_frame(&vdm[1]).expect("parse VDM")),
        Err(AisDecodeError::UnexpectedFragment)
    ));
    assert!(matches!(
        parser.decode(&parse_frame(&vdo[1]).expect("parse VDO")),
        Err(AisDecodeError::UnexpectedFragment)
    ));
}

#[test]
fn decode_distinguishes_outcomes() {
    use AisDecodeOutcome::{Ignored, Message, Pending};
    // Synthetic field/error probes, deliberately using compatible framing.
    let mut p = AisParser::new();
    for (line, expected) in [
        ("$GPRMC,", Ok(Ignored)),
        ("$AIVDM,1,1,,A,1,0", Ok(Ignored)),
        ("!AIABM,", Ok(Ignored)),
        ("!AIVDM,2,1,0,A,1,0", Ok(Pending)),
        ("!AIVDM,1,1,,A,X,0", Err(AisDecodeError::InvalidArmor)),
        (
            "!AIVDM,1,1,,A,1,0",
            Err(AisDecodeError::InvalidMessage { msg_type: Some(1) }),
        ),
        (
            "!AIVDM,1,1,,A,,0",
            Err(AisDecodeError::InvalidMessage { msg_type: None }),
        ),
        ("!AIVDM,1,1,,A,,1", Err(AisDecodeError::InvalidArmor)),
        (
            "!AIVDM,1,1,,A,L,0",
            Ok(Message(AisMessage::Unknown { msg_type: 28 })),
        ),
    ] {
        assert_eq!(
            p.decode(&parse_frame(line).expect("frame")),
            expected,
            "{line}"
        );
    }
    let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26")
        .expect("existing Type 1 fixture");
    assert!(matches!(
        p.decode(&frame),
        Ok(Message(AisMessage::Position(_)))
    ));
}

#[test]
fn decode_preserves_stream_isolation_and_reset() {
    let options = [
        AisTransmitOptions::vdm(AisChannel::A),
        AisTransmitOptions::vdo(AisChannel::A),
        AisTransmitOptions::vdm(AisChannel::B),
    ];
    let lines: Vec<_> = options
        .into_iter()
        .enumerate()
        .map(|(i, option)| {
            SafetyBroadcast {
                repeat_indicator: 0,
                mmsi: 111_111_111 + i as u32,
                text: "A".repeat(100),
            }
            .to_sentences(option.with_sequence_id(0))
            .expect("encode")
        })
        .collect();
    let mut parser = AisParser::new();
    for message in &lines {
        assert_eq!(message.len(), 2);
        let frame = parse_frame(&message[0]).expect("first fragment");
        assert_eq!(parser.decode(&frame), Ok(AisDecodeOutcome::Pending));
    }
    // A malformed unrelated frame must not discard the three valid assemblies.
    let invalid = parse_frame("!AIVDM,2,2,0,Z,1,0").expect("frame");
    assert_eq!(
        parser.decode(&invalid),
        Err(AisDecodeError::InvalidFragmentField { field: "channel" })
    );
    for (i, message) in lines.iter().enumerate() {
        let frame = parse_frame(&message[1]).expect("last fragment");
        let decoded = parser.decode(&frame).expect("decode");
        assert!(
            matches!(&decoded, AisDecodeOutcome::Message(AisMessage::Safety(value))
            if value.mmsi == 111_111_111 + i as u32 && value.text == "A".repeat(100))
        );
    }
    for message in &lines {
        assert_eq!(
            parser.decode(&parse_frame(&message[0]).expect("first")),
            Ok(AisDecodeOutcome::Pending)
        );
    }
    parser.reset();
    for message in &lines {
        assert_eq!(
            parser.decode(&parse_frame(&message[1]).expect("last")),
            Err(AisDecodeError::UnexpectedFragment)
        );
    }
}

#[test]
fn canonical_api_preserves_diagnostics() {
    let ignored = parse_frame("$GPRMC,").expect("frame");
    let invalid = parse_frame("!AIVDM,1,1,,A,X,0").expect("frame");
    let mut parser = AisParser::new();
    assert_eq!(parser.decode(&ignored), Ok(AisDecodeOutcome::Ignored));
    assert_eq!(parser.decode(&invalid), Err(AisDecodeError::InvalidArmor));
    assert_eq!(
        nmea_0183_rs::ais::fragments::FragmentCollector::new()
            .process(&[])
            .err(),
        Some(AisDecodeError::MissingFragmentFields { actual: 0 })
    );
}
