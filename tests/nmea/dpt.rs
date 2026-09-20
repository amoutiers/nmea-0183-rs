#![cfg(feature = "dpt")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Dpt;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIDPT,4.1,0.0*45").expect("valid");
    let dpt = Dpt::parse(&frame.fields).expect("parse");
    let sentence = dpt.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dpt2 = Dpt::parse(&frame2.fields).expect("parse");
    assert_eq!(dpt, dpt2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIDPT,4.1,0.0*45").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dpt(_)));
}

#[test]
fn roundtrip() {
    let original = Dpt {
        depth: Some(12.7),
        offset: Some(-1.5),
        rangescale: None,
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dpt::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn dpt_values() {
    let frame = parse_frame("$IIDPT,4.1,0.0*45").expect("valid");
    let d = Dpt::parse(&frame.fields).expect("parse");
    assert!((d.depth.expect("depth") - 4.1).abs() < 1e-4);
    assert!((d.offset.expect("offset") - 0.0).abs() < 1e-4);
    assert!(d.rangescale.is_none());
}

#[test]
fn dpt_nonfinite_is_an_error_not_missing_data() {
    use nmea_0183_rs::{EncodeError, StrictEncodeError};
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let dpt = Dpt {
            depth: Some(value),
            offset: None,
            rangescale: None,
        };
        assert_eq!(dpt.encode(), Err(EncodeError::NonFiniteNumber));
        assert_eq!(dpt.to_sentence("SD"), Err(EncodeError::NonFiniteNumber));
        assert_eq!(
            dpt.to_sentence_strict("SD"),
            Err(StrictEncodeError::Encode(EncodeError::NonFiniteNumber))
        );
    }
}
