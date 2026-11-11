#![cfg(feature = "hrm")]

use nmea_0183_rs::nmea::sentences::Hrm;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

#[test]
fn hrm_values_daniamant() {
    // Public fixture: Daniamant DanEI-300T Operation and Service Manual, appendix B.2,
    // page 28, https://daniamant.com/wp-content/uploads/2019/06/DanEI-300T-Electronic-Inclinometer-for-Tug-Boat-Operation-and-Service-Manual.pdf
    const FIXTURE: &str = "$ICHRM,-2.2,11.1,3.3,4.4,A,,,,,*00";

    let (body, reported_checksum) = FIXTURE[1..].split_once('*').expect("checksum separator");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(reported_checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid Daniamant HRM frame");
    assert_eq!(frame.fields.len(), 10);
    let hrm = Hrm::parse(&frame.fields);

    assert!((hrm.heel_angle.expect("value") - (-2.2)).abs() < 1e-5);
    assert!((hrm.roll_period.expect("value") - (11.1)).abs() < 1e-5);
    assert!((hrm.roll_amplitude_port.expect("value") - (3.3)).abs() < 1e-5);
    assert!((hrm.roll_amplitude_starboard.expect("value") - (4.4)).abs() < 1e-5);
    assert_eq!(hrm.status, Some('A'));
    assert_eq!(hrm.roll_peak_hold_port, None);
    assert_eq!(hrm.roll_peak_hold_starboard, None);
    assert_eq!(hrm.peak_hold_reset_time, None);
    assert_eq!(hrm.peak_hold_reset_day, None);
    assert_eq!(hrm.peak_hold_reset_month, None);
    assert_eq!(hrm.peak_hold_reset_year, None);
    assert_eq!(hrm.heel_angle_alert_threshold, None);
    assert_eq!(hrm.sentence_status, None);

    let sentence = hrm.to_sentence("IC").expect("encode HRM");
    let roundtrip = parse_frame(sentence.trim()).expect("re-parse HRM");
    assert_eq!(roundtrip.fields.len(), 13);
    assert_eq!(Hrm::parse(&roundtrip.fields), hrm);
}

#[test]
fn hrm_empty() {
    assert_eq!(Hrm::parse(&[]), Hrm::default());
}

#[test]
fn dispatch() {
    let frame = parse_frame("$ICHRM,-2.2,11.1,3.3,4.4,A,,,,,*00").expect("fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Hrm(_)));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$ICHRM,-2.2,11.1,3.3,4.4,A,,,,,*00").expect("fixture");
    let original = NmeaSentence::parse(&frame);
    let line = original.to_sentence("IC").expect("encode");
    assert_eq!(
        NmeaSentence::parse(&parse_frame(&line).expect("decode")),
        original
    );
}

#[test]
fn hrm_encode_roundtrip() {
    let frame = parse_frame("$ICHRM,-2.2,11.1,3.3,4.4,A,,,,,*00").expect("fixture");
    let original = Hrm::parse(&frame.fields);
    let line = original.to_sentence("IC").expect("encode");
    assert_eq!(
        Hrm::parse(&parse_frame(&line).expect("decode").fields),
        original
    );
}

#[test]
fn hrm_extended_fields_roundtrip() {
    let fields = [
        "1", "2", "3", "4", "A", "5", "6", "123456", "7", "8", "2024", "9", "R",
    ];
    let original = Hrm::parse(&fields);
    let line = original.to_sentence("IC").expect("encode HRM");
    assert_eq!(parse_frame(&line).expect("decode HRM").fields, fields);
}
