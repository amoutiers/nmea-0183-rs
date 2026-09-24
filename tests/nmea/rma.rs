#![cfg(feature = "rma")]

use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Rma;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$GPRMA,A,5327.03942,N,11214.42462,W,,,23.1,23,14.8,W*58").expect("valid");
    let rma = Rma::parse(&frame.fields);
    let sentence = rma.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    assert_eq!(Rma::parse(&frame.fields), rma);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$GPRMA,A,5327.03942,N,11214.42462,W,,,23.1,23,14.8,W*58").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rma(_)));
}

#[test]
fn rma_values() {
    let frame = parse_frame("$GPRMA,A,5327.03942,N,11214.42462,W,,,23.1,23,14.8,W*58")
        .expect("valid pynmeagps RMA fixture");
    let rma = Rma::parse(&frame.fields);
    assert_eq!(rma.status, Some('A'));
    assert!((rma.lat.expect("lat") - 5327.03942).abs() < 1e-6);
    assert_eq!(rma.ns, Some('N'));
    assert!((rma.lon.expect("lon") - 11214.42462).abs() < 1e-6);
    assert_eq!(rma.ew, Some('W'));
    assert!(rma.td_a.is_none());
    assert!(rma.td_b.is_none());
    assert!((rma.sog.expect("sog") - 23.1).abs() < 1e-4);
    assert!((rma.cog.expect("cog") - 23.0).abs() < 1e-4);
    assert!((rma.mag_var.expect("mag_var") - 14.8).abs() < 1e-4);
    assert_eq!(rma.mag_var_ew, Some('W'));
}

#[test]
fn roundtrip() {
    let original = Rma {
        status: Some('A'),
        lat: Some(5327.03942),
        ns: Some('N'),
        lon: Some(11214.42462),
        ew: Some('W'),
        td_a: Some(12345.6),
        td_b: Some(23456.7),
        sog: Some(23.1),
        cog: Some(23.0),
        mag_var: Some(14.8),
        mag_var_ew: Some('W'),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    assert_eq!(Rma::parse(&frame.fields), original);
}
