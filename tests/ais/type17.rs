//! Synthetic signed-coordinate probes; Type 17 field layout is documented locally.
#![cfg(feature = "ais")]
use super::{ais_frame_from_bits, expect_message, set_ais_bits};
use nmea_0183_rs::{
    ais::{AisMessage, AisParser},
    parse_frame,
};

#[test]
fn type17_longitude_values_and_sentinel() {
    for (raw, expected) in [
        (0, Some(0.0)),
        (-3001, Some(-3001.0 / 600.0)),
        (108600, None),
    ] {
        let mut bits = vec![0; 80];
        set_ais_bits(&mut bits, 0, 6, 17);
        set_ais_bits(&mut bits, 8, 30, 123456789);
        set_ais_bits(&mut bits, 40, 18, (raw as u32) & 0x3ffff);
        set_ais_bits(&mut bits, 58, 17, 1201);
        let wire = ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::DgnssBroadcast(message) => {
                assert_eq!(message.mmsi, 123456789);
                match expected {
                    Some(value) => assert!(
                        (message.longitude.expect("available longitude") - value).abs() < 1e-10
                    ),
                    None => assert_eq!(message.longitude, None),
                }
                assert!((message.latitude.expect("latitude") - 1201.0 / 600.0).abs() < 1e-10);
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
