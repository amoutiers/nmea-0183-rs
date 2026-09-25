use nmea_0183_rs::nmea::sentences::Hms;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn hms_sim183_fixture_values_and_checksum() {
    // Fixture published in the Ninglu SIM183 NMEA0183 Tester Manual, page 12:
    // https://iprorwxhqiiqli5q.leadongcdn.cn/NLT-SIM183-SSEN%2BSIM183%2BNMEA0183%2BTester%2BManual%2BV150804-aidqiBppKkkRliSqroimplkk.pdf
    const FIXTURE: &str = "$--HMS,01,02,1.2*54";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid SIM183 HMS frame");
    let hms = Hms::parse(&frame.fields).expect("parse HMS");

    assert_eq!(hms.sensor1.as_deref(), Some("01"));
    assert_eq!(hms.sensor2.as_deref(), Some("02"));
    assert_eq!(hms.max_difference_deg, Some(1.2));
}

#[test]
fn hms_roundtrip() {
    let frame = parse_frame("$--HMS,01,02,1.2*54").expect("valid HMS");
    let original = Hms::parse(&frame.fields).expect("parse HMS");

    // `--` in the published fixture is a placeholder, not an encodable talker.
    let sentence = original.to_sentence("HE").expect("encode HMS");
    let frame = parse_frame(sentence.trim()).expect("re-parse HMS");
    assert_eq!(Hms::parse(&frame.fields), Some(original));
}
