//! AIS Type 21 — Aid-to-Navigation Report.
#![cfg(feature = "ais")]

use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type_21_aid_to_navigation_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,B,E>jCfrv2`0c2h0W:0a0h6220d5Du0`Htp00000l1@Dc2P0,4*3C")
        .expect("valid Type 21 frame");
    let msg = expect_message(parser.decode(&frame).expect("Type 21 should decode"));

    match msg {
        AisMessage::AidToNavigation(aton) => {
            assert!(aton.mmsi > 0, "MMSI should be non-zero");
            assert!(
                aton.aid_type <= 31,
                "aid_type must be 0–31, got {}",
                aton.aid_type
            );
            if let (Some(lat), Some(lon)) = (aton.lat, aton.lon) {
                assert!((-90.0..=90.0).contains(&lat), "lat out of range: {lat}");
                assert!((-180.0..=180.0).contains(&lon), "lon out of range: {lon}");
            }
        }
        other => panic!("expected AidToNavigation, got {other:?}"),
    }
}

#[test]
fn type21_values() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,B,E>jCfrv2`0c2h0W:0a0h6220d5Du0`Htp00000l1@Dc2P0,4*3C")
        .expect("valid Type 21 frame");
    let msg = expect_message(parser.decode(&frame).expect("Type 21 should decode"));
    match msg {
        AisMessage::AidToNavigation(a) => {
            assert_eq!(a.mmsi, 992276203);
            assert_eq!(a.name, "EPAVE ANTARA LDDAXJ)");
            assert_eq!(a.aid_type, 28);
            let lat = a.lat.expect("latitude present");
            let lon = a.lon.expect("longitude present");
            assert!(
                (lat - (-27.962026666666667)).abs() < 0.00001,
                "lat was {lat}"
            );
            assert!(
                (lon - (-83.61035333333334)).abs() < 0.00001,
                "lon was {lon}"
            );
        }
        other => panic!("expected AidToNavigation, got {other:?}"),
    }
}
