use nmea_0183_rs::nmea::sentences::Trd;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn trd_navis_dp4000_fixture_values_checksum_and_roundtrip() {
    // Fixture published in the NAVIS NavDP 4000 Operation Manual, TRD Message:
    // https://alphatronmarine.com/files/secured/docuware_documents/144-DP%2BNavis%2BDP4000%2BOperation%2BManual%2B%2B1-5-2018.pdf
    const FIXTURE: &str = "$RCTRD,2,400.5,R,,V,90.8*55";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid Navis DP4000 TRD frame");
    let trd = Trd::parse(&frame.fields).expect("parse TRD");

    assert_eq!(trd.thruster_number.as_deref(), Some("2"));
    assert!((trd.rpm_response.expect("RPM response") - 400.5).abs() < f32::EPSILON);
    assert_eq!(trd.rpm_mode, Some('R'));
    assert!(trd.pitch_response.is_none());
    assert_eq!(trd.pitch_mode, Some('V'));
    assert!((trd.azimuth_response.expect("azimuth response") - 90.8).abs() < f32::EPSILON);

    let sentence = trd.to_sentence("RC").expect("encode TRD");
    assert_eq!(sentence.trim(), FIXTURE);
    let frame = parse_frame(sentence.trim()).expect("re-parse TRD");
    assert_eq!(Trd::parse(&frame.fields), Some(trd));
}
