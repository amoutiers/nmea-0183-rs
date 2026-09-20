#![cfg(feature = "pknds")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Pknds;
use nmea_0183_rs::{EncodeError, NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame(
        "$PKNDS,220516,A,5133.82,N,00042.24,W,173.8,231.8,130694,004.2,W00,U00001,207,00,*28",
    )
    .expect("valid");
    let p = Pknds::parse(&frame.fields);
    let sentence = p.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let p2 = Pknds::parse(&frame2.fields);
    assert_eq!(p, p2);
}

#[test]
fn dispatch() {
    let frame = parse_frame(
        "$PKNDS,220516,A,5133.82,N,00042.24,W,173.8,231.8,130694,004.2,W00,U00001,207,00,*28",
    )
    .expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Pknds(_)
    ));
}

#[test]
fn roundtrip() {
    let original = Pknds {
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
        unit_id: Some("U00001".to_string()),
        status: Some("207".to_string()),
        extension: Some("00".to_string()),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pknds::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn encodes_coordinates_with_nmea_padding_and_validation() {
    let mut value = Pknds::parse(&[]);
    value.lat = Some(133.82);
    value.lon = Some(42.24);
    assert_eq!(value.encode().expect("encode")[2], "0133.82");
    assert_eq!(value.encode().expect("encode")[4], "00042.24");
    value.lat = Some(-0.0);
    value.lon = Some(0.0);
    assert_eq!(value.encode().expect("encode")[2], "0000.0");
    assert_eq!(value.encode().expect("encode")[4], "00000.0");
    for invalid in [-1.0, f64::NAN, f64::INFINITY] {
        value.lon = Some(invalid);
        assert_eq!(value.encode(), Err(EncodeError::InvalidCoordinate));
    }
}
