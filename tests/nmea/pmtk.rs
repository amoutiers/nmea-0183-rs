#![cfg(feature = "pmtk")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Pmtk;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$PMTK001,604,3*32").expect("valid");
    let pmtk = Pmtk::parse(&frame.fields).expect("parse");
    let sentence = pmtk.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let pmtk2 = Pmtk::parse(&frame2.fields).expect("parse");
    assert_eq!(pmtk, pmtk2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$PMTK001,604,3*32").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Pmtk(_)));
}

#[test]
fn roundtrip() {
    let original = Pmtk {
        cmd: Some(604),
        flag: Some(3),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pmtk::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
