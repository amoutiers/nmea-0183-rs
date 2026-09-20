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
        parser.decode(&frame).is_none(),
        "parser should ignore $ NMEA frames"
    );
}

#[test]
fn type8_now_decoded() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,85Mv070j2d>=<e<<=PQhhg`59P00,0*26").expect("valid Type 8 frame");
    match parser.decode(&frame) {
        Some(AisMessage::BinaryBroadcast(bb)) => assert!(bb.mmsi > 0),
        other => panic!("expected BinaryBroadcast (type 8), got {other:?}"),
    }
}

#[test]
fn truncated_payloads_return_none_no_panic() {
    let mut parser = AisParser::new();
    // Each payload is far shorter than the message type's minimum bit length,
    // so the decoder's `bits.len() < N` guard must return None (never panic).
    for frame_str in [
        "!AIVDM,1,1,,A,1,0*17", // type 1 (needs >= 144 bits) - 6 bits
        "!AIVDM,1,1,,A,5,0*13", // type 5 (needs >= 424) - 6 bits
        "!AIVDM,1,1,,A,H,0*6E", // type 24 (needs >= 160) - 6 bits
    ] {
        let frame = parse_frame(frame_str).expect("frame parses");
        // Must not panic; truncated content yields None or Unknown, never a wrong decode.
        let _ = parser.decode(&frame);
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
    assert!(
        parser
            .decode(&parse_frame(&vdm[0]).expect("parse VDM fragment one"))
            .is_none()
    );
    assert!(
        parser
            .decode(&parse_frame(&vdo[0]).expect("parse VDO fragment one"))
            .is_none()
    );
    assert!(matches!(
        parser.decode(&parse_frame(&vdm[1]).expect("parse VDM fragment two")),
        Some(AisMessage::Safety(message))
            if message.mmsi == 111_111_111 && message.text == "A".repeat(100)
    ));
    assert!(matches!(
        parser.decode(&parse_frame(&vdo[1]).expect("parse VDO fragment two")),
        Some(AisMessage::Safety(message))
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
    assert!(
        parser
            .decode(&parse_frame(&vdm[0]).expect("parse VDM"))
            .is_none()
    );
    assert!(
        parser
            .decode(&parse_frame(&vdo[0]).expect("parse VDO"))
            .is_none()
    );
    parser.reset();
    assert!(
        parser
            .decode(&parse_frame(&vdm[1]).expect("parse VDM"))
            .is_none()
    );
    assert!(
        parser
            .decode(&parse_frame(&vdo[1]).expect("parse VDO"))
            .is_none()
    );
}

#[test]
fn detailed_decode_distinguishes_outcomes() {
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
            p.decode_detailed(&parse_frame(line).expect("frame")),
            expected,
            "{line}"
        );
    }
    let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26")
        .expect("existing Type 1 fixture");
    assert!(matches!(
        p.decode_detailed(&frame),
        Ok(Message(AisMessage::Position(_)))
    ));
}

#[test]
fn detailed_decode_preserves_stream_isolation_and_reset() {
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
    let mut detailed = AisParser::new();
    let mut legacy = AisParser::new();
    for message in &lines {
        assert_eq!(message.len(), 2);
        let frame = parse_frame(&message[0]).expect("first fragment");
        assert_eq!(
            detailed.decode_detailed(&frame),
            Ok(AisDecodeOutcome::Pending)
        );
        assert_eq!(legacy.decode(&frame), None);
    }
    // A malformed unrelated frame must not discard the three valid assemblies.
    let invalid = parse_frame("!AIVDM,2,2,0,Z,1,0").expect("frame");
    assert_eq!(
        detailed.decode_detailed(&invalid),
        Err(AisDecodeError::InvalidFragmentField { field: "channel" })
    );
    assert_eq!(legacy.decode(&invalid), None);
    for (i, message) in lines.iter().enumerate() {
        let frame = parse_frame(&message[1]).expect("last fragment");
        let decoded = detailed.decode_detailed(&frame).expect("decode");
        assert!(
            matches!(&decoded, AisDecodeOutcome::Message(AisMessage::Safety(value))
            if value.mmsi == 111_111_111 + i as u32 && value.text == "A".repeat(100))
        );
        assert_eq!(
            decoded,
            AisDecodeOutcome::Message(legacy.decode(&frame).expect("legacy decode"))
        );
    }
    for message in &lines {
        assert_eq!(
            detailed.decode_detailed(&parse_frame(&message[0]).expect("first")),
            Ok(AisDecodeOutcome::Pending)
        );
    }
    detailed.reset();
    for message in &lines {
        assert_eq!(
            detailed.decode_detailed(&parse_frame(&message[1]).expect("last")),
            Err(AisDecodeError::UnexpectedFragment)
        );
    }
}
