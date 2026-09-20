#![cfg(feature = "wpl")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Wpl;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn dispatch() {
    let frame = parse_frame("$IIWPL,5503.4530,N,01037.2742,E,411*6F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Wpl(_)));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIWPL,5503.4530,N,01037.2742,E,411*6F").expect("valid");
    let wpl = Wpl::parse(&frame.fields);
    let sentence = wpl.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let wpl2 = Wpl::parse(&frame2.fields);
    assert_eq!(wpl, wpl2);
}

#[test]
fn roundtrip() {
    let original = Wpl {
        lat: Some(5503.453),
        ns: Some('N'),
        lon: Some(1037.2742),
        ew: Some('E'),
        ident: Some("411".to_string()),
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Wpl::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn wpl_values() {
    let frame = parse_frame("$IIWPL,5503.4530,N,01037.2742,E,411*6F").expect("valid");
    let wpl = Wpl::parse(&frame.fields);
    assert!((wpl.lat.expect("lat") - 5503.453_f64).abs() < 1e-2);
    assert_eq!(wpl.ns, Some('N'));
    assert!((wpl.lon.expect("lon") - 1037.2742_f64).abs() < 1e-2);
    assert_eq!(wpl.ew, Some('E'));
    assert_eq!(wpl.ident.as_deref(), Some("411"));
}
