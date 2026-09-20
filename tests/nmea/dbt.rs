#![cfg(feature = "dbt")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Dbt;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$SDDBT,7.7,f,2.3,M,1.3,F*05").expect("valid");
    let dbt = Dbt::parse(&frame.fields).expect("parse");
    let sentence = dbt.to_sentence("SD").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dbt2 = Dbt::parse(&frame2.fields).expect("parse");
    assert_eq!(dbt, dbt2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIDBT,035.53,f,010.83,M,005.85,F*23").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dbt(_)));
}

#[test]
fn dbt_values() {
    let frame = parse_frame("$SDDBT,7.7,f,2.3,M,1.3,F*05").expect("valid");
    let dbt = Dbt::parse(&frame.fields).expect("parse");
    assert!((dbt.depth_feet.expect("depth_feet") - 7.7).abs() < 1e-4);
    assert!((dbt.depth_meters.expect("depth_meters") - 2.3).abs() < 1e-4);
    assert!((dbt.depth_fathoms.expect("depth_fathoms") - 1.3).abs() < 1e-4);
    // half (b): canonical body equals fixture body (byte-identical)
    let s = dbt.to_sentence("SD").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "SDDBT,7.7,f,2.3,M,1.3,F");
}

#[test]
fn roundtrip() {
    let original = Dbt {
        depth_feet: Some(35.53),
        depth_meters: Some(10.83),
        depth_fathoms: Some(5.85),
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dbt::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
