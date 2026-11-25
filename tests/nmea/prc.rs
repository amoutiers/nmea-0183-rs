#![cfg(feature = "prc")]

use nmea_0183_rs::nmea::Prc;
use nmea_0183_rs::{NmeaSentence, parse_frame};

const FIXTURE: &str = "$RCPRC,,V,0.0,R,,V,,0*1C";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid PRC fixture");
    let original = NmeaSentence::parse(&frame);
    let encoded = original.to_sentence("RC").expect("encode PRC");
    let frame = parse_frame(&encoded).expect("decode PRC");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid PRC fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Prc(_)));
}

#[test]
fn roundtrip() {
    let original = NmeaSentence::Prc(Prc {
        engine_or_shaft: Some("1".into()),
        ..Default::default()
    });
    let encoded = original.to_sentence("RC").expect("encode PRC");
    let frame = parse_frame(&encoded).expect("decode PRC");
    assert_eq!(NmeaSentence::parse(&frame), original);
}
