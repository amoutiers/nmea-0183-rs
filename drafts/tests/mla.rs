use nmea_0183_rs::nmea::sentences::Mla;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn mla_novatel_oem7_values_and_roundtrip() {
    // Fixture published by NovAtel OEM7 GLMLA documentation:
    // https://docs.novatel.com/OEM7/Content/Logs/GLMLA.htm
    const FIXTURE: &str =
        "$GLMLA,23,01,65,0864,81,01f4,1e,96d9,0000,34bf42,129a60,0c37e8,001e0d,000,202*60";

    let body = FIXTURE
        .strip_prefix('$')
        .and_then(|sentence| sentence.split_once('*').map(|(body, _)| body))
        .expect("NMEA fixture has body and checksum");
    let checksum = body.bytes().fold(0u8, |checksum, byte| checksum ^ byte);
    assert_eq!(checksum, 0x60);

    let frame = parse_frame(FIXTURE).expect("valid NovAtel OEM7 MLA frame");
    let mla = Mla::parse(&frame.fields).expect("parse MLA");

    assert_eq!(mla.almanac_count, Some(23));
    assert_eq!(mla.almanac_number.as_deref(), Some("01"));
    assert_eq!(mla.slot, Some(65));
    assert_eq!(mla.calendar_day_count.as_deref(), Some("0864"));
    assert_eq!(mla.health_and_frequency.as_deref(), Some("81"));
    assert_eq!(mla.eccentricity.as_deref(), Some("01f4"));
    assert_eq!(mla.delta_t_dot.as_deref(), Some("1e"));
    assert_eq!(mla.argument_of_perigee.as_deref(), Some("96d9"));
    assert_eq!(mla.t_16_msb.as_deref(), Some("0000"));
    assert_eq!(mla.delta_t.as_deref(), Some("34bf42"));
    assert_eq!(mla.t_pi.as_deref(), Some("129a60"));
    assert_eq!(mla.longitude_of_ascending_node.as_deref(), Some("0c37e8"));
    assert_eq!(mla.delta_i.as_deref(), Some("001e0d"));
    assert_eq!(mla.t_12_lsb.as_deref(), Some("000"));
    assert_eq!(mla.time_scale_shift.as_deref(), Some("202"));

    let sentence = mla.to_sentence("GL").expect("encode MLA");
    assert_eq!(sentence.trim(), FIXTURE);
    let frame = parse_frame(sentence.trim()).expect("re-parse MLA");
    assert_eq!(Mla::parse(&frame.fields), Some(mla));
}
