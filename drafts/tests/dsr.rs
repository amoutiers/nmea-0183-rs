use nmea_0183_rs::nmea::sentences::Dsr;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn dsr_sim183_fixture_values_and_checksum() {
    // Fixture published in the Ninglu SIM183 NMEA0183 Tester Manual, page 12:
    // https://www.manualslib.com/manual/2441097/Ninglu-Nmea0183.html
    const FIXTURE: &str = "$--DSR,1,1,3601234560,00,12345678,E*0A";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid SIM183 DSR frame");
    let dsr = Dsr::parse(&frame.fields).expect("parse DSR");

    assert_eq!(dsr.total_sentences, Some(1));
    assert_eq!(dsr.sentence_number, Some(1));
    assert_eq!(dsr.vessel_mmsi.as_deref(), Some("3601234560"));
    assert_eq!(
        dsr.data_sets,
        [(Some("00".into()), Some("12345678".into()))]
    );
    assert_eq!(dsr.expansion_indicator, Some('E'));
}

#[test]
fn dsr_roundtrip() {
    let frame = parse_frame("$--DSR,1,1,3601234560,00,12345678,E*0A").expect("valid DSR");
    let original = Dsr::parse(&frame.fields).expect("parse DSR");

    let sentence = original.to_sentence("CD").expect("encode DSR");
    let frame = parse_frame(sentence.trim()).expect("re-parse DSR");
    assert_eq!(Dsr::parse(&frame.fields), Some(original));
}
