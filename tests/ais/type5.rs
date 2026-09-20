//! AIS Type 5 — Static and Voyage Related Data (multi-fragment).
#![cfg(feature = "ais")]

use super::expect_message;
use nmea_0183_rs::ais::{AisClass, AisDecodeOutcome, AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type_5_multi_fragment_static_voyage_gpsd() {
    let mut parser = AisParser::new();

    let f1 = parse_frame(
        "!AIVDM,2,1,1,A,55?MbV02;H;s<HtKR20EHE:0@T4@Dn2222222216L961O5Gf0NSQEp6ClRp8,0*1C",
    )
    .expect("valid Type 5 fragment 1");
    assert!(
        matches!(parser.decode(&f1), Ok(AisDecodeOutcome::Pending)),
        "fragment 1 should be Pending"
    );

    let f2 = parse_frame("!AIVDM,2,2,1,A,88888888880,2*25").expect("valid Type 5 fragment 2");
    let msg = expect_message(
        parser
            .decode(&f2)
            .expect("fragment 2 should complete Type 5"),
    );

    match msg {
        AisMessage::StaticVoyage(svd) => {
            assert!(svd.mmsi > 0, "MMSI should be non-zero");
            assert!(
                !svd.vessel_name.is_empty(),
                "vessel_name should not be empty"
            );
            assert_eq!(svd.ais_class, AisClass::A, "Type 5 should be Class A");
        }
        other => panic!("expected StaticVoyage, got {other:?}"),
    }
}

#[test]
fn type5_values() {
    let mut parser = AisParser::new();
    let f1 = parse_frame(
        "!AIVDM,2,1,1,A,55?MbV02;H;s<HtKR20EHE:0@T4@Dn2222222216L961O5Gf0NSQEp6ClRp8,0*1C",
    )
    .expect("valid Type 5 fragment 1");
    assert!(
        matches!(parser.decode(&f1), Ok(AisDecodeOutcome::Pending)),
        "fragment 1 should be Pending"
    );
    let f2 = parse_frame("!AIVDM,2,2,1,A,88888888880,2*25").expect("valid Type 5 fragment 2");
    let msg = expect_message(
        parser
            .decode(&f2)
            .expect("fragment 2 should complete Type 5"),
    );
    match msg {
        AisMessage::StaticVoyage(sv) => {
            assert_eq!(sv.mmsi, 351759000);
            assert_eq!(sv.callsign, "3FOF8");
            assert_eq!(sv.vessel_name, "EVER DIADEM");
            assert_eq!(sv.ship_type, 70);
        }
        other => panic!("expected StaticVoyage, got {other:?}"),
    }
}
