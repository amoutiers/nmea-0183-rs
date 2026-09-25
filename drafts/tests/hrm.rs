use nmea_0183_rs::nmea::sentences::Hrm;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn hrm_daniamant_fixture_values_checksum_and_roundtrip() {
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
    let hrm = Hrm::parse(&frame.fields).expect("parse HRM");

    assert_eq!(hrm.heel_angle, Some(-2.2));
    assert_eq!(hrm.roll_period, Some(11.1));
    assert_eq!(hrm.roll_amplitude_port, Some(3.3));
    assert_eq!(hrm.roll_amplitude_starboard, Some(4.4));
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
    assert_eq!(Hrm::parse(&roundtrip.fields), Some(hrm));
}
