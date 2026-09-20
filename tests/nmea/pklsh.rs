#![cfg(feature = "pklsh")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Pklsh;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$PKLSH,3926.7952,N,12000.5947,W,022732,A,100,2000*1A").expect("valid");
    let p = Pklsh::parse(&frame.fields).expect("parse");
    let sentence = p.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let p2 = Pklsh::parse(&frame2.fields).expect("parse");
    assert_eq!(p, p2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$PKLSH,3926.7952,N,12000.5947,W,022732,A,100,2000*1A").expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Pklsh(_)
    ));
}

#[test]
fn roundtrip() {
    let original = Pklsh {
        lat: Some(3926.7952),
        ns: Some('N'),
        lon: Some(12000.5947),
        ew: Some('W'),
        time: Some("022732".to_string()),
        validity: Some('A'),
        fleet: Some("100".to_string()),
        unit_id: Some("2000".to_string()),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pklsh::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
