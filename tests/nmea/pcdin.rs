#![cfg(feature = "pcdin")]
use nmea_kit::nmea::NmeaEncodable;
use nmea_kit::nmea::sentences::Pcdin;
use nmea_kit::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$PCDIN,01F112,000C72EA,09,28C36A0000B40AFD*56").expect("valid");
    let pcdin = Pcdin::parse(&frame.fields).expect("parse");
    let sentence = pcdin.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let pcdin2 = Pcdin::parse(&frame2.fields).expect("parse");
    assert_eq!(pcdin, pcdin2);
}

#[test]
fn roundtrip() {
    let original = Pcdin {
        pgn: Some("01F112".to_string()),
        timestamp: Some("000C72EA".to_string()),
        source: Some("09".to_string()),
        data: Some("28C36A0000B40AFD".to_string()),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pcdin::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$PCDIN,01F112,000C72EA,09,28C36A0000B40AFD*56").expect("valid");
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Pcdin(_)
    ));
}

#[test]
fn pcdin_values() {
    let frame = parse_frame("$PCDIN,01F112,000C72EA,09,28C36A0000B40AFD*56").expect("valid");
    let p = Pcdin::parse(&frame.fields).expect("parse");
    assert_eq!(p.pgn.as_deref(), Some("01F112"));
    assert_eq!(p.timestamp.as_deref(), Some("000C72EA"));
    assert_eq!(p.source.as_deref(), Some("09"));
    assert_eq!(p.data.as_deref(), Some("28C36A0000B40AFD"));
}
