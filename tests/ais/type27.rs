//! AIS Type 27 — Long Range Position Report.
#![cfg(feature = "ais")]

use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type_27_long_range_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,KCQ9r=hrFUnH7P00,0*41").expect("valid Type 27 frame");
    let msg = expect_message(parser.decode(&frame).expect("Type 27 should decode"));

    match msg {
        AisMessage::LongRangePosition(pos) => {
            assert!(pos.mmsi > 0, "MMSI should be non-zero");
            if let (Some(lat), Some(lon)) = (pos.latitude, pos.longitude) {
                assert!((-90.0..=90.0).contains(&lat), "latitude {lat} out of range");
                assert!(
                    (-180.0..=180.0).contains(&lon),
                    "longitude {lon} out of range"
                );
            }
        }
        other => panic!("expected LongRangePosition, got {other:?}"),
    }
}

#[test]
fn type_27_latlon_scaling_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,KCQ9r=hrFUnH7P00,0*41").expect("valid Type 27 frame");
    match expect_message(parser.decode(&frame).expect("Type 27 should decode")) {
        AisMessage::LongRangePosition(pos) => {
            let lat = pos.latitude.expect("latitude present after /600 fix");
            let lon = pos.longitude.expect("longitude present after /600 fix");
            assert!((lat - 52239.0 / 600.0).abs() < 1e-10, "lat={lat}");
            assert!((lon - (-92521.0 / 600.0)).abs() < 1e-10, "lon={lon}");
        }
        other => panic!("expected LongRangePosition, got {other:?}"),
    }
}

#[test]
fn type27_unavailable_coordinates() {
    match expect_message(
        AisParser::new()
            .decode(&parse_frame("!AIVDM,1,1,,A,K1mg=5Cn`>6bTOwv,0*63").expect("frame"))
            .expect("decode"),
    ) {
        AisMessage::LongRangePosition(m) => {
            assert_eq!(m.longitude, None);
            assert_eq!(m.latitude, None);
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn type27_coordinate_precision() {
    match expect_message(
        AisParser::new()
            .decode(&parse_frame("!AIVDM,1,1,,A,KCQ9r=hrFUnH7P00,0*41").expect("frame"))
            .expect("decode"),
    ) {
        AisMessage::LongRangePosition(m) => {
            assert!((m.longitude.expect("lon") - (-92521.0 / 600.0)).abs() < 1e-10);
            assert!((m.latitude.expect("lat") - 52239.0 / 600.0).abs() < 1e-10);
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn type27_speed_sentinel() {
    for (raw, expected) in [(0, Some(0)), (62, Some(62)), (63, None)] {
        let mut bits = vec![0; 96];
        super::set_ais_bits(&mut bits, 0, 6, 27);
        super::set_ais_bits(&mut bits, 79, 6, raw);
        let wire = super::ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::LongRangePosition(message) => assert_eq!(message.sog, expected),
            other => panic!("unexpected {other:?}"),
        }
    }
}

// Synthetic one-hot probes for the public flags at their documented bit offsets.
#[test]
fn type27_flags_are_exact() {
    let kind = 27;
    for (set_bit, expected) in [
        (None, [false, false]),
        (Some(38), [true, false]),
        (Some(39), [false, true]),
    ] {
        let mut bits = vec![0; 96];
        super::set_ais_bits(&mut bits, 0, 6, kind);
        if let Some(bit) = set_bit {
            super::set_ais_bits(&mut bits, bit, 1, 1);
        }
        let wire = super::ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::LongRangePosition(message) => assert_eq!(
                [message.position_accuracy, message.raim],
                expected,
                "type {kind}, bit {set_bit:?}"
            ),
            other => panic!("unexpected {other:?}"),
        }
    }
}
