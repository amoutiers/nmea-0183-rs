#![cfg(feature = "hms")]

use nmea_0183_rs::nmea::{NmeaEncodable, NmeaSentence, sentences::Hms};
use nmea_0183_rs::parse_frame;

const FIXTURE: &str = "$--HMS,01,02,1.2*54";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid HMS");
    let original = Hms::parse(&frame.fields);
    let sentence = original.to_sentence("HE").expect("encode HMS");
    let frame = parse_frame(sentence.trim()).expect("re-parse HMS");
    assert_eq!(original, Hms::parse(&frame.fields));
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid HMS");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Hms(_)));
}

#[test]
fn roundtrip() {
    let original = Hms {
        sensor1: Some("01".to_string()),
        sensor2: Some("02".to_string()),
        max_difference_deg: Some(1.2),
    };
    let sentence = original.to_sentence("HE").expect("encode HMS");
    let frame = parse_frame(sentence.trim()).expect("re-parse HMS");
    assert_eq!(original, Hms::parse(&frame.fields));
}
