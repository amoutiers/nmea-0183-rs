#![cfg(feature = "xte")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Xte;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPXTE,A,A,0.67,L,N*6F").expect("valid");
    let xte = Xte::parse(&frame.fields).expect("parse");
    let sentence = xte.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let xte2 = Xte::parse(&frame2.fields).expect("parse");
    assert_eq!(xte, xte2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPXTE,A,A,0.67,L,N*6F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Xte(_)));
}

#[test]
fn roundtrip() {
    let original = Xte {
        gwarn: Some('A'),
        lccwarn: Some('A'),
        ctrkerr: Some(0.67),
        dirs: Some('L'),
        disunit: Some('N'),
        mode: Some('D'),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Xte::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn xte_values() {
    let frame = parse_frame("$GPXTE,A,A,0.67,L,N*6F").expect("valid");
    let xte = Xte::parse(&frame.fields).expect("parse");
    assert_eq!(xte.gwarn, Some('A'));
    assert_eq!(xte.lccwarn, Some('A'));
    assert!((xte.ctrkerr.expect("ctrkerr") - 0.67).abs() < 1e-2);
    assert_eq!(xte.dirs, Some('L'));
    assert_eq!(xte.disunit, Some('N'));
    assert!(xte.mode.is_none());
}
