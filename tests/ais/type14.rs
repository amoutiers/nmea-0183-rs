//! AIS Type 14 — Safety-Related Broadcast Message.
#![cfg(feature = "ais")]

use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

const FIX: &str = "!AIVDM,1,1,,A,>5?Per18=HB1U:1@E=B0m<L,0*53";

#[test]
fn type14_safety_broadcast_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame(FIX).expect("valid Type 14 frame");
    let msg = parser.decode(&frame).expect("Type 14 should decode");
    match msg {
        AisMessage::Safety(s) => {
            assert!(s.mmsi > 0, "MMSI should be non-zero");
            assert!(!s.text.is_empty(), "text should not be empty");
        }
        other => panic!("expected Safety, got {other:?}"),
    }
}

#[test]
fn type14_values() {
    let mut parser = AisParser::new();
    let frame = parse_frame(FIX).expect("valid Type 14 frame");
    let msg = parser.decode(&frame).expect("Type 14 should decode");
    match msg {
        AisMessage::Safety(s) => {
            assert_eq!(s.mmsi, 351809000);
            assert_eq!(s.text, "RCVD YR TEST MSG");
        }
        other => panic!("expected Safety, got {other:?}"),
    }
}
