#![cfg(feature = "rsd")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Rsd;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$RARSD,0.00,,2.50,005.0,0.00,,4.50,355.0,,,3.0,N,H*51").expect("valid");
    let rsd = Rsd::parse(&frame.fields).expect("parse");
    let sentence = rsd.to_sentence("RA").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rsd2 = Rsd::parse(&frame2.fields).expect("parse");
    assert_eq!(rsd, rsd2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$RARSD,0.00,,2.50,005.0,0.00,,4.50,355.0,,,3.0,N,H*51").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rsd(_)));
}

#[test]
fn roundtrip() {
    let original = Rsd {
        origin1_range: Some(0.0),
        origin1_bearing: None,
        vrm1: Some(2.5),
        bearing_line1: Some(5.0),
        origin2_range: Some(0.0),
        origin2_bearing: None,
        vrm2: Some(4.5),
        bearing_line2: Some(355.0),
        cursor_range: None,
        cursor_bearing: None,
        range_scale: Some(3.0),
        range_unit: Some('N'),
        display_rotation: Some('H'),
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rsd::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn rsd_values() {
    let frame = parse_frame("$RARSD,0.00,,2.50,005.0,0.00,,4.50,355.0,,,3.0,N,H*51")
        .expect("valid RSD frame");
    let x = Rsd::parse(&frame.fields).expect("parse RSD");
    assert!((x.origin1_range.expect("origin1_range") - 0.0).abs() < 1e-2);
    assert!(x.origin1_bearing.is_none());
    assert!((x.vrm1.expect("vrm1") - 2.5).abs() < 1e-2);
    assert!((x.bearing_line1.expect("bearing_line1") - 5.0).abs() < 1e-2);
    assert!((x.origin2_range.expect("origin2_range") - 0.0).abs() < 1e-2);
    assert!(x.origin2_bearing.is_none());
    assert!((x.vrm2.expect("vrm2") - 4.5).abs() < 1e-2);
    assert!((x.bearing_line2.expect("bearing_line2") - 355.0).abs() < 1e-2);
    assert!(x.cursor_range.is_none());
    assert!(x.cursor_bearing.is_none());
    assert!((x.range_scale.expect("range_scale") - 3.0).abs() < 1e-2);
    assert_eq!(x.range_unit, Some('N'));
    assert_eq!(x.display_rotation, Some('H'));
}
