#![cfg(feature = "vtg")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Vtg;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPVTG,0.0,T,359.3,M,0.0,N,0.0,K,A*2F").expect("valid");
    let vtg = Vtg::parse(&frame.fields).expect("parse");
    let sentence = vtg.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vtg2 = Vtg::parse(&frame2.fields).expect("parse");
    assert_eq!(vtg, vtg2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPVTG,0.0,T,359.3,M,0.0,N,0.0,K,A*2F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vtg(_)));
}

#[test]
fn vtg_values() {
    let frame = parse_frame("$GPVTG,0.0,T,359.3,M,0.0,N,0.0,K,A*2F").expect("valid");
    let vtg = Vtg::parse(&frame.fields).expect("parse");
    assert!((vtg.course_true.expect("course_true") - 0.0).abs() < 1e-4);
    assert!((vtg.course_mag.expect("course_mag") - 359.3).abs() < 1e-3);
    assert!((vtg.speed_kts.expect("speed_kts") - 0.0).abs() < 1e-4);
    assert!((vtg.speed_kmh.expect("speed_kmh") - 0.0).abs() < 1e-4);
    assert_eq!(vtg.mode, Some('A'));
    // half (b): canonical re-encode — "0.0" → "0" for all zero f32 values
    let s = vtg.to_sentence("GP").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "GPVTG,0,T,359.3,M,0,N,0,K,A");
}

#[test]
fn roundtrip() {
    let original = Vtg {
        course_true: Some(0.0),
        course_mag: Some(359.3),
        speed_kts: Some(5.0),
        speed_kmh: Some(9.26),
        mode: Some('A'),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vtg::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
