#![cfg(feature = "txt")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Txt;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPTXT,01,01,02,u-blox ag - www.u-blox.com*50").expect("valid");
    let txt = Txt::parse(&frame.fields).expect("parse");
    let sentence = txt.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let txt2 = Txt::parse(&frame2.fields).expect("parse");
    assert_eq!(txt, txt2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPTXT,01,01,02,u-blox ag - www.u-blox.com*50").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Txt(_)));
}

#[test]
fn roundtrip() {
    let original = Txt {
        num_msg: Some(1),
        msg_num: Some(1),
        msg_type: Some(2),
        text: Some("u-blox ag - www.u-blox.com".to_string()),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Txt::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn txt_values() {
    let frame = parse_frame("$GPTXT,01,01,02,u-blox ag - www.u-blox.com*50").expect("valid");
    let txt = Txt::parse(&frame.fields).expect("parse");
    assert_eq!(txt.num_msg, Some(1));
    assert_eq!(txt.msg_num, Some(1));
    assert_eq!(txt.msg_type, Some(2));
    assert_eq!(txt.text.as_deref(), Some("u-blox ag - www.u-blox.com"));
}
