#![cfg(feature = "ais")]
use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type8_binary_broadcast_gpsd() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,85Mwp`1Kf3aCnsNvBWLi=wQuNhA5t43N`5nCuI=p<IBfVqnMgPGs,0*47")
            .expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::BinaryBroadcast(bb) = msg {
        assert!(bb.mmsi > 0);
        assert!(!bb.data.is_empty(), "should have binary data");
    } else {
        panic!("expected BinaryBroadcast (type 8), got {msg:?}");
    }
}

#[test]
fn type8_dac_fid_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,B,83aDChPj2d<dL<uM=hhhI?a@6HP0,0*40").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::BinaryBroadcast(bb) = msg {
        assert!(bb.dac > 0 || bb.fid > 0, "DAC or FID should be nonzero");
    } else {
        panic!("expected BinaryBroadcast, got {msg:?}");
    }
}

#[test]
fn type8_values() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,B,83aDChPj2d<dL<uM=hhhI?a@6HP0,0*40").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    match msg {
        AisMessage::BinaryBroadcast(bb) => {
            assert_eq!(bb.mmsi, 244650946);
            assert_eq!(bb.dac, 200);
            assert_eq!(bb.fid, 10);
        }
        other => panic!("expected BinaryBroadcast, got {other:?}"),
    }
}

#[test]
fn type8_binary_data() {
    // GPSD ais.nmea: six-bit AIS alphabet expanded independently, excluding 56 header bits.
    match expect_message(AisParser::new().decode(&parse_frame("!AIVDM,1,1,,B,83aDChPj2d<dL<uM=hhhI?a@6HP0,0*40").expect("frame")).expect("decode")) {
        AisMessage::BinaryBroadcast(m)=>assert_eq!(m.data, "1100001100101100011100001100111101011101001101110000110000110000011001001111101001010000000110011000100000000000".bytes().map(|b| b-b'0').collect::<Vec<_>>()),
        other=>panic!("{other:?}")
    }
}
