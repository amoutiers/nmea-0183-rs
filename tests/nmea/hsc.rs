#![cfg(feature = "hsc")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Hsc;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$FTHSC,40.12,T,39.11,M*5E").expect("valid");
    let hsc = Hsc::parse(&frame.fields);
    let sentence = hsc.to_sentence("FT").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let hsc2 = Hsc::parse(&frame2.fields);
    assert_eq!(hsc, hsc2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$FTHSC,40.12,T,39.11,M*5E").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Hsc(_)));
}

#[test]
fn hsc_values() {
    // Fixture: SignalK signalk-parser-nmea0183 HSC sample (no trailing status).
    let frame = parse_frame("$FTHSC,40.12,T,39.11,M*5E").expect("valid");
    let hsc = Hsc::parse(&frame.fields);
    assert!((hsc.cmd_heading_true.expect("true") - 40.12).abs() < 1e-4);
    assert!((hsc.cmd_heading_mag.expect("mag") - 39.11).abs() < 1e-4);
    assert_eq!(hsc.status, None);

    // Fixed-indicator type (T, M): pin the exact canonical body.
    let sentence = hsc.to_sentence("FT").expect("encode");
    let body = sentence
        .trim()
        .trim_start_matches('$')
        .split('*')
        .next()
        .expect("body");
    assert_eq!(body, "FTHSC,40.12,T,39.11,M");
}

#[test]
fn roundtrip() {
    let original = Hsc {
        cmd_heading_true: Some(40.12),
        cmd_heading_mag: Some(39.11),
        status: Some('A'),
    };
    let sentence = original.to_sentence("FT").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Hsc::parse(&frame.fields);
    assert_eq!(original, parsed);
}
