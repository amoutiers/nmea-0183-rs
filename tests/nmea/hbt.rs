#![cfg(feature = "hbt")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Hbt;
use nmea_0183_rs::parse_frame;

#[test]
fn decode_encode() {
    let frame = parse_frame("$HCHBT,1.5,A,1*23").expect("valid");
    let hbt = Hbt::parse(&frame.fields).expect("parse");
    let sentence = hbt.to_sentence("HC").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let hbt2 = Hbt::parse(&frame2.fields).expect("parse");
    assert_eq!(hbt, hbt2);
}

#[test]
fn roundtrip() {
    let original = Hbt {
        interval: Some(1.5),
        operation_status: Some('A'),
        msg_id: Some(1),
    };
    let sentence = original.to_sentence("HC").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Hbt::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn hbt_values() {
    let frame = parse_frame("$HCHBT,1.5,A,1*23").expect("valid HBT fixture");
    let hbt = Hbt::parse(&frame.fields).expect("parse HBT");
    assert!((hbt.interval.expect("interval") - 1.5_f32).abs() < 1e-4);
    assert_eq!(hbt.operation_status, Some('A'));
    assert_eq!(hbt.msg_id, Some(1));
}
