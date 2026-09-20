//! AIS Type 18 — Class B Standard Position Report.
#![cfg(feature = "ais")]

use nmea_0183_rs::ais::{AisClass, AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type_18_class_b_position() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,B6CdCm0t3`tba35f@V9faHi7kP06,0*58")
        .expect("valid Type 18 frame");
    let msg = parser.decode(&frame);

    if let Some(AisMessage::Position(pos)) = &msg {
        assert_eq!(pos.ais_class, AisClass::B, "Type 18 should be Class B");
        assert!(pos.mmsi > 0, "MMSI should be non-zero");
    }
}

#[test]
fn type18_values() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,B6CdCm0t3`tba35f@V9faHi7kP06,0*58")
        .expect("valid Type 18 frame");
    let msg = parser.decode(&frame).expect("Type 18 should decode");
    match msg {
        AisMessage::Position(pos) => {
            assert_eq!(pos.mmsi, 423302100);
            let lat = pos.latitude.expect("latitude present");
            let lon = pos.longitude.expect("longitude present");
            assert!((lat - 40.00528333333333).abs() < 0.00001, "lat was {lat}");
            assert!((lon - 53.010996666666664).abs() < 0.00001, "lon was {lon}");
            let sog = pos.sog.expect("sog present");
            assert!((sog - 1.4).abs() < 0.1, "sog was {sog}");
        }
        other => panic!("expected Position, got {other:?}"),
    }
}
