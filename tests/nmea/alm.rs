#![cfg(feature = "alm")]

use nmea_0183_rs::nmea::sentences::Alm;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

// NovAtel OEM7 GPALM example, with hexadecimal payloads preserved verbatim.
// https://docs.novatel.com/OEM7/Content/Logs/GPALM.htm
const FIXTURE: &str =
    "$GPALM,30,01,01,2210,00,617b,0f,1da7,fd70,a10d0a,24de91,6fe696,16263f,17c,ffe*7B";

#[test]
fn alm_values_novatel_oem7() {
    let frame = parse_frame(FIXTURE).expect("valid NovAtel OEM7 ALM frame");
    let alm = Alm::parse(&frame.fields);

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
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid ALM");
    let original = NmeaSentence::parse(&frame);

    let sentence = original.to_sentence("GP").expect("encode ALM variant");
    let frame = parse_frame(&sentence).expect("re-parse ALM");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid ALM");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Alm(_)));
}

#[test]
fn roundtrip() {
    let original = Alm {
        total_messages: Some(30),
        message_number: Some(1),
        prn: Some(1),
        gps_week: Some(2210),
        sv_health: Some("00".into()),
        eccentricity: Some("617b".into()),
        almanac_reference_time: Some("0f".into()),
        inclination_angle: Some("1da7".into()),
        right_ascension_rate: Some("fd70".into()),
        sqrt_semi_major_axis: Some("a10d0a".into()),
        argument_of_perigee: Some("24de91".into()),
        longitude_of_ascension_node: Some("6fe696".into()),
        mean_anomaly: Some("16263f".into()),
        clock_parameter_af0: Some("17c".into()),
        clock_parameter_af1: Some("ffe".into()),
    };

    let sentence = original.to_sentence("GP").expect("encode ALM");
    let frame = parse_frame(sentence.trim()).expect("re-parse ALM");
    assert_eq!(Alm::parse(&frame.fields), original);
}
