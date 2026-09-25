use nmea_0183_rs::nmea::sentences::Alm;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn alm_novatel_oem7_fixture_values_and_checksum() {
    // Fixture published by NovAtel OEM7 GPALM documentation:
    // https://docs.novatel.com/OEM7/Content/Logs/GPALM.htm
    const FIXTURE: &str =
        "$GPALM,30,01,01,2210,00,617b,0f,1da7,fd70,a10d0a,24de91,6fe696,16263f,17c,ffe*7B";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid NovAtel OEM7 ALM frame");
    let alm = Alm::parse(&frame.fields).expect("parse ALM");

    assert_eq!(alm.total_messages, Some(30));
    assert_eq!(alm.message_number, Some(1));
    assert_eq!(alm.prn, Some(1));
    assert_eq!(alm.gps_week, Some(2210));
    assert_eq!(alm.sv_health.as_deref(), Some("00"));
    assert_eq!(alm.eccentricity.as_deref(), Some("617b"));
    assert_eq!(alm.almanac_reference_time.as_deref(), Some("0f"));
    assert_eq!(alm.inclination_angle.as_deref(), Some("1da7"));
    assert_eq!(alm.right_ascension_rate.as_deref(), Some("fd70"));
    assert_eq!(alm.sqrt_semi_major_axis.as_deref(), Some("a10d0a"));
    assert_eq!(alm.argument_of_perigee.as_deref(), Some("24de91"));
    assert_eq!(alm.longitude_of_ascension_node.as_deref(), Some("6fe696"));
    assert_eq!(alm.mean_anomaly.as_deref(), Some("16263f"));
    assert_eq!(alm.clock_parameter_af0.as_deref(), Some("17c"));
    assert_eq!(alm.clock_parameter_af1.as_deref(), Some("ffe"));
}

#[test]
fn alm_roundtrip() {
    let frame = parse_frame(
        "$GPALM,30,01,01,2210,00,617b,0f,1da7,fd70,a10d0a,24de91,6fe696,16263f,17c,ffe*7B",
    )
    .expect("valid ALM");
    let original = Alm::parse(&frame.fields).expect("parse ALM");

    let sentence = original.to_sentence("GP").expect("encode ALM");
    let frame = parse_frame(sentence.trim()).expect("re-parse ALM");
    assert_eq!(Alm::parse(&frame.fields), Some(original));
}
