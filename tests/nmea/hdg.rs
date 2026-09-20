#![cfg(feature = "hdg")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Hdg;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$SDHDG,181.9,,,0.6,E*32").expect("valid");
    let hdg = Hdg::parse(&frame.fields);
    let sentence = hdg.to_sentence("SD").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let hdg2 = Hdg::parse(&frame2.fields);
    assert_eq!(hdg, hdg2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$SDHDG,181.9,,,0.6,E*32").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Hdg(_)));
}

#[test]
fn hdg_values() {
    let frame = parse_frame("$SDHDG,181.9,,,0.6,E*32").expect("valid");
    let hdg = Hdg::parse(&frame.fields);
    assert!((hdg.heading_mag.expect("heading_mag") - 181.9).abs() < 1e-4);
    assert_eq!(hdg.deviation, None);
    assert_eq!(hdg.deviation_ew, None);
    assert!((hdg.variation.expect("variation") - 0.6).abs() < 1e-4);
    assert_eq!(hdg.variation_ew, Some('E'));
    // half (b): canonical body equals fixture body (byte-identical)
    let s = hdg.to_sentence("SD").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "SDHDG,181.9,,,0.6,E");
}

#[test]
fn roundtrip() {
    let original = Hdg {
        heading_mag: Some(181.9),
        deviation: Some(2.5),
        deviation_ew: Some('E'),
        variation: Some(0.6),
        variation_ew: Some('E'),
    };
    let sentence = original.to_sentence("SD").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Hdg::parse(&frame.fields);
    assert_eq!(original, parsed);
}
