#![cfg(feature = "alf")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Alf;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$VDALF,1,0,1,220516,B,A,S,SAL,001,1,2,0,My alarm*2C").expect("valid");
    let alf = Alf::parse(&frame.fields);
    let sentence = alf.to_sentence("VD").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let alf2 = Alf::parse(&frame2.fields);
    assert_eq!(alf, alf2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$VDALF,1,0,1,220516,B,A,S,SAL,001,1,2,0,My alarm*2C").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Alf(_)));
}

#[test]
fn alf_values() {
    let frame = parse_frame("$VDALF,1,0,1,220516,B,A,S,SAL,001,1,2,0,My alarm*2C").expect("valid");
    let alf = Alf::parse(&frame.fields);
    assert_eq!(alf.num_frags, Some(1));
    assert_eq!(alf.frag_num, Some(0));
    assert_eq!(alf.msg_id, Some(1));
    assert_eq!(alf.time.as_deref(), Some("220516"));
    assert_eq!(alf.category, Some('B'));
    assert_eq!(alf.priority, Some('A'));
    assert_eq!(alf.state, Some('S'));
    assert_eq!(alf.manufacturer.as_deref(), Some("SAL"));
    assert_eq!(alf.alert_id.as_deref(), Some("001"));
    assert_eq!(alf.instance, Some(1));
    assert_eq!(alf.revision, Some(2));
    assert_eq!(alf.escalation, Some(0));
    assert_eq!(alf.text.as_deref(), Some("My alarm"));
}

#[test]
fn roundtrip() {
    let original = Alf {
        num_frags: Some(1),
        frag_num: Some(0),
        msg_id: Some(1),
        time: Some("220516".to_string()),
        category: Some('B'),
        priority: Some('A'),
        state: Some('S'),
        manufacturer: Some("SAL".to_string()),
        alert_id: Some("001".to_string()),
        instance: Some(1),
        revision: Some(2),
        escalation: Some(0),
        text: Some("My alarm".to_string()),
    };
    let sentence = original.to_sentence("VD").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Alf::parse(&frame.fields);
    assert_eq!(original, parsed);
}
