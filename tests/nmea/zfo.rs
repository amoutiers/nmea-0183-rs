#![cfg(feature = "zfo")]

use nmea_0183_rs::nmea::sentences::Zfo;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

// Fixture from marnav, retained in the repository's external_fixtures inventory.
// https://github.com/mariokonrad/marnav/tree/5984da6fd102ce587b554979fcb68f02e8e8b2f9/test/marnav/nmea
const FIXTURE: &str = "$GPZFO,123456.1,000010,POINT1*0C";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid ZFO fixture");
    let original = NmeaSentence::parse(&frame);
    let sentence = original.to_sentence("GP").expect("encode ZFO variant");
    let frame = parse_frame(&sentence).expect("re-parse ZFO");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid ZFO fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Zfo(_)));
}

#[test]
fn roundtrip() {
    let original = Zfo {
        observation_time: Some("235959.99".into()),
        elapsed_time: Some("990001.25".into()),
        origin_waypoint: Some("ORIGIN".into()),
    };
    let sentence = original.to_sentence("GP").expect("encode ZFO");
    let frame = parse_frame(&sentence).expect("re-parse ZFO");
    assert_eq!(Zfo::parse(&frame.fields), original);
}
