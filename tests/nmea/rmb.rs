#![cfg(feature = "rmb")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Rmb;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$ECRMB,A,0.000,L,001,002,4653.550,N,07115.984,W,2.505,334.205,0.000,V*04")
            .expect("valid");
    let rmb = Rmb::parse(&frame.fields).expect("parse");
    let sentence = rmb.to_sentence("EC").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rmb2 = Rmb::parse(&frame2.fields).expect("parse");
    assert_eq!(rmb, rmb2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$ECRMB,A,0.000,L,001,002,4653.550,N,07115.984,W,2.505,334.205,0.000,V*04")
            .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rmb(_)));
}

#[test]
fn roundtrip() {
    let original = Rmb {
        status: Some('A'),
        ctrkerr: Some(0.5),
        dirs: Some('L'),
        wpt_origin: Some("001".to_string()),
        wpt_dest: Some("002".to_string()),
        dest_lat: Some(4653.55),
        ns: Some('N'),
        dest_lon: Some(7115.984),
        ew: Some('W'),
        range: Some(2.505),
        bearing: Some(334.205),
        velclos: Some(0.0),
        arrstatus: Some('V'),
        valstatus: Some('A'),
    };
    let sentence = original.to_sentence("EC").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rmb::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn rmb_values() {
    let frame =
        parse_frame("$ECRMB,A,0.000,L,001,002,4653.550,N,07115.984,W,2.505,334.205,0.000,V*04")
            .expect("valid");
    let r = Rmb::parse(&frame.fields).expect("parse");
    assert_eq!(r.status, Some('A'));
    assert!((r.ctrkerr.expect("ctrkerr") - 0.0).abs() < 1e-4);
    assert_eq!(r.dirs, Some('L'));
    assert_eq!(r.wpt_origin.as_deref(), Some("001"));
    assert_eq!(r.wpt_dest.as_deref(), Some("002"));
    // dest_lat is raw DDMM.MMMM (not decimal degrees)
    assert!((r.dest_lat.expect("dest_lat") - 4653.550).abs() < 1e-4);
    assert_eq!(r.ns, Some('N'));
    // dest_lon is raw DDDMM.MMM (not decimal degrees)
    assert!((r.dest_lon.expect("dest_lon") - 7115.984).abs() < 1e-4);
    assert_eq!(r.ew, Some('W'));
    assert!((r.range.expect("range") - 2.505).abs() < 1e-4);
    assert!((r.bearing.expect("bearing") - 334.205).abs() < 1e-4);
    assert!((r.velclos.expect("velclos") - 0.0).abs() < 1e-4);
    assert_eq!(r.arrstatus, Some('V'));
    assert!(r.valstatus.is_none());
}
