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
        }
        other => panic!("expected BinaryAddressed, got {other:?}"),
    }
}
