#![cfg(feature = "bec")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Bec;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPBEC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*33")
        .expect("valid");
    let bec = Bec::parse(&frame.fields);
    let sentence = bec.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let bec2 = Bec::parse(&frame2.fields);
    assert_eq!(bec, bec2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPBEC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*33")
        .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Bec(_)));
}

#[test]
fn roundtrip() {
    let original = Bec {
        time: Some("220516".to_string()),
        lat: Some(5130.02),
        ns: Some('N'),
        lon: Some(46.34),
        ew: Some('W'),
        bear_true: Some(213.8),
        bear_mag: Some(218.0),
        dist: Some(4.6),
        wpt: Some("EGLM".to_string()),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Bec::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn bec_values() {
    let frame = parse_frame("$GPBEC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*33")
        .expect("valid");
    let x = Bec::parse(&frame.fields);
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
    // (b) wire half
    let s = x.to_sentence("GP").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(
        body,
        "GPBEC,220516,5130.02,N,00046.34,W,213.8,T,218,M,4.6,N,EGLM"
    );
}
