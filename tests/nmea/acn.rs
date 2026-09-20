#![cfg(feature = "acn")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Acn;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn acn_values() {
    let frame = parse_frame("$RAACN,220516,TCK,002,1,A,C*00").expect("valid");
    let acn = Acn::parse(&frame.fields);

    assert_eq!(acn.time.as_deref(), Some("220516"));
    assert_eq!(acn.manufacturer.as_deref(), Some("TCK"));
    assert_eq!(acn.alert_id.as_deref(), Some("002"));
    assert_eq!(acn.instance, Some(1));
    assert_eq!(acn.command, Some('A'));
    assert_eq!(acn.state, Some('C'));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$RAACN,220516,TCK,002,1,A,C*00").expect("valid");
    let acn = Acn::parse(&frame.fields);
    let sentence = acn.to_sentence("RA").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let acn2 = Acn::parse(&frame2.fields);
    assert_eq!(acn, acn2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$RAACN,220516,TCK,002,1,A,C*00").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Acn(_)));
}

#[test]
fn roundtrip() {
    let original = Acn {
        time: Some("220516".to_string()),
        manufacturer: Some("TCK".to_string()),
        alert_id: Some("002".to_string()),
        instance: Some(1),
        command: Some('A'),
        state: Some('C'),
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Acn::parse(&frame.fields);
    assert_eq!(original, parsed);
}
