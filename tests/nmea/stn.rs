#![cfg(feature = "stn")]

use nmea_0183_rs::nmea::sentences::Stn;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

// Fixture from pynmeagps, retained in the repository's external_fixtures inventory.
// https://github.com/semuconsulting/pynmeagps/tree/master/tests
const FIXTURE: &str = "$GPSTN,34*75";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid STN fixture");
    let original = NmeaSentence::parse(&frame);
    let sentence = original.to_sentence("GP").expect("encode STN variant");
    let frame = parse_frame(&sentence).expect("re-parse STN");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid STN fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Stn(_)));
}

#[test]
fn roundtrip() {
    let original = Stn {
        talker_id: Some("07".into()),
    };
    let sentence = original.to_sentence("GP").expect("encode STN");
    let frame = parse_frame(&sentence).expect("re-parse STN");
    assert_eq!(Stn::parse(&frame.fields), original);
}
