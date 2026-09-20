#![cfg(feature = "alc")]
use nmea_0183_rs::nmea::{
    NmeaEncodable,
    sentences::{Alc, AlcEntry},
};
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn alc_values() {
    let frame = parse_frame("$FBALC,02,01,03,02,FEB,01,02,03,TEB,02,03,04*5F").expect("valid");
    let alc = Alc::parse(&frame.fields).expect("parse");
    assert_eq!(alc.num_frags, Some(2));
    assert_eq!(alc.frag_num, Some(1));
    assert_eq!(alc.msg_id, Some(3));
    assert_eq!(alc.entries_num, Some(2));
    assert_eq!(alc.entries.len(), 2);
    assert_eq!(alc.entries[0].manufacturer.as_deref(), Some("FEB"));
    assert_eq!(alc.entries[0].alert_id.as_deref(), Some("01"));
    assert_eq!(alc.entries[0].instance, Some(2));
    assert_eq!(alc.entries[0].revision, Some(3));
    assert_eq!(alc.entries[1].manufacturer.as_deref(), Some("TEB"));
    assert_eq!(alc.entries[1].alert_id.as_deref(), Some("02"));
    assert_eq!(alc.entries[1].instance, Some(3));
    assert_eq!(alc.entries[1].revision, Some(4));
}

#[test]
fn dispatch() {
    let frame = parse_frame("$FBALC,02,01,03,02,FEB,01,02,03,TEB,02,03,04*5F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Alc(_)));
}

#[test]
fn roundtrip() {
    let original = Alc {
        num_frags: Some(1),
        frag_num: Some(1),
        msg_id: Some(3),
        entries_num: Some(1),
        entries: vec![AlcEntry {
            manufacturer: Some("FEB".into()),
            alert_id: Some("01".into()),
            instance: Some(2),
            revision: Some(3),
        }],
    };
    let sentence = original.to_sentence("FB").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    assert_eq!(Alc::parse(&frame.fields).expect("parse"), original);
}

#[test]
fn retains_entries_after_missing_or_partial_group_fields() {
    let alc =
        Alc::parse(&["1", "1", "0", "3", "", "bad", "x", "", "FEB", "01", "2"]).expect("parse");
    assert_eq!(alc.entries.len(), 2);
    assert_eq!(alc.entries[0].manufacturer, None);
    assert_eq!(alc.entries[1].manufacturer.as_deref(), Some("FEB"));
    assert_eq!(alc.entries[1].revision, None);
}
