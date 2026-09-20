#![cfg(feature = "pknid")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Pknid;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$PKNID,00,U00001,015,00,*24").expect("valid");
    let p = Pknid::parse(&frame.fields).expect("parse");
    let sentence = p.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let p2 = Pknid::parse(&frame2.fields).expect("parse");
    assert_eq!(p, p2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$PKNID,00,U00001,015,00,*24").expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Pknid(_)
    ));
}

#[test]
fn roundtrip() {
    let original = Pknid {
        version: Some("00".to_string()),
        unit_id: Some("U00001".to_string()),
        status: Some("015".to_string()),
        extension: Some("00".to_string()),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pknid::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
