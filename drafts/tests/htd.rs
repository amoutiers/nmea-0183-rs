use nmea_0183_rs::nmea::sentences::Htd;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn htd_deif_xdi_fixture_values_and_checksum() {
    // Fixture published in the DEIF XDi Series Designer's Handbook, page 169:
    // https://www.manualslib.com/manual/2903353/Deif-Xdi-Series.html?page=169
    const FIXTURE: &str =
        "$HEHTD,A,0.0,L,M,T,45.0,25.0,0.5,3275.0,200.0,1.0,250.0,T,A,A,A,270.0*67";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid DEIF XDi HTD frame");
    let htd = Htd::parse(&frame.fields).expect("parse HTD");

    assert_eq!(htd.override_status, Some('A'));
    assert_eq!(htd.commanded_rudder_angle, Some(0.0));
    assert_eq!(htd.commanded_rudder_direction, Some('L'));
    assert_eq!(htd.selected_steering_mode, Some('M'));
    assert_eq!(htd.turn_mode, Some('T'));
    assert_eq!(htd.commanded_rudder_limit, Some(45.0));
    assert_eq!(htd.commanded_off_heading_limit, Some(25.0));
    assert_eq!(htd.commanded_radius_of_turn, Some(0.5));
    assert_eq!(htd.commanded_rate_of_turn, Some(3275.0));
    assert_eq!(htd.commanded_heading_to_steer, Some(200.0));
    assert_eq!(htd.commanded_off_track_limit, Some(1.0));
    assert_eq!(htd.commanded_track, Some(250.0));
    assert_eq!(htd.heading_reference, Some('T'));
    assert_eq!(htd.rudder_status, Some('A'));
    assert_eq!(htd.off_heading_status, Some('A'));
    assert_eq!(htd.off_track_status, Some('A'));
    assert_eq!(htd.vessel_heading, Some(270.0));
}

#[test]
fn htd_roundtrip() {
    let frame =
        parse_frame("$HEHTD,A,0.0,L,M,T,45.0,25.0,0.5,3275.0,200.0,1.0,250.0,T,A,A,A,270.0*67")
            .expect("valid HTD");
    let original = Htd::parse(&frame.fields).expect("parse HTD");

    let sentence = original.to_sentence("HE").expect("encode HTD");
    let frame = parse_frame(sentence.trim()).expect("re-parse HTD");
    assert_eq!(Htd::parse(&frame.fields), Some(original));
}
