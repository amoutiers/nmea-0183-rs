#![cfg(feature = "ack")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Ack;
use nmea_0183_rs::parse_frame;

#[test]
fn decode_encode() {
    let frame = parse_frame("$VRACK,001*50").expect("valid");
    let ack = Ack::parse(&frame.fields).expect("parse");
    let sentence = ack.to_sentence("VR").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let ack2 = Ack::parse(&frame2.fields).expect("parse");
    assert_eq!(ack, ack2);
}

#[test]
fn roundtrip() {
    let original = Ack {
        alert_id: Some("001".to_string()),
    };
    let sentence = original.to_sentence("VR").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Ack::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn ack_values() {
    let frame = parse_frame("$VRACK,001*50").expect("valid");
    let a = Ack::parse(&frame.fields).expect("parse");
    assert_eq!(a.alert_id.as_deref(), Some("001"));
}
