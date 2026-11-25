#![cfg(feature = "trc")]

use nmea_0183_rs::nmea::Trc;
use nmea_0183_rs::{NmeaSentence, parse_frame};

const FIXTURE: &str = "$RCTRC,2,402.6,R,,V,88.3,B,C*50";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid TRC fixture");
    let original = NmeaSentence::parse(&frame);
    let encoded = original.to_sentence("RC").expect("encode TRC");
    assert_eq!(encoded.trim_end(), FIXTURE);
    let frame = parse_frame(&encoded).expect("decode TRC");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid TRC fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Trc(_)));
}

#[test]
fn roundtrip() {
    let original = NmeaSentence::Trc(Trc {
        sentence_status: Some('R'),
        ..Default::default()
    });
    let encoded = original.to_sentence("RC").expect("encode TRC");
    let frame = parse_frame(&encoded).expect("decode TRC");
    assert_eq!(NmeaSentence::parse(&frame), original);
}
