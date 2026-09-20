#![cfg(feature = "vdr")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Vdr;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIVDR,10.1,T,12.3,M,1.2,N*3A").expect("valid");
    let vdr = Vdr::parse(&frame.fields);
    let sentence = vdr.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vdr2 = Vdr::parse(&frame2.fields);
    assert_eq!(vdr, vdr2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIVDR,10.1,T,12.3,M,1.2,N*3A").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vdr(_)));
}

#[test]
fn roundtrip() {
    let original = Vdr {
        direction_true: Some(10.1),
        direction_mag: Some(12.3),
        speed_knots: Some(1.2),
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vdr::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn vdr_values() {
    // (a) value half
    let frame = parse_frame("$IIVDR,10.1,T,12.3,M,1.2,N*3A").expect("valid VDR frame");
    let x = Vdr::parse(&frame.fields);
    assert!((x.direction_true.expect("direction_true") - 10.1).abs() < 1e-2);
    assert!((x.direction_mag.expect("direction_mag") - 12.3).abs() < 1e-2);
    assert!((x.speed_knots.expect("speed_knots") - 1.2).abs() < 1e-2);

    // (b) wire half
    let s = x.to_sentence("II").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "IIVDR,10.1,T,12.3,M,1.2,N");
}
