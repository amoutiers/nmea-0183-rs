#![cfg(feature = "pklds")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Pklds;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame(
        "$PKLDS,220516,A,5133.82,N,00042.24,W,173.8,231.8,130694,004.2,W00,100,2000,15,00,*60",
    )
    .expect("valid");
    let p = Pklds::parse(&frame.fields).expect("parse");
    let sentence = p.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let p2 = Pklds::parse(&frame2.fields).expect("parse");
    assert_eq!(p, p2);
}

#[test]
fn dispatch() {
    let frame = parse_frame(
        "$PKLDS,220516,A,5133.82,N,00042.24,W,173.8,231.8,130694,004.2,W00,100,2000,15,00,*60",
    )
    .expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Pklds(_)
    ));
}

#[test]
fn roundtrip() {
    let original = Pklds {
        time: Some("220516".to_string()),
        validity: Some('A'),
        lat: Some(5133.82),
        ns: Some('N'),
        lon: Some(42.24),
        ew: Some('W'),
        speed: Some(173.8),
        course: Some(231.8),
        date: Some("130694".to_string()),
        variation: Some(4.2),
        var_ew: Some("W00".to_string()),
        fleet: Some("100".to_string()),
        unit_id: Some("2000".to_string()),
        status: Some("15".to_string()),
        extension: Some("00".to_string()),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pklds::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
