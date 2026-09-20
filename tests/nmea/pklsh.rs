#![cfg(feature = "pklsh")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Pklsh;
use nmea_0183_rs::{EncodeError, NmeaSentence, parse_frame};

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

#[test]
fn encodes_coordinates_with_nmea_padding_and_validation() {
    let mut value = Pklsh::parse(&[]).expect("parse empty");
    value.lat = Some(133.82);
    value.lon = Some(42.24);
    assert_eq!(value.encode().expect("encode")[0], "0133.82");
    assert_eq!(value.encode().expect("encode")[2], "00042.24");
    value.lat = Some(f64::INFINITY);
    assert_eq!(value.encode(), Err(EncodeError::InvalidCoordinate));
}
