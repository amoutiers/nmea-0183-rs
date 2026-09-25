use nmea_0183_rs::nmea::sentences::Dsi;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn dsi_sim183_fixture_values_and_checksum() {
    // Fixture published in the Ninglu SIM183 NMEA0183 Tester Manual, page 12:
    // https://www.manualslib.com/manual/2441097/Ninglu-Nmea0183.html
    const FIXTURE: &str = "$--DSI,1,1,3601234560,,,,00,12345678,E*3D";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid SIM183 DSI frame");
    let dsi = Dsi::parse(&frame.fields).expect("parse DSI");

    assert_eq!(dsi.total_sentences, Some(1));
    assert_eq!(dsi.sentence_number, Some(1));
    assert_eq!(dsi.vessel_mmsi.as_deref(), Some("3601234560"));
    assert_eq!(dsi.vessel_course.as_deref(), None);
    assert_eq!(dsi.vessel_type.as_deref(), None);
    assert_eq!(dsi.geographic_area.as_deref(), None);
    assert_eq!(
        dsi.command_sets,
        [(Some("00".into()), Some("12345678".into()))]
    );
    assert_eq!(dsi.expansion_indicator, Some('E'));
}

#[test]
fn dsi_roundtrip() {
    let frame = parse_frame("$--DSI,1,1,3601234560,,,,00,12345678,E*3D").expect("valid DSI");
    let original = Dsi::parse(&frame.fields).expect("parse DSI");

    // `--` in the published fixture is a documentation placeholder, not a talker ID.
    let sentence = original.to_sentence("GP").expect("encode DSI");
    let frame = parse_frame(sentence.trim()).expect("re-parse DSI");
    assert_eq!(Dsi::parse(&frame.fields), Some(original));
}
