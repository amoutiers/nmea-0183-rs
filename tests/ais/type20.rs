//! Synthetic Type 20 layout probes, src/ais/messages/data_link_management.rs.
#![cfg(feature = "ais")]
use super::{ais_frame_from_bits, expect_message, set_ais_bits};
use nmea_0183_rs::{
    ais::{AisMessage, AisParser, messages::SlotReservation},
    parse_frame,
};

#[test]
fn type20_reservations_are_exact() {
    let expected = [
        SlotReservation {
            offset: 101,
            slots: 2,
            timeout: 3,
            increment: 501,
        },
        SlotReservation {
            offset: 202,
            slots: 4,
            timeout: 5,
            increment: 1001,
        },
        SlotReservation {
            offset: 303,
            slots: 6,
            timeout: 7,
            increment: 1501,
        },
        SlotReservation {
            offset: 404,
            slots: 8,
            timeout: 1,
            increment: 2001,
        },
    ];
    let fifth = SlotReservation {
        offset: 555,
        slots: 9,
        timeout: 2,
        increment: 17,
    };
    for (length, count) in [(72, 1), (160, 4), (189, 4), (190, 4), (196, 4)] {
        let mut bits = vec![0; length];
        set_ais_bits(&mut bits, 0, 6, 20);
        set_ais_bits(&mut bits, 8, 30, 123456789);
        for (index, entry) in expected
            .iter()
            .chain(std::iter::once(&fifth))
            .take(((length - 40) / 30).min(5))
            .enumerate()
        {
            let start = 40 + index * 30;
            for (offset, width, value) in [
                (0, 12, u32::from(entry.offset)),
                (12, 4, u32::from(entry.slots)),
                (16, 3, u32::from(entry.timeout)),
                (19, 11, u32::from(entry.increment)),
            ] {
                set_ais_bits(&mut bits, start + offset, width, value);
            }
        }
        let wire = ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::DataLinkManagement(message) => {
                assert_eq!(message.mmsi, 123456789);
                assert_eq!(message.reservations, expected[..count]);
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
