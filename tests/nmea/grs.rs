#![cfg(feature = "grs")]

use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Grs;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GNGRS,103607.00,1,-2.1,0.2,2.7,-0.4,,,,,,,,,1,1*53").expect("valid");
    let grs = Grs::parse(&frame.fields);
    let sentence = grs.to_sentence("GN").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    assert_eq!(Grs::parse(&frame.fields), grs);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GNGRS,103607.00,1,-2.1,0.2,2.7,-0.4,,,,,,,,,1,1*53").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Grs(_)));
}

#[test]
fn grs_values() {
    let frame = parse_frame("$GNGRS,103607.00,1,-2.1,0.2,2.7,-0.4,,,,,,,,,1,1*53")
        .expect("valid pynmeagps GRS fixture");
    let grs = Grs::parse(&frame.fields);
    assert_eq!(grs.time.as_deref(), Some("103607.00"));
    assert_eq!(grs.mode, Some(1));
    for (actual, expected) in grs.residuals.iter().zip([-2.1_f32, 0.2, 2.7, -0.4]) {
        assert!((actual.expect("residual") - expected).abs() < 1e-4);
    }
    assert!(grs.residuals[4..].iter().all(Option::is_none));
    assert_eq!(grs.system_id, Some('1'));
    assert_eq!(grs.signal_id, Some('1'));
}

#[test]
fn roundtrip() {
    let original = Grs {
        time: Some("103607.00".to_string()),
        mode: Some(1),
        residuals: [
            Some(-2.1),
            Some(0.2),
            Some(2.7),
            Some(-0.4),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        system_id: Some('1'),
        signal_id: Some('1'),
    };
    let sentence = original.to_sentence("GN").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    assert_eq!(Grs::parse(&frame.fields), original);
}
