#![cfg(feature = "ths")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Ths;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPTHS,77.52,E*34").expect("valid");
    let ths = Ths::parse(&frame.fields).expect("parse");
    let sentence = ths.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let ths2 = Ths::parse(&frame2.fields).expect("parse");
    assert_eq!(ths, ths2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPTHS,77.52,E*34").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Ths(_)));
}

#[test]
fn roundtrip() {
    let original = Ths {
        heading_true: Some(77.52),
        mode: Some('E'),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Ths::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn ths_values() {
    let frame = parse_frame("$GPTHS,77.52,E*34").expect("valid THS frame");
    let x = Ths::parse(&frame.fields).expect("parse THS");
    assert!((x.heading_true.expect("heading_true") - 77.52).abs() < 1e-2);
    assert_eq!(x.mode, Some('E'));
}
