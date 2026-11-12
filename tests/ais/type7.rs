#![cfg(feature = "ais")]
use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type7_binary_ack_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,702R5`hwCjq8,0*6B").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::BinaryAck(ack) = msg {
        assert_eq!(ack.msg_type, 7);
        assert!(ack.mmsi > 0);
        assert!(!ack.acks.is_empty(), "should have at least one ack entry");
    } else {
        panic!("expected BinaryAck (type 7), got {msg:?}");
    }
}

#[test]
fn type7_multiple_acks_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,7IiQ4T`UjA9lC;b:M<MWE@,4*01").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::BinaryAck(ack) = msg {
        assert!(ack.acks.len() >= 2, "should have multiple ack entries");
    } else {
        panic!("expected BinaryAck, got {msg:?}");
    }
}

#[test]
fn type7_values() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,702R5`hwCjq8,0*6B").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    match msg {
        AisMessage::BinaryAck(ack) => {
            assert_eq!(ack.mmsi, 2655651);
            assert_eq!(ack.acks.len(), 1);
            assert_eq!(ack.acks[0].mmsi, 265538450);
            assert_eq!(ack.acks[0].sequence, 0);
        }
        other => panic!("expected BinaryAck (type 7), got {other:?}"),
    }
}

// Synthetic bounds probes: the 200-bit variant deliberately carries an extra entry.
#[test]
fn type7_acknowledgements_are_capped_at_four() {
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
        super::set_ais_bits(&mut bits, 0, 6, 7);
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
                assert_eq!(message.msg_type, 7);
                assert_eq!(message.acks, expected);
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}
