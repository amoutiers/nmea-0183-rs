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

// Synthetic raw ROT probes; values are indicators, not degrees/minute.
#[test]
fn type1_rot_available_and_unavailable() {
    for (raw, expected) in [
        (-128, None),
        (-10, Some(-10.0)),
        (0, Some(0.0)),
        (10, Some(10.0)),
    ] {
        let mut bits = vec![0; 168];
        super::set_ais_bits(&mut bits, 0, 6, 1);
        super::set_ais_bits(&mut bits, 42, 8, (raw as u32) & 0xff);
        let wire = super::ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::Position(message) => assert_eq!(message.rate_of_turn, expected),
            other => panic!("unexpected {other:?}"),
        }
    }
}

// Synthetic one-hot probes for the public flags at their documented bit offsets.
#[test]
fn type1_flags_are_exact() {
    for kind in [1, 2, 3] {
        for (set_bit, expected) in [(None, [false]), (Some(60), [true])] {
            let mut bits = vec![0; 168];
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
}

// Synthetic suffix probes for the accepted minimum-length contract.
#[test]
fn type1_trailing_bits_preserve_report() {
    let decode = |bits: &[u8]| {
        let wire = super::ais_frame_from_bits(bits);
        nmea_0183_rs::ais::AisParser::new()
            .decode(&nmea_0183_rs::parse_frame(&wire).expect("frame"))
    };
    for kind in [1, 2, 3] {
        let mut bits: Vec<u8> = (0..168).map(|index| u8::from(index % 3 == 0)).collect();
        super::set_ais_bits(&mut bits, 0, 6, kind);
        super::set_ais_bits(&mut bits, 6, 2, 2);
        super::set_ais_bits(&mut bits, 8, 30, 123456789);
        let expected = super::expect_message(decode(&bits).expect("168 bits"));
        for length in [169, 174, 192] {
            let mut extended = bits.clone();
            extended.extend((168..length).map(|index| u8::from(index % 2 == 0)));
            assert_eq!(
                super::expect_message(decode(&extended).expect("suffix accepted")),
                expected
            );
            if let nmea_0183_rs::ais::AisMessage::Position(report) = &expected {
                assert_eq!(
                    nmea_0183_rs::ais::PositionReport::decode_class_a(&extended).as_ref(),
                    Some(report)
                );
            } else {
                panic!("expected position report");
            }
        }
        assert!(decode(&bits[..167]).is_err());
        assert!(nmea_0183_rs::ais::PositionReport::decode_class_a(&bits[..167]).is_none());
    }
}
