#![cfg(feature = "vwr")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Vwr;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIVWR,75,R,1.0,N,0.51,M,1.85,K*6C").expect("valid");
    let vwr = Vwr::parse(&frame.fields).expect("parse");
    let sentence = vwr.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vwr2 = Vwr::parse(&frame2.fields).expect("parse");
    assert_eq!(vwr, vwr2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIVWR,75,R,1.0,N,0.51,M,1.85,K*6C").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vwr(_)));
}

#[test]
fn roundtrip() {
    let original = Vwr {
        angle: Some(75.0),
        angle_lr: Some('R'),
        speed_knots: Some(1.0),
        speed_ms: Some(0.51),
        speed_kmh: Some(1.85),
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vwr::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn vwr_values() {
    // (a) value half
    let frame = parse_frame("$IIVWR,75,R,1.0,N,0.51,M,1.85,K*6C").expect("valid VWR frame");
    let x = Vwr::parse(&frame.fields).expect("parse VWR");
    assert!((x.angle.expect("angle") - 75.0).abs() < 1e-2);
    assert_eq!(x.angle_lr, Some('R'));
    assert!((x.speed_knots.expect("speed_knots") - 1.0).abs() < 1e-2);
    assert!((x.speed_ms.expect("speed_ms") - 0.51).abs() < 1e-2);
    assert!((x.speed_kmh.expect("speed_kmh") - 1.85).abs() < 1e-2);

    // (b) wire half — 75.0→"75", 1.0→"1"
    let s = x.to_sentence("II").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "IIVWR,75,R,1,N,0.51,M,1.85,K");
}
