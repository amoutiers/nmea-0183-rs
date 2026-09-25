use nmea_0183_rs::nmea::sentences::Trc;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn trc_navis_dp4000_fixture_values_checksum_and_roundtrip() {
    // Fixture published in the NAVIS NavDP 4000 Operation Manual, TRC Message:
    // https://alphatronmarine.com/files/secured/docuware_documents/144-DP%2BNavis%2BDP4000%2BOperation%2BManual%2B%2B1-5-2018.pdf
    const FIXTURE: &str = "$RCTRC,2,402.6,R,,V,88.3,B,C*50";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid Navis DP4000 TRC frame");
    let trc = Trc::parse(&frame.fields).expect("parse TRC");

    assert_eq!(trc.thruster_number.as_deref(), Some("2"));
    assert_eq!(trc.rpm_demand, Some(402.6));
    assert_eq!(trc.rpm_mode, Some('R'));
    assert!(trc.pitch_demand.is_none());
    assert_eq!(trc.pitch_mode, Some('V'));
    assert_eq!(trc.azimuth_demand, Some(88.3));
    assert_eq!(trc.operating_location, Some('B'));
    assert_eq!(trc.sentence_status, Some('C'));

    let sentence = trc.to_sentence("RC").expect("encode TRC");
    assert_eq!(sentence.trim(), FIXTURE);
    let frame = parse_frame(sentence.trim()).expect("re-parse TRC");
    assert_eq!(Trc::parse(&frame.fields), Some(trc));
}
