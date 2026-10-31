#![cfg(feature = "fsi")]

use nmea_0183_rs::nmea::sentences::Fsi;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

// https://github.com/mariokonrad/marnav/blob/master/test/marnav/nmea/Test_nmea_fsi.cpp
const FIXTURE: &str = "$GPFSI,156000,156025,,,*60";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid FSI");
    let original = NmeaSentence::parse(&frame);
    let sentence = original.to_sentence("GP").expect("encode FSI variant");
    let frame = parse_frame(&sentence).expect("re-parse FSI");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid FSI");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Fsi(_)));
}

#[test]
fn fsi_values_marnav() {
    let frame = parse_frame(FIXTURE).expect("valid FSI");
    let fsi = Fsi::parse(&frame.fields);
    assert_eq!(fsi.transmit_frequency.as_deref(), Some("156000"));
    assert_eq!(fsi.receive_frequency.as_deref(), Some("156025"));
    assert_eq!(fsi.operating_mode, None);
    assert_eq!(fsi.power, None);
    assert_eq!(fsi.status, None);
}

#[test]
fn roundtrip() {
    let original = Fsi {
        transmit_frequency: Some("156000".into()),
        receive_frequency: Some("156025".into()),
        operating_mode: Some('F'),
        power: Some(9),
        status: Some('R'),
    };
    let sentence = original.to_sentence("GP").expect("encode FSI");
    assert_eq!(
        sentence.split('*').next(),
        Some("$GPFSI,156000,156025,F,9,R")
    );
    let frame = parse_frame(&sentence).expect("re-parse FSI");
    assert_eq!(Fsi::parse(&frame.fields), original);
}
