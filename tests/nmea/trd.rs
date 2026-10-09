#![cfg(feature = "trd")]

use nmea_0183_rs::nmea::{NmeaEncodable, sentences::Trd};
use nmea_0183_rs::{NmeaSentence, parse_frame};

const FIXTURE: &str = "$RCTRD,2,400.5,R,,V,90.8*55";
// Navis NavDP 4000 Operation Manual, TRD message.
// https://alphatronmarine.com/files/secured/docuware_documents/144-DP%2BNavis%2BDP4000%2BOperation%2BManual%2B%2B1-5-2018.pdf

fn assert_close(actual: Option<f32>, expected: f32) {
    assert!((actual.expect("value present") - expected).abs() < 0.001);
}

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid TRD frame");
    let original = Trd::parse(&frame.fields);
    let sentence = original.to_sentence("RC").expect("encode TRD");
    let encoded = parse_frame(sentence.trim()).expect("re-parse TRD");
    assert_eq!(Trd::parse(&encoded.fields), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid TRD frame");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Trd(_)));
}

#[test]
fn trd_values_navis() {
    let frame = parse_frame(FIXTURE).expect("valid Navis DP4000 TRD frame");
    let trd = Trd::parse(&frame.fields);

    assert_eq!(trd.thruster_number.as_deref(), Some("2"));
    assert_close(trd.rpm_response, 400.5);
    assert_eq!(trd.rpm_mode, Some('R'));
    assert!(trd.pitch_response.is_none());
    assert_eq!(trd.pitch_mode, Some('V'));
    assert_close(trd.azimuth_response, 90.8);
}

#[test]
fn roundtrip() {
    let original = Trd {
        thruster_number: Some("2".into()),
        rpm_response: Some(400.5),
        rpm_mode: Some('R'),
        pitch_response: None,
        pitch_mode: Some('V'),
        azimuth_response: Some(90.8),
    };

    let sentence = original.to_sentence("RC").expect("encode TRD");
    let frame = parse_frame(sentence.trim()).expect("re-parse TRD");
    assert_eq!(Trd::parse(&frame.fields), original);
}
