#![cfg(feature = "dbs")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Dbs;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIDBS,035.53,f,010.83,M,005.85,F*24").expect("valid");
    let dbs = Dbs::parse(&frame.fields).expect("parse");
    let sentence = dbs.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dbs2 = Dbs::parse(&frame2.fields).expect("parse");
    assert_eq!(dbs, dbs2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIDBS,035.53,f,010.83,M,005.85,F*24").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dbs(_)));
}

#[test]
fn roundtrip() {
    let original = Dbs {
        depth_feet: Some(35.53),
        depth_meters: Some(10.83),
        depth_fathoms: Some(5.85),
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dbs::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn dbs_values() {
    let frame = parse_frame("$IIDBS,035.53,f,010.83,M,005.85,F*24").expect("valid");
    let x = Dbs::parse(&frame.fields).expect("parse");
    // (a) value half
    assert!((x.depth_feet.expect("depth_feet") - 35.53).abs() < 1e-2);
    assert!((x.depth_meters.expect("depth_meters") - 10.83).abs() < 1e-2);
    assert!((x.depth_fathoms.expect("depth_fathoms") - 5.85).abs() < 1e-2);
    // (b) wire half
    let s = x.to_sentence("II").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "IIDBS,35.53,f,10.83,M,5.85,F");
}
