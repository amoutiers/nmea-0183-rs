#![cfg(feature = "ais")]
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type15_interrogation_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,?5OP=l00052HD00,2*5B").expect("valid");
    let msg = parser.decode(&frame).expect("decoded");
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
    let msg = parser.decode(&frame).expect("decoded");
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
    let msg = parser.decode(&frame).expect("decoded");
    match msg {
        AisMessage::Interrogation(intr) => {
            assert_eq!(intr.mmsi, 368578000);
            assert_eq!(intr.mmsi_1, 5158);
            assert_eq!(intr.msg_type_1_1, 5);
        }
        other => panic!("expected Interrogation, got {other:?}"),
    }
}
