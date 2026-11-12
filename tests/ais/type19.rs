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

// Synthetic one-hot probes for the public flags at their documented bit offsets.
#[test]
fn type19_flags_are_exact() {
    let kind = 19;
    for (set_bit, expected) in [(None, [false]), (Some(56), [true])] {
        let mut bits = vec![0; 312];
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
            AisMessage::Position(message) => assert_eq!(
                [message.position_accuracy],
                expected,
                "type {kind}, bit {set_bit:?}"
            ),
            other => panic!("unexpected {other:?}"),
        }
    }
}
