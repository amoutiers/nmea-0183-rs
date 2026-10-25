#![cfg(feature = "xtr")]

use nmea_0183_rs::nmea::sentences::Xtr;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

// Fixture from marnav, retained in the repository's external_fixtures inventory.
// https://github.com/mariokonrad/marnav/tree/5984da6fd102ce587b554979fcb68f02e8e8b2f9/test/marnav/nmea
const FIXTURE: &str = "$GPXTR,,,*65";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid XTR fixture");
    let original = NmeaSentence::parse(&frame);
    let sentence = original.to_sentence("GP").expect("encode XTR variant");
    let frame = parse_frame(&sentence).expect("re-parse XTR");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid XTR fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Xtr(_)));
}

#[test]
fn roundtrip() {
    let original = Xtr {
        cross_track_error: Some(0.125),
        steer_direction: Some('R'),
        unit: Some('N'),
    };
    let sentence = original.to_sentence("GP").expect("encode XTR");
    let frame = parse_frame(&sentence).expect("re-parse XTR");
    assert_eq!(Xtr::parse(&frame.fields), original);
}
