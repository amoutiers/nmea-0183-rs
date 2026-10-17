#![cfg(feature = "dsi")]

use nmea_0183_rs::nmea::{NmeaEncodable, NmeaSentence, sentences::Dsi};
use nmea_0183_rs::parse_frame;

const FIXTURE: &str = "$--DSI,1,1,3601234560,,,,00,12345678,E*3D";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid DSI");
    let original = Dsi::parse(&frame.fields);
    let sentence = original.to_sentence("GP").expect("encode DSI");
    let frame = parse_frame(sentence.trim()).expect("re-parse DSI");
    assert_eq!(original, Dsi::parse(&frame.fields));
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid DSI");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dsi(_)));
}

#[test]
fn roundtrip() {
    let original = Dsi {
        total_sentences: Some(1),
        sentence_number: Some(1),
        vessel_mmsi: Some("3601234560".to_string()),
        vessel_course: None,
        vessel_type: None,
        geographic_area: None,
        command_sets: vec![(Some("00".to_string()), Some("12345678".to_string()))],
        expansion_indicator: Some('E'),
    };
    let sentence = original.to_sentence("GP").expect("encode DSI");
    let frame = parse_frame(sentence.trim()).expect("re-parse DSI");
    assert_eq!(original, Dsi::parse(&frame.fields));
}
