#![cfg(feature = "mwv")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Mwv;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIMWV,336,R,13.41,N,A*22").expect("valid");
    let mwv = Mwv::parse(&frame.fields).expect("parse");
    let sentence = mwv.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let mwv2 = Mwv::parse(&frame2.fields).expect("parse");
    assert_eq!(mwv, mwv2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIMWV,336,R,13.41,N,A*22").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Mwv(_)));
}

#[test]
fn mwv_values() {
    let frame = parse_frame("$IIMWV,336,R,13.41,N,A*22").expect("valid");
    let mwv = Mwv::parse(&frame.fields).expect("parse");
    assert!((mwv.wind_angle.expect("wind_angle") - 336.0).abs() < 1e-3);
    assert_eq!(mwv.reference, Some('R'));
    assert!((mwv.wind_speed.expect("wind_speed") - 13.41).abs() < 1e-4);
    assert_eq!(mwv.speed_units, Some('N'));
    assert_eq!(mwv.status, Some('A'));
    // half (b): canonical body equals fixture body (byte-identical)
    let s = mwv.to_sentence("II").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "IIMWV,336,R,13.41,N,A");
}

#[test]
fn roundtrip() {
    let original = Mwv {
        wind_angle: Some(336.0),
        reference: Some('R'),
        wind_speed: Some(13.41),
        speed_units: Some('N'),
        status: Some('A'),
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Mwv::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
