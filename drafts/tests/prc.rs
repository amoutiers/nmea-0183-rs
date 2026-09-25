use nmea_0183_rs::nmea::sentences::Prc;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn prc_jrc_vdr_datanalyzer_fixture_values_checksum_and_roundtrip() {
    // Fixture published in the JRC VDR DATAnalyzer User Manual, Log screen:
    // https://jrcbrasil.com/vda/manuals/vdr_datanalyzer_manual.pdf
    const FIXTURE: &str = "$RCPRC,,V,0.0,R,,V,,0*1C";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid JRC VDR DATAnalyzer PRC frame");
    let prc = Prc::parse(&frame.fields).expect("parse PRC");

    assert!(prc.lever_demand.is_none());
    assert_eq!(prc.lever_status, Some('V'));
    assert_eq!(prc.rpm_demand, Some(0.0));
    assert_eq!(prc.rpm_mode, Some('R'));
    assert!(prc.pitch_demand.is_none());
    assert_eq!(prc.pitch_mode, Some('V'));
    assert!(prc.operating_location.is_none());
    assert_eq!(prc.engine_or_shaft.as_deref(), Some("0"));

    let sentence = prc.to_sentence("RC").expect("encode PRC");
    let frame = parse_frame(sentence.trim()).expect("re-parse PRC");
    assert_eq!(Prc::parse(&frame.fields), Some(prc));
}
