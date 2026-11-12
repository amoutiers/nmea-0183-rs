#![cfg(feature = "ais")]
use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser, ChannelManagementScope};
use nmea_0183_rs::parse_frame;
// Synthetic ITU-R M.1371-5 message 22: destination fields at 69/104 (30 bits),
// addressed selector at 139. Geographic fields: 18/17 signed bits, 1/600 degree.

#[test]
fn type22_addressed() {
    let wire = "!AIVDM,1,1,,A,F3aEOK000000lvmpp3CsGSP@0000,0*03";
    let (body, checksum) = wire[1..].split_once('*').expect("checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |v, b| v ^ b),
        u8::from_str_radix(checksum, 16).expect("hex")
    );
    match expect_message(
        AisParser::new()
            .decode(&parse_frame(wire).expect("frame"))
            .expect("decode"),
    ) {
        AisMessage::ChannelManagement(m) => assert_eq!(
            m.scope,
            ChannelManagementScope::Addressed {
                station_1_mmsi: 111111111,
                station_2_mmsi: 222222222
            }
        ),
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn type22_geographic() {
    let wire = "!AIVDM,1,1,,A,F3aEOK000007ub072?n`?`T00000,0*5C";
    let (body, checksum) = wire[1..].split_once('*').expect("checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |v, b| v ^ b),
        u8::from_str_radix(checksum, 16).expect("hex")
    );
    match expect_message(
        AisParser::new()
            .decode(&parse_frame(wire).expect("frame"))
            .expect("decode"),
    ) {
        AisMessage::ChannelManagement(m) => assert_eq!(
            m.scope,
            ChannelManagementScope::Geographic {
                northeast_longitude: Some(-2.0),
                northeast_latitude: Some(3.0),
                southwest_longitude: Some(-4.0),
                southwest_latitude: Some(-5.0)
            }
        ),
        other => panic!("unexpected {other:?}"),
    }
}

// Synthetic one-hot probes for the public flags at their documented bit offsets.
#[test]
fn type22_flags_are_exact() {
    let kind = 22;
    for (set_bit, expected) in [
        (None, [false, false, false]),
        (Some(68), [true, false, false]),
        (Some(140), [false, true, false]),
        (Some(141), [false, false, true]),
    ] {
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
            AisMessage::ChannelManagement(message) => assert_eq!(
                [
                    message.power_low,
                    message.channel_a_bandwidth,
                    message.channel_b_bandwidth
                ],
                expected,
                "type {kind}, bit {set_bit:?}"
            ),
            other => panic!("unexpected {other:?}"),
        }
    }
}
