use nmea_0183_rs::nmea::sentences::Gfa;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

// Public fixture: John Deere/NavCom Onyx Technical Reference Manual Rev. A,
// table 156 (p. 293): https://www.deere.com/assets/pdfs/navcom/User%20Manuals/96-312008-3001%20RevA17i_Onyx%20TRM.pdf
const GFA_FIXTURE: &str =
    "$GNGFA,224229.00,0001.7,0002.9,000.43,000.22,014.4868,000.83,0010.0,SCC*0C";

#[test]
fn gfa_deere_navcom_values() {
    let (body, checksum) = GFA_FIXTURE
        .trim_start_matches('$')
        .split_once('*')
        .expect("fixture has checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |xor, byte| xor ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(GFA_FIXTURE).expect("valid Deere/NavCom GFA frame");
    let gfa = Gfa::parse(&frame.fields).expect("parse GFA");

    assert_eq!(gfa.time.as_deref(), Some("224229.00"));
    assert_eq!(gfa.horizontal_protection_level, Some(1.7));
    assert_eq!(gfa.vertical_protection_level, Some(2.9));
    assert_eq!(gfa.semi_major_std_dev, Some(0.43));
    assert_eq!(gfa.semi_minor_std_dev, Some(0.22));
    assert_eq!(gfa.semi_major_orientation, Some(14.4868));
    assert_eq!(gfa.altitude_std_dev, Some(0.83));
    assert_eq!(gfa.selected_accuracy_level, Some(10.0));
    assert_eq!(gfa.integrity_status.as_deref(), Some("SCC"));
}

#[test]
fn gfa_roundtrip() {
    let original = Gfa {
        time: Some("224229.00".to_string()),
        horizontal_protection_level: Some(1.7),
        vertical_protection_level: Some(2.9),
        semi_major_std_dev: Some(0.43),
        semi_minor_std_dev: Some(0.22),
        semi_major_orientation: Some(14.4868),
        altitude_std_dev: Some(0.83),
        selected_accuracy_level: Some(10.0),
        integrity_status: Some("SCC".to_string()),
    };

    let sentence = original.to_sentence("GN").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    assert_eq!(Gfa::parse(&frame.fields), Some(original));
}
