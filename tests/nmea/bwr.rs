#![cfg(feature = "bwr")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Bwr;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPBWR,225444,4917.24,N,12309.57,W,051.9,T,031.6,M,001.3,N,004*38")
        .expect("valid");
    let bwr = Bwr::parse(&frame.fields).expect("parse");
    let sentence = bwr.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let bwr2 = Bwr::parse(&frame2.fields).expect("parse");
    assert_eq!(bwr, bwr2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPBWR,225444,4917.24,N,12309.57,W,051.9,T,031.6,M,001.3,N,004*38")
        .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Bwr(_)));
}

#[test]
fn roundtrip() {
    let original = Bwr {
        time: Some("225444".to_string()),
        lat: Some(4917.24),
        ns: Some('N'),
        lon: Some(12309.57),
        ew: Some('W'),
        bear_true: Some(51.9),
        bear_mag: Some(31.6),
        dist: Some(1.3),
        wpt: Some("004".to_string()),
        mode: None,
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Bwr::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn bwr_values() {
    let frame = parse_frame("$GPBWR,225444,4917.24,N,12309.57,W,051.9,T,031.6,M,001.3,N,004*38")
        .expect("valid");
    let x = Bwr::parse(&frame.fields).expect("parse");
    // (a) value half
    assert_eq!(x.time.as_deref(), Some("225444"));
    assert!((x.lat.expect("lat") - 4917.24).abs() < 1e-2);
    assert_eq!(x.ns, Some('N'));
    assert!((x.lon.expect("lon") - 12309.57).abs() < 1e-2);
    assert_eq!(x.ew, Some('W'));
    assert!((x.bear_true.expect("bear_true") - 51.9).abs() < 1e-2);
    assert!((x.bear_mag.expect("bear_mag") - 31.6).abs() < 1e-2);
    assert!((x.dist.expect("dist") - 1.3).abs() < 1e-2);
    assert_eq!(x.wpt.as_deref(), Some("004"));
    assert!(x.mode.is_none());
    // (b) wire half
    let s = x.to_sentence("GP").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(
        body,
        "GPBWR,225444,4917.24,N,12309.57,W,51.9,T,31.6,M,1.3,N,004,"
    );
}
