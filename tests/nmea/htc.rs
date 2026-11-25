#![cfg(feature = "htc")]

use nmea_0183_rs::nmea::Htc;
use nmea_0183_rs::{NmeaSentence, parse_frame};

const FIXTURE: &str = "$--HTC,A,123.4,R,M,N,,,,,,,,T*1D";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid HTC fixture");
    let original = NmeaSentence::parse(&frame);
    let encoded = original.to_sentence("HE").expect("encode HTC");
    let (body, _) = encoded.split_once('*').expect("checksum");
    assert_eq!(body, "$HEHTC,A,123.4,R,M,N,,,,,,,,T");
    let frame = parse_frame(&encoded).expect("decode HTC");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid HTC fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Htc(_)));
}

#[test]
fn roundtrip() {
    let original = NmeaSentence::Htc(Htc {
        sentence_status: Some('C'),
        ..Default::default()
    });
    let encoded = original.to_sentence("HE").expect("encode HTC");
    let frame = parse_frame(&encoded).expect("decode HTC");
    assert_eq!(NmeaSentence::parse(&frame), original);
}
