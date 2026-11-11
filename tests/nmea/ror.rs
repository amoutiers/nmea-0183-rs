#![cfg(feature = "ror")]

use nmea_0183_rs::nmea::sentences::Ror;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

#[test]
fn ror_values_pynmea2_ship() {
    // Fixture published in the pynmea2-ship README:
    // https://github.com/acousticwave/pynmea2-ship
    const FIXTURE: &str = "$AGROR,0.1,A,,V,B*1F";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid pynmea2-ship ROR frame");
    let ror = Ror::parse(&frame.fields);

    assert!((ror.starboard_order.expect("value") - 0.1).abs() < 1e-5);
    assert_eq!(ror.starboard_status, Some('A'));
    assert!(ror.port_order.is_none());
    assert_eq!(ror.port_status, Some('V'));
    assert_eq!(ror.source, Some('B'));
    assert_eq!(ror.center_order, None);
    assert_eq!(ror.center_status, None);
    assert_eq!(ror.bow_order, None);
    assert_eq!(ror.bow_status, None);
}

#[test]
fn ror_roundtrip() {
    let frame = parse_frame("$AGROR,0.1,A,,V,B*1F").expect("valid ROR");
    let original = Ror::parse(&frame.fields);

    let sentence = original.to_sentence("AG").expect("encode ROR");
    let frame = parse_frame(sentence.trim()).expect("re-parse ROR");
    assert_eq!(Ror::parse(&frame.fields), original);
}

#[test]
fn ror_extended_fields_iec2024() {
    let original = Ror::parse(&["1", "A", "2", "A", "B", "3", "A", "4", "V"]);
    let line = original.to_sentence("AG").expect("encode ROR");
    let frame = parse_frame(&line).expect("decode ROR");
    assert_eq!(frame.fields, ["1", "A", "2", "A", "B", "3", "A", "4", "V"]);
}

#[test]
fn ror_empty() {
    assert_eq!(Ror::parse(&[]), Ror::default());
}

#[test]
fn dispatch() {
    let frame = parse_frame("$AGROR,0.1,A,,V,B*1F").expect("fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Ror(_)));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$AGROR,0.1,A,,V,B*1F").expect("fixture");
    let original = NmeaSentence::parse(&frame);
    let line = original.to_sentence("AG").expect("encode");
    assert_eq!(
        NmeaSentence::parse(&parse_frame(&line).expect("decode")),
        original
    );
}

#[test]
fn ror_encode_roundtrip() {
    let frame = parse_frame("$AGROR,0.1,A,,V,B*1F").expect("fixture");
    let original = Ror::parse(&frame.fields);
    let line = original.to_sentence("AG").expect("encode");
    assert_eq!(
        Ror::parse(&parse_frame(&line).expect("decode").fields),
        original
    );
}
