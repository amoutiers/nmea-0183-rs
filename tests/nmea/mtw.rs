#![cfg(feature = "mtw")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Mtw;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$YXMTW,15.2,C*14").expect("valid");
    let mtw = Mtw::parse(&frame.fields).expect("parse");
    let sentence = mtw.to_sentence("YX").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let mtw2 = Mtw::parse(&frame2.fields).expect("parse");
    assert_eq!(mtw, mtw2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$YXMTW,15.2,C*14").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Mtw(_)));
}

#[test]
fn roundtrip() {
    let original = Mtw {
        temperature: Some(15.2),
        units: Some('C'),
    };
    let sentence = original.to_sentence("YX").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Mtw::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn mtw_values() {
    let frame = parse_frame("$YXMTW,15.2,C*14").expect("valid");
    let m = Mtw::parse(&frame.fields).expect("parse");
    assert!((m.temperature.expect("temperature") - 15.2).abs() < 1e-4);
    assert_eq!(m.units, Some('C'));
}
