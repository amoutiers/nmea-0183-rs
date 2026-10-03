#![cfg(feature = "wnc")]

use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Wnc;
use nmea_0183_rs::{NmeaSentence, parse_frame};

// Upstream marnav parser vector, also retained in external_fixtures/marnav/.
// https://github.com/mariokonrad/marnav/tree/5984da6fd102ce587b554979fcb68f02e8e8b2f9/test/marnav/nmea
const MARNAV_WNC: &str = "$GPWNC,12.3,N,5.6,K,POINT1,POINT2*78";

#[test]
fn decode_encode() {
    let frame = parse_frame(MARNAV_WNC).expect("valid marnav WNC");
    let original = NmeaSentence::parse(&frame);
    let sentence = original.to_sentence("GP").expect("encode WNC variant");
    let frame = parse_frame(&sentence).expect("re-parse WNC");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(MARNAV_WNC).expect("valid marnav WNC");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Wnc(_)));
}

#[test]
fn roundtrip() {
    let original = Wnc {
        distance_nm: Some(12.3),
        distance_km: Some(5.6),
        destination_waypoint: Some("POINT1".into()),
        origin_waypoint: Some("POINT2".into()),
    };
    let sentence = original.to_sentence("GP").expect("encode WNC");
    let frame = parse_frame(&sentence).expect("re-parse WNC");
    assert_eq!(Wnc::parse(&frame.fields), original);
}

#[test]
fn wnc_values() {
    let frame = parse_frame(MARNAV_WNC).expect("valid marnav WNC");
    let wnc = Wnc::parse(&frame.fields);
    assert!((wnc.distance_nm.expect("distance_nm") - 12.3).abs() < 1e-4);
    assert!((wnc.distance_km.expect("distance_km") - 5.6).abs() < 1e-4);
    assert_eq!(wnc.destination_waypoint.as_deref(), Some("POINT1"));
    assert_eq!(wnc.origin_waypoint.as_deref(), Some("POINT2"));

    let sentence = wnc.to_sentence("GP").expect("encode WNC");
    let (body, _) = sentence.split_once('*').expect("checksum");
    assert_eq!(body, "$GPWNC,12.3,N,5.6,K,POINT1,POINT2");
}
