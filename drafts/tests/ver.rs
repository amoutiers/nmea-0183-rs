use nmea_0183_rs::nmea::sentences::Ver;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn ver_ocean_signal_ata100_values_checksum_and_roundtrip() {
    // Public fixture: Ocean Signal ATA100 IEC 61162-1 digital-interface test report,
    // TUV SUD document 75936860-03 Issue 1, page 30,
    // https://fcc.report/FCC-ID/XYEATA100/4655050.pdf
    const FIXTURE: &str = "$AIVER,1,1,AI,OSG,777888999,1234567Q,ATA100,00.2.16,02.0.00,0*1F";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid Ocean Signal ATA100 VER frame");
    let ver = Ver::parse(&frame.fields).expect("parse VER");

    assert_eq!(ver.total_sentences, Some(1));
    assert_eq!(ver.sentence_number, Some(1));
    assert_eq!(ver.device_type.as_deref(), Some("AI"));
    assert_eq!(ver.vendor_id.as_deref(), Some("OSG"));
    assert_eq!(ver.unique_identifier.as_deref(), Some("777888999"));
    assert_eq!(ver.manufacturer_serial_number.as_deref(), Some("1234567Q"));
    assert_eq!(ver.model_code.as_deref(), Some("ATA100"));
    assert_eq!(ver.software_revision.as_deref(), Some("00.2.16"));
    assert_eq!(ver.hardware_revision.as_deref(), Some("02.0.00"));
    assert_eq!(ver.sequence_id.as_deref(), Some("0"));

    let sentence = ver.to_sentence("AI").expect("encode VER");
    assert_eq!(sentence.trim(), FIXTURE);
    let roundtrip = parse_frame(sentence.trim()).expect("re-parse VER");
    assert_eq!(Ver::parse(&roundtrip.fields), Some(ver));
}
