#![cfg(feature = "mss")]

use nmea_0183_rs::nmea::sentences::Mss;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

// https://github.com/semuconsulting/pynmeagps/blob/master/tests/pygpsdata-nmea4.log
const FIXTURE: &str = "$GPMSS,55,27,318.0,100,*66";

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid MSS");
    let original = NmeaSentence::parse(&frame);
    let sentence = original.to_sentence("GP").expect("encode MSS variant");
    let frame = parse_frame(&sentence).expect("re-parse MSS");
    assert_eq!(NmeaSentence::parse(&frame), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid MSS");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Mss(_)));
}

#[test]
fn mss_values_pynmeagps() {
    let frame = parse_frame(FIXTURE).expect("valid MSS");
    let mss = Mss::parse(&frame.fields);
    assert!((mss.strength.expect("signal strength") - 55.0).abs() < 0.001);
    assert!((mss.snr.expect("SNR") - 27.0).abs() < 0.001);
    assert!((mss.frequency.expect("frequency") - 318.0).abs() < 0.001);
    assert_eq!(mss.beacon_bps, Some(100));
    assert_eq!(mss.channel, None);
}

#[test]
fn mss_values_trimble() {
    // https://help.fieldsystems.trimble.com/r750/nmea0183-messages-mss.htm
    let frame = parse_frame("$GPMSS,26.4,7.2,283.5,200,2*61").expect("valid Trimble MSS");
    let mss = Mss::parse(&frame.fields);
    assert!((mss.strength.expect("signal strength") - 26.4).abs() < 0.001);
    assert!((mss.snr.expect("SNR") - 7.2).abs() < 0.001);
    assert!((mss.frequency.expect("frequency") - 283.5).abs() < 0.001);
    assert_eq!(mss.beacon_bps, Some(200));
    assert_eq!(mss.channel, Some('2'));
}

#[test]
fn roundtrip() {
    let original = Mss {
        strength: Some(26.4),
        snr: Some(7.2),
        frequency: Some(283.5),
        beacon_bps: Some(200),
        channel: Some('2'),
    };
    let sentence = original.to_sentence("GP").expect("encode MSS");
    assert_eq!(sentence.trim(), "$GPMSS,26.4,7.2,283.5,200,2*61");
    let frame = parse_frame(&sentence).expect("re-parse MSS");
    assert_eq!(Mss::parse(&frame.fields), original);
}
