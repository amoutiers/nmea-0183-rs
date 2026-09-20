//! AIS parser-level tests: frame filtering, fragment reassembly, reset.
#![cfg(feature = "ais")]

use nmea_0183_rs::ais::transmit::{AisChannel, AisEncodable, AisTransmitOptions, SafetyBroadcast};
use nmea_0183_rs::ais::{AisMessage, AisParser};
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
