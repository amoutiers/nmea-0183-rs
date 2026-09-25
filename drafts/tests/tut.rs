use nmea_0183_rs::nmea::sentences::Tut;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn tut_nmea_301_ascii_fixture_values_and_roundtrip() {
    // Fixture published in NMEA 0183 v3.01, "TUT - Transmission of Multi-language Text":
    // https://www.plaisance-pratique.com/IMG/pdf/NMEA0183-2.pdf
    const FIXTURE: &str = "$INTUT,SD,01,01,1,A,5368616C6C6F7720576174657221*4B";

    let (body, reported_checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(reported_checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid NMEA 3.01 TUT frame");
    let tut = Tut::parse(&frame.fields).expect("parse TUT");

    assert_eq!(tut.source_identifier.as_deref(), Some("SD"));
    assert_eq!(tut.total_sentences.as_deref(), Some("01"));
    assert_eq!(tut.sentence_number.as_deref(), Some("01"));
    assert_eq!(tut.sequential_message_identifier, Some(1));
    assert_eq!(tut.translation_code.as_deref(), Some("A"));
    assert_eq!(
        tut.text_body.as_deref(),
        Some("5368616C6C6F7720576174657221")
    );

    let sentence = tut.to_sentence("IN").expect("encode TUT");
    assert_eq!(sentence.trim(), FIXTURE);
    let frame = parse_frame(sentence.trim()).expect("re-parse TUT");
    assert_eq!(Tut::parse(&frame.fields), Some(tut));
}
