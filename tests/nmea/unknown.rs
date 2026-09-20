#![cfg(feature = "nmea")]

use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn dispatch_unknown() {
    let frame = parse_frame("$GPXYZ,1,2,3").expect("valid");
    match NmeaSentence::parse(&frame) {
        NmeaSentence::Unknown {
            sentence_type,
            fields,
        } => {
            assert_eq!(sentence_type, "XYZ");
            assert_eq!(fields, vec!["1", "2", "3"]);
        }
        other => panic!("expected Unknown, got {other:?}"),
    }
}

#[test]
fn unsupported_becomes_unknown() {
    let frame = parse_frame("$GPZZZ,1,2,3").expect("valid");
    match NmeaSentence::parse(&frame) {
        NmeaSentence::Unknown { sentence_type, .. } => assert_eq!(sentence_type, "ZZZ"),
        other => panic!("expected Unknown for ZZZ, got {other:?}"),
    }
}
