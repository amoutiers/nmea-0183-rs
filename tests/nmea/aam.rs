#![cfg(feature = "aam")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Aam;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPAAM,A,A,0.10,N,WPTNME*32").expect("valid");
    let aam = Aam::parse(&frame.fields);
    let sentence = aam.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let aam2 = Aam::parse(&frame2.fields);
    assert_eq!(aam, aam2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPAAM,A,A,0.10,N,WPTNME*32").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Aam(_)));
}

#[test]
fn roundtrip() {
    let original = Aam {
        arrce: Some('A'),
        perp: Some('A'),
        crad: Some(0.1),
        cunit: Some('N'),
        wpt: Some("DEST".to_string()),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Aam::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn aam_values() {
    let frame = parse_frame("$GPAAM,A,A,0.10,N,WPTNME*32").expect("valid");
    let a = Aam::parse(&frame.fields);
    assert_eq!(a.arrce, Some('A'));
    assert_eq!(a.perp, Some('A'));
    assert!((a.crad.expect("crad") - 0.10).abs() < 1e-4);
    assert_eq!(a.cunit, Some('N'));
    assert_eq!(a.wpt.as_deref(), Some("WPTNME"));
}
