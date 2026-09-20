#![cfg(feature = "gll")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Gll;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPGLL,5958.613,N,02325.928,E,121022,A,D*40").expect("valid");
    let gll = Gll::parse(&frame.fields);
    let sentence = gll.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gll2 = Gll::parse(&frame2.fields);
    assert_eq!(gll, gll2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPGLL,5958.613,N,02325.928,E,121022,A,D*40").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gll(_)));
}

#[test]
fn gll_values() {
    let frame = parse_frame("$GPGLL,5958.613,N,02325.928,E,121022,A,D*40").expect("valid");
    let gll = Gll::parse(&frame.fields);
    assert!((gll.lat.expect("lat") - 5958.613).abs() < 1e-9);
    assert_eq!(gll.ns, Some('N'));
    assert!((gll.lon.expect("lon") - 2325.928).abs() < 1e-9);
    assert_eq!(gll.ew, Some('E'));
    assert_eq!(gll.time, Some("121022".to_string()));
    assert_eq!(gll.status, Some('A'));
    assert_eq!(gll.mode, Some('D'));
    // half (b): canonical re-encode — mode always emitted, producing trailing comma if mode=Some
    let s = gll.to_sentence("GP").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "GPGLL,5958.613,N,02325.928,E,121022,A,D");
}

#[test]
fn roundtrip() {
    let original = Gll {
        lat: Some(4807.038),
        ns: Some('N'),
        lon: Some(1131.0),
        ew: Some('E'),
        time: Some("120000".to_string()),
        status: Some('A'),
        mode: Some('A'),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gll::parse(&frame.fields);
    assert_eq!(original, parsed);
}
