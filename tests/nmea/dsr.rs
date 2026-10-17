#![cfg(feature = "dsr")]

use nmea_0183_rs::nmea::{NmeaEncodable, NmeaSentence, sentences::Dsr};
use nmea_0183_rs::parse_frame;

const FIXTURE: &str = "$--DSR,1,1,3601234560,00,12345678,E*0A";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid DSR");
    let original = Dsr::parse(&frame.fields);
    let sentence = original.to_sentence("CD").expect("encode DSR");
    let frame = parse_frame(sentence.trim()).expect("re-parse DSR");
    assert_eq!(original, Dsr::parse(&frame.fields));
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid DSR");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dsr(_)));
}

#[test]
fn roundtrip() {
    let original = Dsr {
        total_sentences: Some(1),
        sentence_number: Some(1),
        vessel_mmsi: Some("3601234560".to_string()),
        data_sets: vec![(Some("00".to_string()), Some("12345678".to_string()))],
        expansion_indicator: Some('E'),
    };
    let sentence = original.to_sentence("CD").expect("encode DSR");
    let frame = parse_frame(sentence.trim()).expect("re-parse DSR");
    assert_eq!(original, Dsr::parse(&frame.fields));
}
