#![cfg(feature = "ais")]
use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type9_sar_aircraft_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,91b77=h3h00nHt0Q3r@@07000<0b,0*69").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::SarAircraft(sar) = msg {
        assert!(sar.mmsi > 0);
        if let Some(lat) = sar.latitude {
            assert!((-90.0..=90.0).contains(&lat));
        }
        if let Some(lon) = sar.longitude {
            assert!((-180.0..=180.0).contains(&lon));
        }
    } else {
        panic!("expected SarAircraft (type 9), got {msg:?}");
    }
}

#[test]
fn type9_second_fixture_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,B,91b55wi;hbOS@OdQAC062Ch2089h,0*30").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::SarAircraft(sar) = msg {
        assert!(sar.mmsi > 0);
    } else {
        panic!("expected SarAircraft, got {msg:?}");
    }
}

#[test]
fn type9_flag_offsets_gpsd() {
    let mut parser = AisParser::new();
    // Fixture: bit 142=1 (dte), bit 146=0 (assigned), bit 147=0 (raim).
    // Verified by hand-decoding the 6-bit armor payload. These three bits
    // were corrected from wrong offsets (138/139) to ITU-R M.1371 (146/147).
    let frame = parse_frame("!AIVDM,1,1,,B,91b55wi;hbOS@OdQAC062Ch2089h,0*30").expect("valid");
    match expect_message(parser.decode(&frame).expect("decoded")) {
        AisMessage::SarAircraft(sar) => {
            assert!(sar.dte, "DTE@142 should be true for this fixture");
            assert!(
                !sar.assigned,
                "assigned@146 should be false for this fixture"
            );
            assert!(!sar.raim, "raim@147 should be false for this fixture");
        }
        other => panic!("expected SarAircraft, got {other:?}"),
    }
}
