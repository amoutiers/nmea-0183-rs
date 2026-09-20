#![cfg(feature = "mta")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Mta;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIMTA,17.2,C*01").expect("valid");
    let mta = Mta::parse(&frame.fields);
    let sentence = mta.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let mta2 = Mta::parse(&frame2.fields);
    assert_eq!(mta, mta2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIMTA,17.2,C*01").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Mta(_)));
}

#[test]
fn roundtrip() {
    let original = Mta {
        temperature: Some(17.2),
        units: Some('C'),
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Mta::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn mta_values() {
    // Fixture: SignalK signalk-parser-nmea0183 MTA example.
    let frame = parse_frame("$IIMTA,17.2,C*01").expect("valid MTA frame");
    let m = Mta::parse(&frame.fields);
    assert!((m.temperature.expect("temperature") - 17.2).abs() < 1e-4);
    assert_eq!(m.units, Some('C'));
}
