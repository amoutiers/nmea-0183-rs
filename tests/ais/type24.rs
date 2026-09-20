//! AIS Type 24 — Static Data Report (Class B).
#![cfg(feature = "ais")]

use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser, StaticDataReport};
use nmea_0183_rs::parse_frame;

#[test]
fn type_24_class_b_static_gpsd() {
    let mut parser = AisParser::new();

    let frame_a = parse_frame("!AIVDM,1,1,,A,H42O55i18tMET00000000000000,2*6D")
        .expect("valid Type 24 Part A");
    match expect_message(parser.decode(&frame_a).expect("Part A should decode")) {
        AisMessage::StaticReport(StaticDataReport::PartA {
            mmsi, vessel_name, ..
        }) => {
            assert!(mmsi > 0, "MMSI should be non-zero");
            assert!(!vessel_name.is_empty(), "vessel_name should not be empty");
        }
        other => panic!("expected StaticReport(PartA), got {other:?}"),
    }

    let frame_b = parse_frame("!AIVDM,1,1,,A,H42O55lti4hhhilD3nink000?050,0*40")
        .expect("valid Type 24 Part B");
    match expect_message(parser.decode(&frame_b).expect("Part B should decode")) {
        AisMessage::StaticReport(StaticDataReport::PartB { mmsi, .. }) => {
            assert!(mmsi > 0, "MMSI should be non-zero");
        }
        other => panic!("expected StaticReport(PartB), got {other:?}"),
    }
}

#[test]
fn type_24_part_b_values_gpsd() {
    let mut parser = AisParser::new();
    let frame_b = parse_frame("!AIVDM,1,1,,A,H42O55lti4hhhilD3nink000?050,0*40")
        .expect("valid Type 24 Part B");
    match expect_message(parser.decode(&frame_b).expect("Part B should decode")) {
        AisMessage::StaticReport(StaticDataReport::PartB {
            mmsi,
            callsign,
            ship_type,
            ..
        }) => {
            assert_eq!(mmsi, 271041815);
            assert_eq!(ship_type, 60, "ship_type @40");
            assert_eq!(callsign, "TC6163", "callsign @90");
        }
        other => panic!("expected StaticReport(PartB), got {other:?}"),
    }
}
