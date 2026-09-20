#![cfg(feature = "alr")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Alr;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn alr_values() {
    let frame = parse_frame("$RAALR,220516,001,A,A,Bilge pump alarm1*4C").expect("valid");
    let alr = Alr::parse(&frame.fields);

    assert_eq!(alr.time.as_deref(), Some("220516"));
    assert_eq!(alr.alarm_id.as_deref(), Some("001"));
    assert_eq!(alr.condition, Some('A'));
    assert_eq!(alr.state, Some('A'));
    assert_eq!(alr.description.as_deref(), Some("Bilge pump alarm1"));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$RAALR,220516,001,A,A,Bilge pump alarm1*4C").expect("valid");
    let alr = Alr::parse(&frame.fields);
    let sentence = alr.to_sentence("RA").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let alr2 = Alr::parse(&frame2.fields);
    assert_eq!(alr, alr2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$RAALR,220516,001,A,A,Bilge pump alarm1*4C").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Alr(_)));
}

#[test]
fn roundtrip() {
    let original = Alr {
        time: Some("220516".to_string()),
        alarm_id: Some("001".to_string()),
        condition: Some('A'),
        state: Some('A'),
        description: Some("Bilge pump alarm1".to_string()),
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Alr::parse(&frame.fields);
    assert_eq!(original, parsed);
}
