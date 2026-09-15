#![cfg(feature = "pklid")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Pklid;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$PKLID,00,100,2000,15,00,*6D").expect("valid");
    let p = Pklid::parse(&frame.fields).expect("parse");
    let sentence = p.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let p2 = Pklid::parse(&frame2.fields).expect("parse");
    assert_eq!(p, p2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$PKLID,00,100,2000,15,00,*6D").expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Pklid(_)
    ));
}

#[test]
fn roundtrip() {
    let original = Pklid {
        version: Some("00".to_string()),
        fleet: Some("100".to_string()),
        unit_id: Some("2000".to_string()),
        status: Some("15".to_string()),
        extension: Some("00".to_string()),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pklid::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
