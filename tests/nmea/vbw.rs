#![cfg(feature = "vbw")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Vbw;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPVBW,12.3,0.07,A,11.78,0.12,A*6F").expect("valid");
    let vbw = Vbw::parse(&frame.fields);
    let sentence = vbw.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vbw2 = Vbw::parse(&frame2.fields);
    assert_eq!(vbw, vbw2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPVBW,12.3,0.07,A,11.78,0.12,A*6F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vbw(_)));
}

#[test]
fn roundtrip() {
    let original = Vbw {
        long_water_spd: Some(5.2),
        trans_water_spd: Some(0.1),
        water_spd_status: Some('A'),
        long_ground_spd: Some(5.3),
        trans_ground_spd: Some(0.2),
        ground_spd_status: Some('A'),
        stern_trans_water_spd: None,
        stern_water_spd_status: None,
        stern_trans_ground_spd: None,
        stern_ground_spd_status: None,
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vbw::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn vbw_values() {
    let frame = parse_frame("$GPVBW,12.3,0.07,A,11.78,0.12,A*6F").expect("valid");
    let vbw = Vbw::parse(&frame.fields);
    assert!((vbw.long_water_spd.expect("long_water_spd") - 12.3).abs() < 1e-2);
    assert!((vbw.trans_water_spd.expect("trans_water_spd") - 0.07).abs() < 1e-2);
    assert_eq!(vbw.water_spd_status, Some('A'));
    assert!((vbw.long_ground_spd.expect("long_ground_spd") - 11.78).abs() < 1e-2);
    assert!((vbw.trans_ground_spd.expect("trans_ground_spd") - 0.12).abs() < 1e-2);
    assert_eq!(vbw.ground_spd_status, Some('A'));
    assert!(vbw.stern_trans_water_spd.is_none());
    assert!(vbw.stern_water_spd_status.is_none());
    assert!(vbw.stern_trans_ground_spd.is_none());
    assert!(vbw.stern_ground_spd_status.is_none());
}
