#![cfg(feature = "ais")]
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type13_safety_ack_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,=39UOj0jFs9R,0*65").expect("valid");
    let msg = parser.decode(&frame).expect("decoded");
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
    let msg = parser.decode(&frame).expect("decoded");
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
