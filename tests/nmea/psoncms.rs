#![cfg(feature = "psoncms")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Psoncms;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame(
        "$PSONCMS,0.0905,0.4217,0.9020,-0.0196,-1.7685,0.3861,-9.6648,-0.0116,0.0065,-0.0080,0.0581,0.3846,0.7421,33.1*76",
    )
    .expect("valid");
    let p = Psoncms::parse(&frame.fields).expect("parse");
    let sentence = p.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let p2 = Psoncms::parse(&frame2.fields).expect("parse");
    assert_eq!(p, p2);
}

#[test]
fn dispatch() {
    let frame = parse_frame(
        "$PSONCMS,0.0905,0.4217,0.9020,-0.0196,-1.7685,0.3861,-9.6648,-0.0116,0.0065,-0.0080,0.0581,0.3846,0.7421,33.1*76",
    )
    .expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Psoncms(_)
    ));
}

#[test]
fn roundtrip() {
    let original = Psoncms {
        quaternion_0: Some(0.0905),
        quaternion_1: Some(0.4217),
        quaternion_2: Some(0.9020),
        quaternion_3: Some(-0.0196),
        accel_x: Some(-1.7685),
        accel_y: Some(0.3861),
        accel_z: Some(-9.6648),
        rot_x: Some(-0.0116),
        rot_y: Some(0.0065),
        rot_z: Some(-0.0080),
        mag_x: Some(0.0581),
        mag_y: Some(0.3846),
        mag_z: Some(0.7421),
        temperature: Some(33.1),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Psoncms::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
