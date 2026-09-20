#![cfg(feature = "bod")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Bod;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPBOD,097.0,T,103.2,M,POINTB,POINTA*4A").expect("valid");
    let bod = Bod::parse(&frame.fields);
    let sentence = bod.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let bod2 = Bod::parse(&frame2.fields);
    assert_eq!(bod, bod2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPBOD,097.0,T,103.2,M,POINTB,POINTA*4A").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Bod(_)));
}

#[test]
fn roundtrip() {
    let original = Bod {
        bear_true: Some(97.0),
        bear_true_type: Some('T'),
        bear_mag: Some(103.2),
        bear_mag_type: Some('M'),
        wpt_dest: Some("POINTB".to_string()),
        wpt_origin: Some("POINTA".to_string()),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Bod::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn bod_values() {
    let frame = parse_frame("$GPBOD,097.0,T,103.2,M,POINTB,POINTA*4A").expect("valid");
    let b = Bod::parse(&frame.fields);
    assert!((b.bear_true.expect("bear_true") - 97.0).abs() < 1e-4);
    assert_eq!(b.bear_true_type, Some('T'));
    assert!((b.bear_mag.expect("bear_mag") - 103.2).abs() < 1e-4);
    assert_eq!(b.bear_mag_type, Some('M'));
    assert_eq!(b.wpt_dest.as_deref(), Some("POINTB"));
    assert_eq!(b.wpt_origin.as_deref(), Some("POINTA"));
}
