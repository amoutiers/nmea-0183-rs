#![cfg(feature = "dor")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Dor;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$FRDOR,E,233042,FD,FP,000,010,C,C,Door Closed : TEST FPA Name*4D")
        .expect("valid");
    let dor = Dor::parse(&frame.fields);
    let sentence = dor.to_sentence("FR").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dor2 = Dor::parse(&frame2.fields);
    assert_eq!(dor, dor2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$FRDOR,E,233042,FD,FP,000,010,C,C,Door Closed : TEST FPA Name*4D")
        .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dor(_)));
}

#[test]
fn dor_values() {
    let frame = parse_frame("$FRDOR,E,233042,FD,FP,000,010,C,C,Door Closed : TEST FPA Name*4D")
        .expect("valid");
    let dor = Dor::parse(&frame.fields);
    assert_eq!(dor.door_type, Some('E'));
    assert_eq!(dor.time.as_deref(), Some("233042"));
    assert_eq!(dor.system.as_deref(), Some("FD"));
    assert_eq!(dor.division1.as_deref(), Some("FP"));
    assert_eq!(dor.division2.as_deref(), Some("000"));
    assert_eq!(dor.door_number.as_deref(), Some("010"));
    assert_eq!(dor.door_status, Some('C'));
    assert_eq!(dor.switch_setting, Some('C'));
    assert_eq!(dor.message.as_deref(), Some("Door Closed : TEST FPA Name"));
}

#[test]
fn roundtrip() {
    let original = Dor {
        door_type: Some('E'),
        time: Some("233042".to_string()),
        system: Some("FD".to_string()),
        division1: Some("FP".to_string()),
        division2: Some("000".to_string()),
        door_number: Some("010".to_string()),
        door_status: Some('C'),
        switch_setting: Some('C'),
        message: Some("Door Closed".to_string()),
    };
    let sentence = original.to_sentence("FR").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dor::parse(&frame.fields);
    assert_eq!(original, parsed);
}
