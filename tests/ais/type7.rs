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
