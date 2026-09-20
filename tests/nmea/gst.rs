#![cfg(feature = "gst")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Gst;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPGST,131519.00,11,,,,0.70,0.49,1.1*53").expect("valid");
    let gst = Gst::parse(&frame.fields).expect("parse");
    let sentence = gst.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gst2 = Gst::parse(&frame2.fields).expect("parse");
    assert_eq!(gst, gst2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPGST,131519.00,11,,,,0.70,0.49,1.1*53").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gst(_)));
}

#[test]
fn roundtrip() {
    let original = Gst {
        time: Some("131519.00".to_string()),
        range_rms: Some(11.0),
        std_major: Some(5.2),
        std_minor: Some(3.1),
        orient: Some(45.0),
        std_lat: Some(0.7),
        std_lon: Some(0.49),
        std_alt: Some(1.1),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gst::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn gst_values() {
    let frame = parse_frame("$GPGST,131519.00,11,,,,0.70,0.49,1.1*53").expect("valid GST fixture");
    let gst = Gst::parse(&frame.fields).expect("parse GST");
    assert_eq!(gst.time.as_deref(), Some("131519.00"));
    assert!((gst.range_rms.expect("range_rms") - 11.0_f32).abs() < 1e-4);
    assert!(gst.std_major.is_none());
    assert!(gst.std_minor.is_none());
    assert!(gst.orient.is_none());
    assert!((gst.std_lat.expect("std_lat") - 0.70_f32).abs() < 1e-4);
    assert!((gst.std_lon.expect("std_lon") - 0.49_f32).abs() < 1e-4);
    assert!((gst.std_alt.expect("std_alt") - 1.1_f32).abs() < 1e-4);
}
