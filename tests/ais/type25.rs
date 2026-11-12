#![cfg(feature = "ais")]
use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;
// Synthetic addressed probe: ITU-R M.1371-5 tables 80 and 82.
// Reserved bits after destination and before radio state are intentional.

#[test]
fn type25_addressed_payload() {
    match expect_message(
        AisParser::new()
            .decode(&parse_frame("!AIVDM,1,1,,A,I1mg=5KcNJ;4dP,4*4F").expect("frame"))
            .expect("decode"),
    ) {
        AisMessage::BinarySingleSlot(m) => {
            assert_eq!(m.destination_mmsi, Some(987654321));
            assert_eq!(m.data, vec![1, 0, 1, 1, 0, 0, 1, 0]);
        }
        other => panic!("{other:?}"),
    }
}

// Synthetic one-hot probes for the public flags at their documented bit offsets.
#[test]
fn type25_flags_are_exact() {
    let kind = 25;
    for (set_bit, expected) in [(None, [false]), (Some(39), [true])] {
        let mut bits = vec![0; 40];
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
            AisMessage::BinarySingleSlot(message) => assert_eq!(
                [message.binary_data_flag],
                expected,
                "type {kind}, bit {set_bit:?}"
            ),
            other => panic!("unexpected {other:?}"),
        }
    }
}
