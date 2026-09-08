#![cfg(feature = "phtro")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Phtro;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$PHTRO,10.37,P,177.62,T*65").expect("valid");
    let phtro = Phtro::parse(&frame.fields).expect("parse");
    let sentence = phtro.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let phtro2 = Phtro::parse(&frame2.fields).expect("parse");
    assert_eq!(phtro, phtro2);
}

#[test]
fn roundtrip() {
    let original = Phtro {
        pitch: Some(10.37),
        bow: Some('P'),
        roll: Some(177.62),
        port: Some('T'),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Phtro::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$PHTRO,10.37,P,177.62,T*65").expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Phtro(_)
    ));
}

#[test]
fn phtro_values() {
    let frame = parse_frame("$PHTRO,10.37,P,177.62,T*65").expect("valid");
    let p = Phtro::parse(&frame.fields).expect("parse");
    assert!((p.pitch.expect("pitch") - 10.37).abs() < 0.01);
    assert_eq!(p.bow, Some('P'));
    assert!((p.roll.expect("roll") - 177.62).abs() < 0.01);
    assert_eq!(p.port, Some('T'));
}
