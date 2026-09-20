#![cfg(feature = "pgrme")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Pgrme;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$PGRME,3.3,M,4.9,M,6.0,M*25").expect("valid");
    let pgrme = Pgrme::parse(&frame.fields).expect("parse");
    let sentence = pgrme.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let pgrme2 = Pgrme::parse(&frame2.fields).expect("parse");
    assert_eq!(pgrme, pgrme2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$PGRME,3.3,M,4.9,M,6.0,M*25").expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Pgrme(_)
    ));
}

#[test]
fn roundtrip() {
    let original = Pgrme {
        horizontal: Some(3.3),
        vertical: Some(4.9),
        spherical: Some(6.0),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pgrme::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn pgrme_values() {
    // (a) value half
    let frame = parse_frame("$PGRME,3.3,M,4.9,M,6.0,M*25").expect("valid");
    let x = Pgrme::parse(&frame.fields).expect("parse");
    assert!((x.horizontal.expect("horizontal") - 3.3).abs() < 1e-2);
    assert!((x.vertical.expect("vertical") - 4.9).abs() < 1e-2);
    assert!((x.spherical.expect("spherical") - 6.0).abs() < 1e-2);

    // (b) wire half — proprietary: to_sentence("") body starts with PGRME,
    // normalization: 6.0→6 via format!("{}", v)
    let s = x.to_sentence("").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "PGRME,3.3,M,4.9,M,6,M");
}
