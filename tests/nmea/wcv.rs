#![cfg(feature = "wcv")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Wcv;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let wcv = Wcv {
        vel: Some(5.3),
        wpt: Some("DEST".to_string()),
        mode: Some('A'),
    };
    let sentence = wcv.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("valid");
    let wcv2 = Wcv::parse(&frame.fields).expect("parse");
    let sentence2 = wcv2.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence2.trim()).expect("re-parse");
    let wcv3 = Wcv::parse(&frame2.fields).expect("parse");
    assert_eq!(wcv2, wcv3);
}

#[test]
fn dispatch() {
    let wcv = Wcv {
        vel: Some(5.3),
        wpt: Some("DEST".to_string()),
        mode: Some('A'),
    };
    let sentence = wcv.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Wcv(_)));
}

#[test]
fn roundtrip() {
    let original = Wcv {
        vel: Some(5.3),
        wpt: Some("DEST".to_string()),
        mode: Some('A'),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Wcv::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn wcv_values() {
    // (a) value half — authored fixture $IIWCV,5.3,N,DEST,A*63
    let frame = parse_frame("$IIWCV,5.3,N,DEST,A*63").expect("valid WCV frame");
    let x = Wcv::parse(&frame.fields).expect("parse WCV");
    assert!((x.vel.expect("vel") - 5.3).abs() < 1e-2);
    assert_eq!(x.wpt.as_deref(), Some("DEST"));
    assert_eq!(x.mode, Some('A'));

    // (b) wire half
    let s = x.to_sentence("II").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "IIWCV,5.3,N,DEST,A");
}
