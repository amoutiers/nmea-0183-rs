#![cfg(feature = "rot")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Rot;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$HEROT,0.0,A*2B").expect("valid");
    let rot = Rot::parse(&frame.fields).expect("parse");
    let sentence = rot.to_sentence("HE").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rot2 = Rot::parse(&frame2.fields).expect("parse");
    assert_eq!(rot, rot2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$HEROT,0.0,A*2B").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rot(_)));
}

#[test]
fn roundtrip() {
    let original = Rot {
        rate_of_turn: Some(35.6),
        valid: Some('A'),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rot::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn rot_values() {
    let frame = parse_frame("$HEROT,0.0,A*2B").expect("valid");
    let r = Rot::parse(&frame.fields).expect("parse");
    assert!((r.rate_of_turn.expect("rate_of_turn") - 0.0).abs() < 1e-4);
    assert_eq!(r.valid, Some('A'));
}
