#![cfg(feature = "dbk")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Dbk;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$SDDBK,12.3,f,3.7,M,2.0,F*2F").expect("valid");
    let dbk = Dbk::parse(&frame.fields);
    let sentence = dbk.to_sentence("SD").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dbk2 = Dbk::parse(&frame2.fields);
    assert_eq!(dbk, dbk2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIDBK,035.53,f,010.83,M,005.85,F*3C").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dbk(_)));
}

#[test]
fn roundtrip() {
    let original = Dbk {
        depth_feet: Some(35.53),
        depth_meters: Some(10.83),
        depth_fathoms: Some(5.85),
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dbk::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn dbk_values() {
    let frame = parse_frame("$SDDBK,12.3,f,3.7,M,2.0,F*2F").expect("valid");
    let x = Dbk::parse(&frame.fields);
    // (a) value half
    assert!((x.depth_feet.expect("depth_feet") - 12.3).abs() < 1e-2);
    assert!((x.depth_meters.expect("depth_meters") - 3.7).abs() < 1e-2);
    assert!((x.depth_fathoms.expect("depth_fathoms") - 2.0).abs() < 1e-2);
    // (b) wire half
    let s = x.to_sentence("SD").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "SDDBK,12.3,f,3.7,M,2,F");
}
