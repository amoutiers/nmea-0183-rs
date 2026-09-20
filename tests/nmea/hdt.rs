#![cfg(feature = "hdt")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Hdt;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$HEHDT,4.0,T*2B").expect("valid");
    let hdt = Hdt::parse(&frame.fields);
    let sentence = hdt.to_sentence("HE").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let hdt2 = Hdt::parse(&frame2.fields);
    assert_eq!(hdt, hdt2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$HEHDT,4.0,T*2B").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Hdt(_)));
}

#[test]
fn roundtrip() {
    let original = Hdt {
        heading_true: Some(123.456),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Hdt::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn hdt_values() {
    // (a) value half
    let frame = parse_frame("$HEHDT,4.0,T*2B").expect("valid");
    let x = Hdt::parse(&frame.fields);
    assert!((x.heading_true.expect("heading_true") - 4.0).abs() < 1e-2);

    // (b) wire half — 4.0 normalises to "4" via format!("{}", v)
    let s = x.to_sentence("HE").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "HEHDT,4,T");
}
