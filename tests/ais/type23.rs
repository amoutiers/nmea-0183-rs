//! Synthetic Type 23 zones, signed 1/600 degree layout from group_assignment.rs.
#![cfg(feature = "ais")]
use super::{ais_frame_from_bits, expect_message, set_ais_bits};
use nmea_0183_rs::{
    ais::{AisMessage, AisParser, messages::GroupAssignment},
    parse_frame,
};

fn decode_zone(raw: [i32; 4]) -> GroupAssignment {
    let mut bits = vec![0; 160];
    set_ais_bits(&mut bits, 0, 6, 23);
    for ((offset, width), value) in [(40, 18), (58, 17), (75, 18), (93, 17)]
        .into_iter()
        .zip(raw)
    {
        set_ais_bits(
            &mut bits,
            offset,
            width,
            (value as u32) & ((1 << width) - 1),
        );
    }
    let wire = ais_frame_from_bits(&bits);
    match expect_message(
        AisParser::new()
            .decode(&parse_frame(&wire).expect("frame"))
            .expect("decode"),
    ) {
        AisMessage::GroupAssignment(message) => message,
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn type23_zone_coordinates_are_exact() {
    let message = decode_zone([-1201, 1801, -2401, -3001]);
    for (actual, expected) in [
        (message.northeast_longitude, -1201.0 / 600.0),
        (message.northeast_latitude, 1801.0 / 600.0),
        (message.southwest_longitude, -2401.0 / 600.0),
        (message.southwest_latitude, -3001.0 / 600.0),
    ] {
        assert!((actual.expect("available coordinate") - expected).abs() < 1e-10);
    }
}

#[test]
fn type23_zone_coordinate_boundaries() {
    let message = decode_zone([108000, 54000, -108000, -54000]);
    assert_eq!(
        (
            message.northeast_longitude,
            message.northeast_latitude,
            message.southwest_longitude,
            message.southwest_latitude
        ),
        (Some(180.0), Some(90.0), Some(-180.0), Some(-90.0))
    );
    let message = decode_zone([108600, 54600, -1200, -1800]);
    assert_eq!(
        (
            message.northeast_longitude,
            message.northeast_latitude,
            message.southwest_longitude,
            message.southwest_latitude
        ),
        (None, None, Some(-2.0), Some(-3.0))
    );
}
