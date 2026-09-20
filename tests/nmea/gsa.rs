#![cfg(feature = "gsa")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Gsa;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39").expect("valid");
    let gsa = Gsa::parse(&frame.fields).expect("parse");
    let sentence = gsa.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gsa2 = Gsa::parse(&frame2.fields).expect("parse");
    assert_eq!(gsa, gsa2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gsa(_)));
}

#[test]
fn roundtrip() {
    let original = Gsa {
        mode: Some('A'),
        fix_type: Some(3),
        prns: [
            Some(4),
            Some(5),
            None,
            Some(9),
            Some(12),
            None,
            None,
            Some(24),
            None,
            None,
            None,
            None,
        ],
        pdop: Some(2.5),
        hdop: Some(1.3),
        vdop: Some(2.1),
        system_id: None,
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gsa::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn gsa_values() {
    // Wire: $GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39
    // PRN slots (0-based): 04(0) 05(1) _(2) 09(3) 12(4) _(5) _(6) 24(7) _(8) _(9) _(10) _(11)
    let frame =
        parse_frame("$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39").expect("valid GSA fixture");
    let gsa = Gsa::parse(&frame.fields).expect("parse GSA");
    assert_eq!(gsa.mode, Some('A'));
    assert_eq!(gsa.fix_type, Some(3));
    assert_eq!(gsa.prns[0], Some(4));
    assert_eq!(gsa.prns[1], Some(5));
    assert!(gsa.prns[2].is_none());
    assert_eq!(gsa.prns[3], Some(9));
    assert_eq!(gsa.prns[4], Some(12));
    assert!(gsa.prns[5].is_none());
    assert!(gsa.prns[6].is_none());
    assert_eq!(gsa.prns[7], Some(24));
    assert!(gsa.prns[8].is_none());
    assert!(gsa.prns[9].is_none());
    assert!(gsa.prns[10].is_none());
    assert!(gsa.prns[11].is_none());
    assert!((gsa.pdop.expect("pdop") - 2.5_f32).abs() < 1e-4);
    assert!((gsa.hdop.expect("hdop") - 1.3_f32).abs() < 1e-4);
    assert!((gsa.vdop.expect("vdop") - 2.1_f32).abs() < 1e-4);
    assert!(gsa.system_id.is_none());
}
