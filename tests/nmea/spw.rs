#![cfg(feature = "spw")]

use nmea_0183_rs::nmea::sentences::Spw;
use nmea_0183_rs::{NmeaEncodable, NmeaSentence, parse_frame};

#[test]
fn spw_values_emtrak() {
    // https://em-trak.com/wp-content/uploads/A100-user-manual-EN.pdf
    let frame = parse_frame("$IISPW,EPV,211000001,2,SESAME*1A").expect("valid SPW");
    assert_eq!(
        Spw::parse(&frame.fields),
        Spw {
            protected_sentence: Some("EPV".into()),
            identifier: Some("211000001".into()),
            password_level: Some(2),
            password: Some("SESAME".into())
        }
    );
}

#[test]
fn spw_empty() {
    assert_eq!(Spw::parse(&[]), Spw::default());
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IISPW,EPV,211000001,2,SESAME*1A").expect("fixture");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Spw(_)));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("$IISPW,EPV,211000001,2,SESAME*1A").expect("fixture");
    let original = NmeaSentence::parse(&frame);
    let line = original.to_sentence("II").expect("encode");
    assert_eq!(
        NmeaSentence::parse(&parse_frame(&line).expect("decode")),
        original
    );
}

#[test]
fn spw_encode_roundtrip() {
    let frame = parse_frame("$IISPW,EPV,211000001,2,SESAME*1A").expect("fixture");
    let original = Spw::parse(&frame.fields);
    let line = original.to_sentence("II").expect("encode");
    assert_eq!(
        Spw::parse(&parse_frame(&line).expect("decode").fields),
        original
    );
}
