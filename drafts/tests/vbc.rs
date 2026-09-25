use nmea_0183_rs::nmea::sentences::Vbc;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn vbc_maritime_dissector_fixture_values_checksums_and_roundtrip() {
    // Fixture from fkie-cad/maritime-dissector's
    // tests/iec-61162-450-nmea-multisentence.pcap (tag n:884):
    // https://github.com/fkie-cad/maritime-dissector
    const FIXTURE: &str =
        "\\s:IN0001,n:884,c:1683881316755*48\\$INVBC,0.0,-0.0,-0.0,-0.0,A,-0.0,-0.0,-0.0,-0.0,A*7D";

    let (tag, sentence) = FIXTURE[1..].split_once('\\').expect("tag block delimiter");
    let (tag_body, tag_checksum) = tag.split_once('*').expect("tag checksum");
    assert_eq!(
        tag_body
            .bytes()
            .fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(tag_checksum, 16).expect("hex tag checksum")
    );

    let (body, checksum) = sentence[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid maritime-dissector VBC frame");
    assert_eq!(frame.tag_block, Some("s:IN0001,n:884,c:1683881316755"));
    let vbc = Vbc::parse(&frame.fields).expect("parse VBC");

    assert_eq!(vbc.longitudinal_water_speed, Some(0.0));
    assert_eq!(vbc.bow_transverse_water_speed, Some(-0.0));
    assert_eq!(vbc.ccrp_transverse_water_speed, Some(-0.0));
    assert_eq!(vbc.stern_transverse_water_speed, Some(-0.0));
    assert_eq!(vbc.water_speed_status, Some('A'));
    assert_eq!(vbc.longitudinal_ground_speed, Some(-0.0));
    assert_eq!(vbc.bow_transverse_ground_speed, Some(-0.0));
    assert_eq!(vbc.ccrp_transverse_ground_speed, Some(-0.0));
    assert_eq!(vbc.stern_transverse_ground_speed, Some(-0.0));
    assert_eq!(vbc.ground_speed_status, Some('A'));

    let encoded = vbc.to_sentence("IN").expect("encode VBC");
    let frame = parse_frame(encoded.trim()).expect("re-parse VBC");
    assert_eq!(Vbc::parse(&frame.fields), Some(vbc));
}
