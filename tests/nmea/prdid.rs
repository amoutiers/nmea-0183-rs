#![cfg(feature = "prdid")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Prdid;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$PRDID,-10.37,2.34,230.34*62").expect("valid");
    let prdid = Prdid::parse(&frame.fields).expect("parse");
    let sentence = prdid.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let prdid2 = Prdid::parse(&frame2.fields).expect("parse");
    assert_eq!(prdid, prdid2);
}

#[test]
fn roundtrip() {
    let original = Prdid {
        pitch: Some(-10.37),
        roll: Some(2.34),
        heading: Some(230.34),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Prdid::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$PRDID,-10.37,2.34,230.34*62").expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Prdid(_)
    ));
}

#[test]
fn prdid_values() {
    let frame = parse_frame("$PRDID,-10.37,2.34,230.34*62").expect("valid");
    let p = Prdid::parse(&frame.fields).expect("parse");
    assert!((p.pitch.expect("pitch") + 10.37).abs() < 0.01);
    assert!((p.roll.expect("roll") - 2.34).abs() < 0.01);
    assert!((p.heading.expect("heading") - 230.34).abs() < 0.01);
}
