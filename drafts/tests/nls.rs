use nmea_0183_rs::nmea::sentences::{NavigationLightStatus, Nls};
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn nls_iec_61162_1_2024_fixture_values_and_checksum() {
    // Fixture published in IEC 61162-1:2024, 8.3.71, page 96:
    // https://previewnorm.com/iec/IEC%2061162-1-2024%20PDF.pdf
    const FIXTURE: &str = "$NLNLS,2,1,96,4,12,2,,3,1,,471,3,4,6,3,*68";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid IEC NLS frame");
    let nls = Nls::parse(&frame.fields).expect("parse NLS");

    assert_eq!(nls.total_sentences, Some(2));
    assert_eq!(nls.sentence_number, Some(1));
    assert_eq!(nls.sequential_message_id, Some(96));
    assert_eq!(nls.light_count, Some(4));
    assert_eq!(
        nls.lights,
        [
            NavigationLightStatus {
                light_id: Some(12),
                status: Some(2),
                remaining_working_hours: None,
            },
            NavigationLightStatus {
                light_id: Some(3),
                status: Some(1),
                remaining_working_hours: None,
            },
            NavigationLightStatus {
                light_id: Some(471),
                status: Some(3),
                remaining_working_hours: Some(4),
            },
            NavigationLightStatus {
                light_id: Some(6),
                status: Some(3),
                remaining_working_hours: None,
            },
        ]
    );
}

#[test]
fn nls_roundtrip() {
    let frame = parse_frame("$NLNLS,2,1,96,4,12,2,,3,1,,471,3,4,6,3,*68").expect("valid NLS");
    let original = Nls::parse(&frame.fields).expect("parse NLS");

    let sentence = original.to_sentence("NL").expect("encode NLS");
    let frame = parse_frame(sentence.trim()).expect("re-parse NLS");
    assert_eq!(Nls::parse(&frame.fields), Some(original));
}
