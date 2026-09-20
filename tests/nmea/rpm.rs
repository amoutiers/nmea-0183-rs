#![cfg(feature = "rpm")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Rpm;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIRPM,E,1,2418.2,10.5,A*5F").expect("valid");
    let rpm = Rpm::parse(&frame.fields);
    let sentence = rpm.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rpm2 = Rpm::parse(&frame2.fields);
    assert_eq!(rpm, rpm2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIRPM,E,1,2418.2,10.5,A*5F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rpm(_)));
}

#[test]
fn roundtrip() {
    let original = Rpm {
        source: Some('E'),
        engine_shaft_num: Some(1),
        rpm: Some(2418.2),
        pitch: Some(10.5),
        status: Some('A'),
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rpm::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn rpm_values() {
    let frame = parse_frame("$IIRPM,E,1,2418.2,10.5,A*5F").expect("valid");
    let r = Rpm::parse(&frame.fields);
    assert_eq!(r.source, Some('E'));
    assert_eq!(r.engine_shaft_num, Some(1));
    assert!((r.rpm.expect("rpm") - 2418.2).abs() < 1e-2);
    assert!((r.pitch.expect("pitch") - 10.5).abs() < 1e-4);
    assert_eq!(r.status, Some('A'));
}
