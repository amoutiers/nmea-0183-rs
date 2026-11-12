#![cfg(feature = "ais")]
use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type15_interrogation_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,?5OP=l00052HD00,2*5B").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::Interrogation(intr) = msg {
        assert!(intr.mmsi > 0);
        assert!(intr.mmsi_1 > 0);
    } else {
        panic!("expected Interrogation (type 15), got {msg:?}");
    }
}

#[test]
fn type15_with_second_station_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,?39a?2PjKFFPD01o:Gq1igvp2<3w,0*0B").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::Interrogation(intr) = msg {
        assert!(intr.mmsi > 0);
    } else {
        panic!("expected Interrogation, got {msg:?}");
    }
}

#[test]
fn type15_values() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,?5OP=l00052HD00,2*5B").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    match msg {
        AisMessage::Interrogation(intr) => {
            assert_eq!(intr.mmsi, 368578000);
            assert_eq!(intr.mmsi_1, 5158);
            assert_eq!(intr.msg_type_1_1, 5);
        }
        other => panic!("expected Interrogation, got {other:?}"),
    }
}

// Synthetic 160-bit optional requests, matching interrogation.rs field layout.
#[test]
fn type15_optional_requests_are_exact() {
    for available in [true, false] {
        let mut bits = vec![0; 160];
        for (offset, width, value) in [
            (0, 6, 15),
            (40, 30, 111111111),
            (70, 6, 5),
            (76, 12, 321),
            (90, 6, if available { 18 } else { 0 }),
            (96, 12, 123),
            (110, 30, if available { 222222222 } else { 0 }),
            (140, 6, 24),
            (146, 12, 456),
        ] {
            super::set_ais_bits(&mut bits, offset, width, value);
        }
        let wire = super::ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::Interrogation(message) => {
                assert_eq!(
                    (
                        message.mmsi_1,
                        message.msg_type_1_1,
                        message.slot_offset_1_1
                    ),
                    (111111111, 5, 321)
                );
                let expected = if available {
                    (Some(18), Some(123), Some(222222222), Some(24), Some(456))
                } else {
                    (None, None, None, None, None)
                };
                assert_eq!(
                    (
                        message.msg_type_1_2,
                        message.slot_offset_1_2,
                        message.mmsi_2,
                        message.msg_type_2_1,
                        message.slot_offset_2_1
                    ),
                    expected
                );
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
