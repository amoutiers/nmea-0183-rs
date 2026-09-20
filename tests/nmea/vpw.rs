#![cfg(feature = "vpw")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Vpw;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIVPW,4.5,N,6.7,M*52").expect("valid");
    let vpw = Vpw::parse(&frame.fields).expect("parse");
    let sentence = vpw.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vpw2 = Vpw::parse(&frame2.fields).expect("parse");
    assert_eq!(vpw, vpw2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIVPW,4.5,N,6.7,M*52").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vpw(_)));
}

#[test]
fn roundtrip() {
    let original = Vpw {
        speed_knots: Some(4.5),
        speed_ms: Some(6.7),
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vpw::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn vpw_values() {
    // Fixture: VPW with both knots and m/s present (gonmea VPW example).
    // (a) value half
    let frame = parse_frame("$IIVPW,4.5,N,6.7,M*52").expect("valid VPW frame");
    let x = Vpw::parse(&frame.fields).expect("parse VPW");
    assert!((x.speed_knots.expect("speed_knots") - 4.5).abs() < 1e-4);
    assert!((x.speed_ms.expect("speed_ms") - 6.7).abs() < 1e-4);

    // (b) wire half — fixed 'N'/'M' indicators always emitted
    let s = x.to_sentence("II").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "IIVPW,4.5,N,6.7,M");
}
