#![cfg(feature = "bwc")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Bwc;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPBWC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*21")
        .expect("valid");
    let bwc = Bwc::parse(&frame.fields);
    let sentence = bwc.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let bwc2 = Bwc::parse(&frame2.fields);
    assert_eq!(bwc, bwc2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPBWC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*21")
        .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Bwc(_)));
}

#[test]
fn roundtrip() {
    let original = Bwc {
        time: Some("225444".to_string()),
        lat: Some(4917.24),
        ns: Some('N'),
        lon: Some(12309.57),
        ew: Some('W'),
        bear_true: Some(51.9),
        bear_mag: Some(31.6),
        dist: Some(1.3),
        wpt: Some("004".to_string()),
        mode: Some('A'),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Bwc::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn bwc_values() {
    let frame = parse_frame("$GPBWC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*21")
        .expect("valid");
    let x = Bwc::parse(&frame.fields);
    // (a) value half
    assert_eq!(x.time.as_deref(), Some("220516"));
    assert!((x.lat.expect("lat") - 5130.02).abs() < 1e-2);
    assert_eq!(x.ns, Some('N'));
    assert!((x.lon.expect("lon") - 46.34).abs() < 1e-2);
    assert_eq!(x.ew, Some('W'));
    assert!((x.bear_true.expect("bear_true") - 213.8).abs() < 1e-2);
    assert!((x.bear_mag.expect("bear_mag") - 218.0).abs() < 1e-2);
    assert!((x.dist.expect("dist") - 4.6).abs() < 1e-2);
    assert_eq!(x.wpt.as_deref(), Some("EGLM"));
    assert!(x.mode.is_none());
    // (b) wire half
    let s = x.to_sentence("GP").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(
        body,
        "GPBWC,220516,5130.02,N,00046.34,W,213.8,T,218,M,4.6,N,EGLM,"
    );
}
