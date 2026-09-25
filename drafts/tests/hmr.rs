use nmea_0183_rs::nmea::sentences::Hmr;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn hmr_sim183_fixture_values_and_checksum() {
    // Fixture published in the Ninglu SIM183 NMEA0183 Tester Manual, page 12:
    // https://www.manualslib.com/manual/2441097/Ninglu-Nmea0183.html
    const FIXTURE: &str = "$--HMR,01,02,1.2,0.1,A,123.4,A,M,0.1,E,123.5,A,M,0.1,E,1.2,E*52";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid SIM183 HMR frame");
    let hmr = Hmr::parse(&frame.fields).expect("parse HMR");

    assert_eq!(hmr.sensor_1_id.as_deref(), Some("01"));
    assert_eq!(hmr.sensor_2_id.as_deref(), Some("02"));
    assert_eq!(hmr.difference_limit, Some(1.2));
    assert_eq!(hmr.sensor_difference, Some(0.1));
    assert_eq!(hmr.warning_flag, Some('A'));
    assert_eq!(hmr.sensor_1_heading, Some(123.4));
    assert_eq!(hmr.sensor_1_status, Some('A'));
    assert_eq!(hmr.sensor_1_type, Some('M'));
    assert_eq!(hmr.sensor_1_deviation, Some(0.1));
    assert_eq!(hmr.sensor_1_deviation_ew, Some('E'));
    assert_eq!(hmr.sensor_2_heading, Some(123.5));
    assert_eq!(hmr.sensor_2_status, Some('A'));
    assert_eq!(hmr.sensor_2_type, Some('M'));
    assert_eq!(hmr.sensor_2_deviation, Some(0.1));
    assert_eq!(hmr.sensor_2_deviation_ew, Some('E'));
    assert_eq!(hmr.variation, Some(1.2));
    assert_eq!(hmr.variation_ew, Some('E'));
}

#[test]
fn hmr_roundtrip() {
    let frame = parse_frame("$--HMR,01,02,1.2,0.1,A,123.4,A,M,0.1,E,123.5,A,M,0.1,E,1.2,E*52")
        .expect("valid HMR");
    let original = Hmr::parse(&frame.fields).expect("parse HMR");

    let sentence = original.to_sentence("HC").expect("encode HMR");
    let frame = parse_frame(sentence.trim()).expect("re-parse HMR");
    assert_eq!(Hmr::parse(&frame.fields), Some(original));
}
