#![cfg(feature = "arc")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Arc;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn arc_values() {
    let frame = parse_frame("$RAARC,220516,TCK,002,1,A*73").expect("valid");
    let arc = Arc::parse(&frame.fields);

    assert_eq!(arc.time.as_deref(), Some("220516"));
    assert_eq!(arc.manufacturer.as_deref(), Some("TCK"));
    assert_eq!(arc.alert_id.as_deref(), Some("002"));
    assert_eq!(arc.instance, Some(1));
    assert_eq!(arc.command, Some('A'));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$RAARC,220516,TCK,002,1,A*73").expect("valid");
    let arc = Arc::parse(&frame.fields);
    let sentence = arc.to_sentence("RA").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let arc2 = Arc::parse(&frame2.fields);
    assert_eq!(arc, arc2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$RAARC,220516,TCK,002,1,A*73").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Arc(_)));
}

#[test]
fn roundtrip() {
    let original = Arc {
        time: Some("220516".to_string()),
        manufacturer: Some("TCK".to_string()),
        alert_id: Some("002".to_string()),
        instance: Some(1),
        command: Some('A'),
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Arc::parse(&frame.fields);
    assert_eq!(original, parsed);
}
