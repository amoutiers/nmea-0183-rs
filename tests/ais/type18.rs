//! AIS Type 18 — Class B Standard Position Report.
#![cfg(feature = "ais")]

use super::expect_message;
use nmea_0183_rs::ais::{AisClass, AisDecodeOutcome, AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type_18_class_b_position() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,B6CdCm0t3`tba35f@V9faHi7kP06,0*58")
        .expect("valid Type 18 frame");
    let msg = parser.decode(&frame);

    if let Ok(AisDecodeOutcome::Message(AisMessage::Position(pos))) = &msg {
        assert_eq!(pos.ais_class, AisClass::B, "Type 18 should be Class B");
        assert!(pos.mmsi > 0, "MMSI should be non-zero");
    }
}

#[test]
fn type18_values() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,B6CdCm0t3`tba35f@V9faHi7kP06,0*58")
        .expect("valid Type 18 frame");
    let msg = expect_message(parser.decode(&frame).expect("Type 18 should decode"));
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

// Synthetic one-hot probes for the public flags at their documented bit offsets.
#[test]
fn type18_flags_are_exact() {
    let kind = 18;
    for (set_bit, expected) in [(None, [false]), (Some(56), [true])] {
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

// Synthetic suffix probes for the accepted minimum-length contract.
#[test]
fn type18_trailing_bits_preserve_report() {
    let decode = |bits: &[u8]| {
        let wire = super::ais_frame_from_bits(bits);
        nmea_0183_rs::ais::AisParser::new()
            .decode(&nmea_0183_rs::parse_frame(&wire).expect("frame"))
    };
    for kind in [18] {
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
                    nmea_0183_rs::ais::PositionReport::decode_class_b(&extended).as_ref(),
                    Some(report)
                );
            } else {
                panic!("expected position report");
            }
        }
        assert!(decode(&bits[..167]).is_err());
        assert!(nmea_0183_rs::ais::PositionReport::decode_class_b(&bits[..167]).is_none());
    }
}
