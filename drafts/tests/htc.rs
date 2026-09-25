use nmea_0183_rs::nmea::sentences::Htc;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn htc_sim183_fixture_values_and_checksum() {
    // Fixture published in the Ninglu SIM183 NMEA0183 Tester Manual, page 12:
    // https://www.manualslib.com/manual/2441097/Ninglu-Nmea0183.html
    const FIXTURE: &str = "$--HTC,A,123.4,R,M,N,,,,,,,,T*1D";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid SIM183 HTC frame");
    let htc = Htc::parse(&frame.fields).expect("parse HTC");

    assert_eq!(htc.override_status, Some('A'));
    assert_eq!(htc.commanded_rudder_angle, Some(123.4));
    assert_eq!(htc.commanded_rudder_direction, Some('R'));
    assert_eq!(htc.selected_steering_mode, Some('M'));
    assert_eq!(htc.turn_mode, Some('N'));
    assert_eq!(htc.commanded_rudder_limit, None);
    assert_eq!(htc.commanded_off_heading_limit, None);
    assert_eq!(htc.commanded_radius_of_turn, None);
    assert_eq!(htc.commanded_rate_of_turn, None);
    assert_eq!(htc.commanded_heading_to_steer, None);
    assert_eq!(htc.commanded_off_track_limit, None);
    assert_eq!(htc.commanded_track, None);
    assert_eq!(htc.heading_reference, Some('T'));
    assert_eq!(htc.sentence_status, None);
}

#[test]
fn htc_roundtrip() {
    let frame = parse_frame("$--HTC,A,123.4,R,M,N,,,,,,,,T*1D").expect("valid HTC");
    let original = Htc::parse(&frame.fields).expect("parse HTC");

    let sentence = original.to_sentence("HE").expect("encode HTC");
    let frame = parse_frame(sentence.trim()).expect("re-parse HTC");
    assert_eq!(Htc::parse(&frame.fields), Some(original));
}
