#![cfg(feature = "ais")]
use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type6_binary_addressed_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,B,6B?n;be:cbapalgc;i6?Ow4,2*4A").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::BinaryAddressed(ba) = msg {
        assert!(ba.mmsi > 0);
        assert!(ba.dest_mmsi > 0);
    } else {
        panic!("expected BinaryAddressed (type 6), got {msg:?}");
    }
}

#[test]
fn type6_dac_fid_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,6h2E:81>NmKC04p0J<000?vv20Ru,0*31").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::BinaryAddressed(ba) = msg {
        assert!(ba.dac > 0 || ba.fid > 0, "DAC or FID should be nonzero");
    } else {
        panic!("expected BinaryAddressed, got {msg:?}");
    }
}

#[test]
fn type6_values() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,6h2E:81>NmKC04p0J<000?vv20Ru,0*31").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    match msg {
        AisMessage::BinaryAddressed(ba) => {
            assert_eq!(ba.mmsi, 2443808);
            assert_eq!(ba.dac, 1);
            assert_eq!(ba.fid, 14);
            assert_eq!(ba.dest_mmsi, 329176500);
            assert_eq!(ba.sequence, 0);
            assert!(ba.retransmit);
            // GPSD fixture, independently unarmored after its 88-bit header.
            assert_eq!(
                ba.data,
                "00000000011010001100000000000000000000001111111110111110000010000000100010111101"
                    .bytes()
                    .map(|bit| bit - b'0')
                    .collect::<Vec<_>>()
            );
        }
        other => panic!("expected BinaryAddressed, got {other:?}"),
    }
}

// Synthetic one-hot probes for the public flags at their documented bit offsets.
#[test]
fn type6_flags_are_exact() {
    let kind = 6;
    for (set_bit, expected) in [(None, [false]), (Some(70), [true])] {
        let mut bits = vec![0; 88];
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
            AisMessage::BinaryAddressed(message) => assert_eq!(
                [message.retransmit],
                expected,
                "type {kind}, bit {set_bit:?}"
            ),
            other => panic!("unexpected {other:?}"),
        }
    }
}
