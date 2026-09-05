#![cfg(feature = "pgrmt")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Pgrmt;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$PGRMT,GPS24xd-HVS VER 2.30,,,,,,,,*10").expect("valid");
    let pgrmt = Pgrmt::parse(&frame.fields).expect("parse");
    let sentence = pgrmt.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let pgrmt2 = Pgrmt::parse(&frame2.fields).expect("parse");
    assert_eq!(pgrmt, pgrmt2);
}

#[test]
fn roundtrip() {
    let original = Pgrmt {
        product_info: Some("GPS24xd-HVS".to_string()),
        rom_checksum: Some('P'),
        receiver_failure: Some('P'),
        stored_data: Some('R'),
        rtc_lost: Some('R'),
        osc_drift: Some('P'),
        data_collection: Some('C'),
        sensor_temp: Some("25".to_string()),
        sensor_config: Some('S'),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pgrmt::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$PGRMT,GPS24xd-HVS VER 2.30,,,,,,,,*10").expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Pgrmt(_)
    ));
}

#[test]
fn pgrmt_values() {
    let frame = parse_frame("$PGRMT,GPS24xd-HVS VER 2.30,P,P,R,R,P,C,25,S*57").expect("valid");
    let p = Pgrmt::parse(&frame.fields).expect("parse");
    assert_eq!(p.product_info.as_deref(), Some("GPS24xd-HVS VER 2.30"));
    assert_eq!(p.rom_checksum, Some('P'));
    assert_eq!(p.receiver_failure, Some('P'));
    assert_eq!(p.stored_data, Some('R'));
    assert_eq!(p.rtc_lost, Some('R'));
    assert_eq!(p.osc_drift, Some('P'));
    assert_eq!(p.data_collection, Some('C'));
    assert_eq!(p.sensor_temp.as_deref(), Some("25"));
    assert_eq!(p.sensor_config, Some('S'));
}
