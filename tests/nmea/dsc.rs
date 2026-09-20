#![cfg(feature = "dsc")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Dsc;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$CDDSC,12,3380400790,12,06,00,1423108312,2019,,,S,E*6A").expect("valid");
    let dsc = Dsc::parse(&frame.fields);
    let sentence = dsc.to_sentence("CD").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dsc2 = Dsc::parse(&frame2.fields);
    assert_eq!(dsc, dsc2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$CDDSC,12,3380400790,12,06,00,1423108312,2019,,,S,E*6A").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dsc(_)));
}

#[test]
fn roundtrip() {
    let original = Dsc {
        format_specifier: Some("12".to_string()),
        address: Some("3380400790".to_string()),
        category: Some("12".to_string()),
        cmd1: Some("06".to_string()),
        cmd2: Some("00".to_string()),
        position: Some("1423108312".to_string()),
        time_or_tel: Some("2019".to_string()),
        mmsi: None,
        distress_cause: None,
        ack: Some('S'),
        expansion: Some('E'),
    };
    let sentence = original.to_sentence("CD").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dsc::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn dsc_values() {
    let frame =
        parse_frame("$CDDSC,12,3380400790,12,06,00,1423108312,2019,,,S,E*6A").expect("valid");
    let dsc = Dsc::parse(&frame.fields);
    assert_eq!(
        dsc,
        Dsc {
            format_specifier: Some("12".to_string()),
            address: Some("3380400790".to_string()),
            category: Some("12".to_string()),
            cmd1: Some("06".to_string()),
            cmd2: Some("00".to_string()),
            position: Some("1423108312".to_string()),
            time_or_tel: Some("2019".to_string()),
            mmsi: None,
            distress_cause: None,
            ack: Some('S'),
            expansion: Some('E'),
        }
    );
}
