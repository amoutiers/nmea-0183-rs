#![cfg(feature = "dse")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::{Dse, DseDataSet};
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$CDDSE,1,1,A,3380400790,00,46504437*15").expect("valid");
    let dse = Dse::parse(&frame.fields).expect("parse");
    let sentence = dse.to_sentence("CD").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dse2 = Dse::parse(&frame2.fields).expect("parse");
    assert_eq!(dse, dse2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$CDDSE,1,1,A,3380400790,00,46504437*15").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dse(_)));
}

#[test]
fn roundtrip() {
    let original = Dse {
        total: Some(1),
        number: Some(1),
        ack: Some('A'),
        mmsi: Some("3380400790".to_string()),
        datasets: vec![DseDataSet {
            code: Some("00".to_string()),
            data: Some("46504437".to_string()),
        }],
    };
    let sentence = original.to_sentence("CD").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dse::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn dse_values() {
    let frame = parse_frame("$CDDSE,1,1,A,3380400790,00,46504437*15").expect("valid");
    let dse = Dse::parse(&frame.fields).expect("parse");
    assert_eq!(
        dse,
        Dse {
            total: Some(1),
            number: Some(1),
            ack: Some('A'),
            mmsi: Some("3380400790".to_string()),
            datasets: vec![DseDataSet {
                code: Some("00".to_string()),
                data: Some("46504437".to_string()),
            }],
        }
    );
}

#[test]
fn retains_datasets_after_missing_code() {
    let dse =
        Dse::parse(&["1", "1", "A", "123", "", "first", "01", "second", "02"]).expect("parse");
    assert_eq!(dse.datasets.len(), 3);
    assert_eq!(dse.datasets[0].code, None);
    assert_eq!(dse.datasets[1].code.as_deref(), Some("01"));
    assert_eq!(dse.datasets[2].data, None);
}
