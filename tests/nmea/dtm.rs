#![cfg(feature = "dtm")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Dtm;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPDTM,W84,,0.0,N,0.0,E,0.0,W84*6F").expect("valid");
    let dtm = Dtm::parse(&frame.fields);
    let sentence = dtm.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let dtm2 = Dtm::parse(&frame2.fields);
    assert_eq!(dtm, dtm2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPDTM,W84,,0.0,N,0.0,E,0.0,W84*6F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Dtm(_)));
}

#[test]
fn roundtrip() {
    let original = Dtm {
        datum: Some("W84".to_string()),
        sub_datum: None,
        lat_offset: Some(0.0),
        ns: Some('N'),
        lon_offset: Some(0.0),
        ew: Some('E'),
        alt_offset: Some(0.0),
        ref_datum: Some("W84".to_string()),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Dtm::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn dtm_values() {
    let frame = parse_frame("$GPDTM,W84,,0.0,N,0.0,E,0.0,W84*6F").expect("valid DTM fixture");
    let dtm = Dtm::parse(&frame.fields);
    assert_eq!(dtm.datum.as_deref(), Some("W84"));
    assert!(dtm.sub_datum.is_none());
    assert!((dtm.lat_offset.expect("lat_offset") - 0.0_f32).abs() < 1e-4);
    assert_eq!(dtm.ns, Some('N'));
    assert!((dtm.lon_offset.expect("lon_offset") - 0.0_f32).abs() < 1e-4);
    assert_eq!(dtm.ew, Some('E'));
    assert!((dtm.alt_offset.expect("alt_offset") - 0.0_f32).abs() < 1e-4);
    assert_eq!(dtm.ref_datum.as_deref(), Some("W84"));
}
