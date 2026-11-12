#![cfg(feature = "ais")]
use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type13_safety_ack_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,=39UOj0jFs9R,0*65").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::BinaryAck(ack) = msg {
        assert_eq!(ack.msg_type, 13);
        assert!(ack.mmsi > 0);
        assert!(!ack.acks.is_empty());
    } else {
        panic!("expected BinaryAck (type 13), got {msg:?}");
    }
}

#[test]
fn type13_values() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,=39UOj0jFs9R,0*65").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    match msg {
        AisMessage::BinaryAck(ack) => {
            assert_eq!(ack.mmsi, 211378120);
            assert_eq!(ack.acks.len(), 1);
            assert_eq!(ack.acks[0].mmsi, 211217560);
            assert_eq!(ack.acks[0].sequence, 2);
        }
        other => panic!("expected BinaryAck (type 13), got {other:?}"),
    }
}

// Synthetic bounds probes: the 200-bit variant deliberately carries an extra entry.
#[test]
fn type13_acknowledgements_are_capped_at_four() {
    use nmea_0183_rs::ais::messages::AckEntry;
    let expected = vec![
        AckEntry {
            mmsi: 111111111,
            sequence: 0,
        },
        AckEntry {
            mmsi: 222222222,
            sequence: 1,
        },
        AckEntry {
            mmsi: 333333333,
            sequence: 2,
        },
        AckEntry {
            mmsi: 444444444,
            sequence: 3,
        },
    ];
    for length in [168, 200] {
        let mut bits = vec![0; length];
        super::set_ais_bits(&mut bits, 0, 6, 13);
        for (index, entry) in expected.iter().enumerate() {
            super::set_ais_bits(&mut bits, 40 + 32 * index, 30, entry.mmsi);
            super::set_ais_bits(&mut bits, 70 + 32 * index, 2, u32::from(entry.sequence));
        }
        if length == 200 {
            super::set_ais_bits(&mut bits, 168, 30, 555555555);
        }
        let wire = super::ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::BinaryAck(message) => {
                assert_eq!(message.msg_type, 13);
                assert_eq!(message.acks, expected);
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
