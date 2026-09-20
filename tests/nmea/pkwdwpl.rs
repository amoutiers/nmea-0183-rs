#![cfg(feature = "pkwdwpl")]
use nmea_kit::nmea::NmeaEncodable;

use nmea_kit::nmea::sentences::Pkwdwpl;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$PKWDWPL,150803,A,4237.14,N,07120.83,W,173.8,231.8,190316,1120,test,/'*39")
            .expect("valid");
    let p = Pkwdwpl::parse(&frame.fields).expect("parse");
    let sentence = p.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let p2 = Pkwdwpl::parse(&frame2.fields).expect("parse");
    assert_eq!(p, p2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$PKWDWPL,150803,A,4237.14,N,07120.83,W,173.8,231.8,190316,1120,test,/'*39")
            .expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Pkwdwpl(_)
    ));
}

#[test]
fn roundtrip() {
    let original = Pkwdwpl {
        time: Some("150803".to_string()),
        validity: Some('A'),
        lat: Some(4237.14),
        ns: Some('N'),
        lon: Some(7120.83),
        ew: Some('W'),
        speed: Some(173.8),
        course: Some(231.8),
        date: Some("190316".to_string()),
        altitude: Some(1120.0),
        wpt_name: Some("test".to_string()),
        table_symbol: Some("/'".to_string()),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pkwdwpl::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
