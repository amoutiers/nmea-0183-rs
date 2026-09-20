//! AIS Type 19 — Class B Extended Position Report.
#![cfg(feature = "ais")]

use super::expect_message;
use nmea_0183_rs::ais::{AisClass, AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type_19_class_b_extended_gpsd() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,B,C5N3SRgPEnJGEBT>NhWAwwo862PaLELTBJ:V00000000S0D:R220,0*0B")
            .expect("valid Type 19 frame");
    let msg = expect_message(parser.decode(&frame).expect("Type 19 should decode"));

    match msg {
        AisMessage::Position(pos) => {
            assert_eq!(pos.msg_type, 19);
            assert_eq!(pos.ais_class, AisClass::BPlus, "Type 19 should be Class B+");
            assert!(pos.mmsi > 0);
        }
        other => panic!("expected Position, got {other:?}"),
    }
}

#[test]
fn type19_values() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,B,C5N3SRgPEnJGEBT>NhWAwwo862PaLELTBJ:V00000000S0D:R220,0*0B")
            .expect("valid Type 19 frame");
    let msg = expect_message(parser.decode(&frame).expect("Type 19 should decode"));
    match msg {
        AisMessage::Position(pos) => {
            assert_eq!(pos.mmsi, 367059850);
            let lat = pos.latitude.expect("latitude present");
            let lon = pos.longitude.expect("longitude present");
            assert!((lat - 29.543695).abs() < 0.00001, "lat was {lat}");
            assert!(
                (lon - (-88.81039166666666)).abs() < 0.00001,
                "lon was {lon}"
            );
            let sog = pos.sog.expect("sog present");
            assert!((sog - 8.7).abs() < 0.1, "sog was {sog}");
        }
        other => panic!("expected Position, got {other:?}"),
    }
}
