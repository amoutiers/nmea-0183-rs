#![cfg(feature = "mwd")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Mwd;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIMWD,046.,T,046.,M,10.1,N,05.2,M*43").expect("valid");
    let mwd = Mwd::parse(&frame.fields).expect("parse");
    let sentence = mwd.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let mwd2 = Mwd::parse(&frame2.fields).expect("parse");
    assert_eq!(mwd, mwd2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIMWD,,,046.,M,10.1,N,05.2,M*0B").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Mwd(_)));
}

#[test]
fn roundtrip() {
    let original = Mwd {
        wind_dir_true: Some(270.0),
        wind_dir_mag: Some(268.5),
        wind_speed_kts: Some(12.4),
        wind_speed_ms: Some(6.4),
    };
    let sentence = original.to_sentence("WI").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Mwd::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn mwd_values() {
    // (a) value half
    let frame = parse_frame("$IIMWD,046.,T,046.,M,10.1,N,05.2,M*43").expect("valid");
    let x = Mwd::parse(&frame.fields).expect("parse");
    assert!((x.wind_dir_true.expect("wind_dir_true") - 46.0).abs() < 1e-2);
    assert!((x.wind_dir_mag.expect("wind_dir_mag") - 46.0).abs() < 1e-2);
    assert!((x.wind_speed_kts.expect("wind_speed_kts") - 10.1).abs() < 1e-2);
    assert!((x.wind_speed_ms.expect("wind_speed_ms") - 5.2).abs() < 1e-2);

    // (b) wire half — 046.→46, 05.2→5.2 via format!("{}", v)
    let s = x.to_sentence("II").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "IIMWD,46,T,46,M,10.1,N,5.2,M");
}
