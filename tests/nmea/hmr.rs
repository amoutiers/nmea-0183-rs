#![cfg(feature = "hmr")]

use nmea_0183_rs::nmea::{NmeaEncodable, NmeaSentence, sentences::Hmr};
use nmea_0183_rs::parse_frame;

const FIXTURE: &str = "$--HMR,01,02,1.2,0.1,A,123.4,A,M,0.1,E,123.5,A,M,0.1,E,1.2,E*52";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid HMR");
    let original = Hmr::parse(&frame.fields);
    let sentence = original.to_sentence("HC").expect("encode HMR");
    let frame = parse_frame(sentence.trim()).expect("re-parse HMR");
    assert_eq!(original, Hmr::parse(&frame.fields));
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid HMR");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Hmr(_)));
}

#[test]
fn roundtrip() {
    let original = Hmr {
        sensor_1_id: Some("01".to_string()),
        sensor_2_id: Some("02".to_string()),
        difference_limit: Some(1.2),
        sensor_difference: Some(0.1),
        warning_flag: Some('A'),
        sensor_1_heading: Some(123.4),
        sensor_1_status: Some('A'),
        sensor_1_type: Some('M'),
        sensor_1_deviation: Some(0.1),
        sensor_1_deviation_ew: Some('E'),
        sensor_2_heading: Some(123.5),
        sensor_2_status: Some('A'),
        sensor_2_type: Some('M'),
        sensor_2_deviation: Some(0.1),
        sensor_2_deviation_ew: Some('E'),
        variation: Some(1.2),
        variation_ew: Some('E'),
    };
    let sentence = original.to_sentence("HC").expect("encode HMR");
    let frame = parse_frame(sentence.trim()).expect("re-parse HMR");
    assert_eq!(original, Hmr::parse(&frame.fields));
}
