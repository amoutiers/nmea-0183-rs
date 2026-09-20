#![cfg(feature = "apb")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Apb;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPAPB,A,A,0.10,R,N,V,V,011,M,DEST,011,M,011,M*3C").expect("valid");
    let apb = Apb::parse(&frame.fields).expect("parse");
    let sentence = apb.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let apb2 = Apb::parse(&frame2.fields).expect("parse");
    assert_eq!(apb, apb2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPAPB,A,A,0.10,R,N,V,V,011,M,DEST,011,M,011,M*3C").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Apb(_)));
}

#[test]
fn roundtrip() {
    let original = Apb {
        lcgwarn: Some('A'),
        lccwarn: Some('A'),
        ctrkerr: Some(0.1),
        dirs: Some('R'),
        ctrkunit: Some('N'),
        aalmcirc: Some('V'),
        aalmperp: Some('V'),
        bear_o2d: Some(11.0),
        bear_o2d_type: Some('M'),
        wpt: Some("DEST".to_string()),
        bear_dest: Some(11.0),
        bear_dest_type: Some('M'),
        bear_steer: Some(11.0),
        bear_steer_type: Some('M'),
        mode: Some('A'),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Apb::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn apb_values() {
    let frame = parse_frame("$GPAPB,A,A,0.10,R,N,V,V,011,M,DEST,011,M,011,M*3C").expect("valid");
    let a = Apb::parse(&frame.fields).expect("parse");
    assert_eq!(a.lcgwarn, Some('A'));
    assert_eq!(a.lccwarn, Some('A'));
    assert!((a.ctrkerr.expect("ctrkerr") - 0.10).abs() < 1e-4);
    assert_eq!(a.dirs, Some('R'));
    assert_eq!(a.ctrkunit, Some('N'));
    assert_eq!(a.aalmcirc, Some('V'));
    assert_eq!(a.aalmperp, Some('V'));
    assert!((a.bear_o2d.expect("bear_o2d") - 11.0).abs() < 1e-4);
    assert_eq!(a.bear_o2d_type, Some('M'));
    assert_eq!(a.wpt.as_deref(), Some("DEST"));
    assert!((a.bear_dest.expect("bear_dest") - 11.0).abs() < 1e-4);
    assert_eq!(a.bear_dest_type, Some('M'));
    assert!((a.bear_steer.expect("bear_steer") - 11.0).abs() < 1e-4);
    assert_eq!(a.bear_steer_type, Some('M'));
    assert!(a.mode.is_none());
}
