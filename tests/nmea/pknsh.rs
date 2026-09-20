#![cfg(feature = "pknsh")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Pknsh;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$PKNSH,3926.7952,N,12000.5947,W,022732,A,U00001*63").expect("valid");
    let p = Pknsh::parse(&frame.fields).expect("parse");
    let sentence = p.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let p2 = Pknsh::parse(&frame2.fields).expect("parse");
    assert_eq!(p, p2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$PKNSH,3926.7952,N,12000.5947,W,022732,A,U00001*63").expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Pknsh(_)
    ));
}

#[test]
fn roundtrip() {
    let original = Pknsh {
        lat: Some(3926.7952),
        ns: Some('N'),
        lon: Some(12000.5947),
        ew: Some('W'),
        time: Some("022732".to_string()),
        validity: Some('A'),
        unit_id: Some("U00001".to_string()),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pknsh::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
