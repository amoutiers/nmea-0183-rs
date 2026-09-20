//! AIS Type 4 — Base Station Report.
#![cfg(feature = "ais")]

use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type_4_base_station_gpsd() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,403OviQuMGCqWrRO9>E6fE700@GO,0*4D").expect("valid Type 4 frame");
    let msg = parser.decode(&frame).expect("Type 4 should decode");

    match msg {
        AisMessage::BaseStation(report) => {
            assert!(report.mmsi > 0, "MMSI should be non-zero");
            if let (Some(lat), Some(lon)) = (report.latitude, report.longitude) {
                assert!((-90.0..=90.0).contains(&lat), "latitude {lat} out of range");
                assert!(
                    (-180.0..=180.0).contains(&lon),
                    "longitude {lon} out of range"
                );
            }
            if let Some(h) = report.hour {
                assert!(h < 24, "hour sentinel not filtered: {h}");
            }
            if let Some(m) = report.minute {
                assert!(m < 60, "minute sentinel not filtered: {m}");
            }
        }
        other => panic!("expected BaseStation, got {other:?}"),
    }
}

#[test]
fn type4_values() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,403OviQuMGCqWrRO9>E6fE700@GO,0*4D").expect("valid Type 4 frame");
    let msg = parser.decode(&frame).expect("Type 4 should decode");
    match msg {
        AisMessage::BaseStation(r) => {
            assert_eq!(r.mmsi, 3669702);
            assert_eq!(r.year, Some(2007));
            assert_eq!(r.month, Some(5));
            let lat = r.latitude.expect("latitude present");
            let lon = r.longitude.expect("longitude present");
            assert!((lat - 36.883766666666666).abs() < 0.00001, "lat was {lat}");
            assert!(
                (lon - (-76.35236166666667)).abs() < 0.00001,
                "lon was {lon}"
            );
        }
        other => panic!("expected BaseStation, got {other:?}"),
    }
}
