#![cfg(feature = "zdl")]

use nmea_0183_rs::nmea::sentences::Zdl;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

// Fixture from marnav, retained in the repository's external_fixtures inventory.
// https://github.com/mariokonrad/marnav/tree/5984da6fd102ce587b554979fcb68f02e8e8b2f9/test/marnav/nmea
const FIXTURE: &str = "$GPZDL,383401,12.3,R*28";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid ZDL fixture");
    let original = NmeaSentence::parse(&frame);
    let sentence = original.to_sentence("GP").expect("encode ZDL variant");
    let frame = parse_frame(&sentence).expect("re-parse ZDL");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid ZDL fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Zdl(_)));
}

#[test]
fn roundtrip() {
    let original = Zdl {
        time: Some("990001.25".into()),
        distance: Some(0.125),
        point_type: Some('R'),
    };
    let sentence = original.to_sentence("GP").expect("encode ZDL");
    let frame = parse_frame(&sentence).expect("re-parse ZDL");
    assert_eq!(Zdl::parse(&frame.fields), original);
}
