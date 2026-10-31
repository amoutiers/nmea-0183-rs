#![cfg(feature = "msk")]

use nmea_0183_rs::nmea::sentences::Msk;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

// https://github.com/semuconsulting/pynmeagps/blob/master/tests/pygpsdata-nmea4.log
const FIXTURE: &str = "$GPMSK,318.0,A,100,M,2*45";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid MSK");
    let original = NmeaSentence::parse(&frame);
    let sentence = original.to_sentence("GP").expect("encode MSK variant");
    let frame = parse_frame(&sentence).expect("re-parse MSK");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid MSK");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Msk(_)));
}

#[test]
fn msk_values_pynmeagps() {
    let frame = parse_frame(FIXTURE).expect("valid MSK");
    let msk = Msk::parse(&frame.fields);
    assert!((msk.frequency.expect("frequency") - 318.0).abs() < 0.001);
    assert_eq!(msk.frequency_mode, Some('A'));
    assert_eq!(msk.beacon_bps, Some(100));
    assert_eq!(msk.bps_mode, Some('M'));
    assert!((msk.mss_interval.expect("MSS interval") - 2.0).abs() < 0.001);
    assert_eq!(msk.channel, None);
    assert_eq!(msk.status, None);
}

#[test]
fn roundtrip() {
    let original = Msk {
        frequency: Some(318.5),
        frequency_mode: Some('M'),
        beacon_bps: Some(200),
        bps_mode: Some('A'),
        mss_interval: Some(2.5),
        channel: Some('2'),
        status: Some('R'),
    };
    let sentence = original.to_sentence("GP").expect("encode MSK");
    assert_eq!(
        sentence.split('*').next(),
        Some("$GPMSK,318.5,M,200,A,2.5,2,R")
    );
    let frame = parse_frame(&sentence).expect("re-parse MSK");
    assert_eq!(Msk::parse(&frame.fields), original);
}
