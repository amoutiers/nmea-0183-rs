#![cfg(feature = "htd")]

use nmea_0183_rs::nmea::Htd;
use nmea_0183_rs::{NmeaSentence, parse_frame};

const FIXTURE: &str = "$HEHTD,A,0.0,L,M,T,45.0,25.0,0.5,3275.0,200.0,1.0,250.0,T,A,A,A,270.0*67";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid HTD fixture");
    let original = NmeaSentence::parse(&frame);
    let encoded = original.to_sentence("HE").expect("encode HTD");
    let frame = parse_frame(&encoded).expect("decode HTD");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid HTD fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Htd(_)));
}

#[test]
fn roundtrip() {
    let original = NmeaSentence::Htd(Htd {
        vessel_heading: Some(130.0),
        ..Default::default()
    });
    let encoded = original.to_sentence("HE").expect("encode HTD");
    let frame = parse_frame(&encoded).expect("decode HTD");
    assert_eq!(NmeaSentence::parse(&frame), original);
}
