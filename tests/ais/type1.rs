//! AIS Types 1/2/3 — Class A Position Report.
#![cfg(feature = "ais")]

use super::expect_message;
use nmea_0183_rs::ais::{AisClass, AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type_1_single_fragment_class_a_signalk() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").expect("valid Type 1 frame");
    let msg = expect_message(parser.decode(&frame).expect("Type 1 should decode"));

    match msg {
        AisMessage::Position(pos) => {
            assert_eq!(pos.msg_type, 1);
            assert!(pos.mmsi > 0);
            assert_eq!(pos.ais_class, AisClass::A);
            let lat = pos.latitude.expect("latitude present");
            let lon = pos.longitude.expect("longitude present");
            assert!((-90.0..=90.0).contains(&lat), "latitude {lat} out of range");
            assert!(
                (-180.0..=180.0).contains(&lon),
                "longitude {lon} out of range"
            );
        }
        other => panic!("expected Position, got {other:?}"),
    }
}

#[test]
fn sentinel_filtering_signalk() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").expect("valid Type 1 frame");
    let msg = expect_message(parser.decode(&frame).expect("Type 1 should decode"));

    if let AisMessage::Position(pos) = msg {
        if let Some(lat) = pos.latitude {
            assert!((-90.0..=90.0).contains(&lat), "lat {lat} out of range");
        }
        if let Some(lon) = pos.longitude {
            assert!((-180.0..=180.0).contains(&lon), "lon {lon} out of range");
        }
        if let Some(hdg) = pos.heading {
            assert!(hdg < 360, "heading {hdg} should be < 360");
        }
    }
}

#[test]
fn type1_values() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").expect("valid Type 1 frame");
    let msg = expect_message(parser.decode(&frame).expect("Type 1 should decode"));
    match msg {
        AisMessage::Position(pos) => {
            assert_eq!(pos.msg_type, 1);
            assert_eq!(pos.mmsi, 244670316);
            let lat = pos.latitude.expect("latitude present");
            let lon = pos.longitude.expect("longitude present");
            assert!((lat - 51.89475).abs() < 0.00001, "lat was {lat}");
            assert!((lon - 4.379285).abs() < 0.00001, "lon was {lon}");
            let cog = pos.cog.expect("cog present");
            assert!((cog - 70.6).abs() < 0.1, "cog was {cog}");
        }
        other => panic!("expected Position, got {other:?}"),
    }
}
