#![cfg(feature = "ttd")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Ttd;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let original = Ttd {
        num_frags: Some("01".to_string()),
        frag_num: Some("01".to_string()),
        msg_id: Some(1),
        payload: Some("trackdata".to_string()),
        fill_bits: Some(0),
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("valid");
    let ttd = Ttd::parse(&frame.fields).expect("parse");
    let sentence2 = ttd.to_sentence("RA").expect("encode");
    let frame2 = parse_frame(sentence2.trim()).expect("re-parse");
    let ttd2 = Ttd::parse(&frame2.fields).expect("parse");
    assert_eq!(ttd, ttd2);
}

#[test]
fn dispatch() {
    let original = Ttd {
        num_frags: Some("01".to_string()),
        frag_num: Some("01".to_string()),
        msg_id: Some(1),
        payload: Some("test".to_string()),
        fill_bits: Some(0),
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Ttd(_)));
}

#[test]
fn roundtrip() {
    let original = Ttd {
        num_frags: Some("01".to_string()),
        frag_num: Some("01".to_string()),
        msg_id: Some(1),
        payload: Some("trackdata".to_string()),
        fill_bits: Some(0),
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Ttd::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn ttd_values() {
    let original = Ttd {
        num_frags: Some("1A".to_string()),
        frag_num: Some("01".to_string()),
        msg_id: Some(1),
        payload: Some("trackdata".to_string()),
        fill_bits: Some(0),
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("valid");
    let t = Ttd::parse(&frame.fields).expect("parse");
    assert_eq!(t, original);
}
