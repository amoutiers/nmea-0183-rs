#![cfg(feature = "ver")]

use nmea_0183_rs::nmea::{NmeaEncodable, sentences::Ver};
use nmea_0183_rs::{NmeaSentence, parse_frame};

const FIXTURE: &str = "$AIVER,1,1,AI,OSG,777888999,1234567Q,ATA100,00.2.16,02.0.00,0*1F";
// Ocean Signal ATA100 IEC 61162-1 digital-interface test report, p. 30.
// https://fcc.report/FCC-ID/XYEATA100/4655050.pdf

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid VER frame");
    let original = Ver::parse(&frame.fields);
    let sentence = original.to_sentence("AI").expect("encode VER");
    let encoded = parse_frame(sentence.trim()).expect("re-parse VER");
    assert_eq!(Ver::parse(&encoded.fields), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid VER frame");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Ver(_)));
}

#[test]
fn ver_values_ocean_signal() {
    let frame = parse_frame(FIXTURE).expect("valid Ocean Signal ATA100 VER frame");
    let ver = Ver::parse(&frame.fields);

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
}

#[test]
fn roundtrip() {
    let original = Ver {
        total_sentences: Some(1),
        sentence_number: Some(1),
        device_type: Some("AI".into()),
        vendor_id: Some("OSG".into()),
        unique_identifier: Some("777888999".into()),
        manufacturer_serial_number: Some("1234567Q".into()),
        model_code: Some("ATA100".into()),
        software_revision: Some("00.2.16".into()),
        hardware_revision: Some("02.0.00".into()),
        sequence_id: Some("0".into()),
    };

    let sentence = original.to_sentence("AI").expect("encode VER");
    let frame = parse_frame(sentence.trim()).expect("re-parse VER");
    assert_eq!(Ver::parse(&frame.fields), original);
}
