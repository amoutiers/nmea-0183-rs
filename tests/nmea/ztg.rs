#![cfg(feature = "ztg")]

use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Ztg;
use nmea_0183_rs::{NmeaSentence, parse_frame};

// Upstream marnav parser vector, also retained in external_fixtures/marnav/.
// https://github.com/mariokonrad/marnav/tree/5984da6fd102ce587b554979fcb68f02e8e8b2f9/test/marnav/nmea
const MARNAV_ZTG: &str = "$GPZTG,123456.1,000010,POINT1*16";

#[test]
fn decode_encode() {
    let frame = parse_frame(MARNAV_ZTG).expect("valid marnav ZTG");
    let original = NmeaSentence::parse(&frame);
    let sentence = original.to_sentence("GP").expect("encode ZTG variant");
    let frame = parse_frame(&sentence).expect("re-parse ZTG");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(MARNAV_ZTG).expect("valid marnav ZTG");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Ztg(_)));
}

#[test]
fn roundtrip() {
    let original = Ztg {
        observation_time: Some("000001.25".into()),
        time_to_go: Some("990010.5".into()),
        destination_waypoint: Some("DEST".into()),
    };
    let sentence = original.to_sentence("GP").expect("encode ZTG");
    let frame = parse_frame(&sentence).expect("re-parse ZTG");
    assert_eq!(Ztg::parse(&frame.fields), original);
}

#[test]
fn ztg_values() {
    let frame = parse_frame(MARNAV_ZTG).expect("valid marnav ZTG");
    let ztg = Ztg::parse(&frame.fields);
    assert_eq!(ztg.observation_time.as_deref(), Some("123456.1"));
    assert_eq!(ztg.time_to_go.as_deref(), Some("000010"));
    assert_eq!(ztg.destination_waypoint.as_deref(), Some("POINT1"));
}
