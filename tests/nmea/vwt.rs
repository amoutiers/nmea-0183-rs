#![cfg(feature = "vwt")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Vwt;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIVWT,030.,R,10.1,N,05.2,M,018.7,K*75").expect("valid");
    let vwt = Vwt::parse(&frame.fields);
    let sentence = vwt.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vwt2 = Vwt::parse(&frame2.fields);
    assert_eq!(vwt, vwt2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIVWT,030.,R,10.1,N,05.2,M,018.7,K*75").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vwt(_)));
}

#[test]
fn roundtrip() {
    let original = Vwt {
        angle: Some(30.0),
        angle_lr: Some('R'),
        speed_knots: Some(10.1),
        speed_ms: Some(5.2),
        speed_kmh: Some(18.7),
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vwt::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn vwt_values() {
    // (a) value half — fixture has 030., 05.2, 018.7 which parse to 30.0, 5.2, 18.7
    let frame = parse_frame("$IIVWT,030.,R,10.1,N,05.2,M,018.7,K*75").expect("valid VWT frame");
    let x = Vwt::parse(&frame.fields);
    assert!((x.angle.expect("angle") - 30.0).abs() < 1e-2);
    assert_eq!(x.angle_lr, Some('R'));
    assert!((x.speed_knots.expect("speed_knots") - 10.1).abs() < 1e-2);
    assert!((x.speed_ms.expect("speed_ms") - 5.2).abs() < 1e-2);
    assert!((x.speed_kmh.expect("speed_kmh") - 18.7).abs() < 1e-2);

    // (b) wire half — 030.→"30", 05.2→"5.2", 018.7→"18.7"
    let s = x.to_sentence("II").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "IIVWT,30,R,10.1,N,5.2,M,18.7,K");
}
