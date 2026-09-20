#![cfg(feature = "tlb")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::{Tlb, TlbTarget};
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$RATLB,1,XXX*20").expect("valid");
    let tlb = Tlb::parse(&frame.fields);
    let sentence = tlb.to_sentence("RA").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let tlb2 = Tlb::parse(&frame2.fields);
    assert_eq!(tlb, tlb2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$RATLB,1,XXX*20").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Tlb(_)));
}

#[test]
fn roundtrip() {
    let original = Tlb {
        targets: vec![
            TlbTarget {
                number: Some(1),
                label: Some("ALPHA".to_string()),
            },
            TlbTarget {
                number: Some(2),
                label: Some("BETA".to_string()),
            },
        ],
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Tlb::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn retains_targets_after_missing_number() {
    let tlb = Tlb::parse(&["", "unknown", "2", "BETA", "3"]);
    assert_eq!(tlb.targets.len(), 3);
    assert_eq!(tlb.targets[0].number, None);
    assert_eq!(tlb.targets[1].label.as_deref(), Some("BETA"));
    assert_eq!(tlb.targets[2].label, None);
}
