#![cfg(feature = "gbs")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Gbs;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPGBS,194907.00,3.0,1.9,4.2,,,,*4E").expect("valid");
    let gbs = Gbs::parse(&frame.fields);
    let sentence = gbs.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gbs2 = Gbs::parse(&frame2.fields);
    assert_eq!(gbs, gbs2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPGBS,194907.00,3.0,1.9,4.2,,,,*4E").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gbs(_)));
}

#[test]
fn roundtrip() {
    let original = Gbs {
        time: Some("194907.00".to_string()),
        err_lat: Some(3.0),
        err_lon: Some(1.9),
        err_alt: Some(4.2),
        svid: Some(12),
        prob: Some(0.5),
        bias: Some(1.1),
        stddev: Some(0.8),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gbs::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn gbs_values() {
    let frame = parse_frame("$GPGBS,194907.00,3.0,1.9,4.2,,,,*4E").expect("valid GBS fixture");
    let gbs = Gbs::parse(&frame.fields);
    assert_eq!(gbs.time.as_deref(), Some("194907.00"));
    assert!((gbs.err_lat.expect("err_lat") - 3.0_f32).abs() < 1e-4);
    assert!((gbs.err_lon.expect("err_lon") - 1.9_f32).abs() < 1e-4);
    assert!((gbs.err_alt.expect("err_alt") - 4.2_f32).abs() < 1e-4);
    assert!(gbs.svid.is_none());
    assert!(gbs.prob.is_none());
    assert!(gbs.bias.is_none());
    assert!(gbs.stddev.is_none());
}
