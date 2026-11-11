#![cfg(feature = "nak")]

use nmea_0183_rs::nmea::sentences::Nak;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

#[test]
fn nak_values_specification() {
    // Example constructed from the IEC 61162-1:2024 NAK field specification.
    let frame = parse_frame("$ANNAK,UP,AID,999999990,11,*1E").expect("valid NAK");
    let nak = Nak::parse(&frame.fields);
    assert_eq!(nak.talker_id.as_deref(), Some("UP"));
    assert_eq!(nak.formatter.as_deref(), Some("AID"));
    assert_eq!(nak.identifier.as_deref(), Some("999999990"));
    assert_eq!(nak.reason.as_deref(), Some("11"));
    assert!(nak.text.is_none());
}

#[test]
fn nak_empty() {
    assert_eq!(Nak::parse(&[]), Nak::default());
}

#[test]
fn dispatch() {
    let frame = parse_frame("$ANNAK,UP,AID,999999990,11,*1E").expect("fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Nak(_)));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$ANNAK,UP,AID,999999990,11,*1E").expect("fixture");
    let original = NmeaSentence::parse(&frame);
    let line = original.to_sentence("AN").expect("encode");
    assert_eq!(
        NmeaSentence::parse(&parse_frame(&line).expect("decode")),
        original
    );
}

#[test]
fn nak_encode_roundtrip() {
    let frame = parse_frame("$ANNAK,UP,AID,999999990,11,*1E").expect("fixture");
    let original = Nak::parse(&frame.fields);
    let line = original.to_sentence("AN").expect("encode");
    assert_eq!(
        Nak::parse(&parse_frame(&line).expect("decode").fields),
        original
    );
}
