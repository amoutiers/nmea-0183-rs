//! AIS Type 24 — Static Data Report (Class B).
#![cfg(feature = "ais")]

use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser, StaticDataReport};
use nmea_0183_rs::parse_frame;

#[test]
fn type_24_class_b_static_gpsd() {
    let mut parser = AisParser::new();

    let frame_a = parse_frame("!AIVDM,1,1,,A,H42O55i18tMET00000000000000,2*6D")
        .expect("valid Type 24 Part A");
    match expect_message(parser.decode(&frame_a).expect("Part A should decode")) {
        AisMessage::StaticReport(StaticDataReport::PartA {
            mmsi, vessel_name, ..
        }) => {
            assert!(mmsi > 0, "MMSI should be non-zero");
            assert!(!vessel_name.is_empty(), "vessel_name should not be empty");
        }
        other => panic!("expected StaticReport(PartA), got {other:?}"),
    }

    let frame_b = parse_frame("!AIVDM,1,1,,A,H42O55lti4hhhilD3nink000?050,0*40")
        .expect("valid Type 24 Part B");
    match expect_message(parser.decode(&frame_b).expect("Part B should decode")) {
        AisMessage::StaticReport(StaticDataReport::PartB { mmsi, .. }) => {
            assert!(mmsi > 0, "MMSI should be non-zero");
        }
        other => panic!("expected StaticReport(PartB), got {other:?}"),
    }
}

#[test]
fn type_24_part_b_values_gpsd() {
    let mut parser = AisParser::new();
    let frame_b = parse_frame("!AIVDM,1,1,,A,H42O55lti4hhhilD3nink000?050,0*40")
        .expect("valid Type 24 Part B");
    match expect_message(parser.decode(&frame_b).expect("Part B should decode")) {
        AisMessage::StaticReport(StaticDataReport::PartB {
            mmsi,
            callsign,
            ship_type,
            ..
        }) => {
            assert_eq!(mmsi, 271041815);
            assert_eq!(ship_type, 60, "ship_type @40");
            assert_eq!(callsign, "TC6163", "callsign @90");
        }
        other => panic!("expected StaticReport(PartB), got {other:?}"),
    }
}

// Synthetic suffix probes for the accepted minimum-length contract.
#[test]
fn type24_trailing_bits_preserve_report() {
    let decode = |bits: &[u8]| {
        let wire = super::ais_frame_from_bits(bits);
        nmea_0183_rs::ais::AisParser::new()
            .decode(&nmea_0183_rs::parse_frame(&wire).expect("frame"))
    };
    for kind in [24] {
        let mut bits: Vec<u8> = (0..168).map(|index| u8::from(index % 3 == 0)).collect();
        super::set_ais_bits(&mut bits, 0, 6, kind);
        super::set_ais_bits(&mut bits, 6, 2, 2);
        super::set_ais_bits(&mut bits, 8, 30, 123456789);
        super::set_ais_bits(&mut bits, 38, 2, 1);
        let expected = super::expect_message(decode(&bits).expect("168 bits"));
        for length in [169, 174, 192] {
            let mut extended = bits.clone();
            extended.extend((168..length).map(|index| u8::from(index % 2 == 0)));
            assert_eq!(
                super::expect_message(decode(&extended).expect("suffix accepted")),
                expected
            );
            if let nmea_0183_rs::ais::AisMessage::StaticReport(report) = &expected {
                assert_eq!(
                    nmea_0183_rs::ais::StaticDataReport::decode(&extended).as_ref(),
                    Some(report)
                );
            } else {
                panic!("expected static report");
            }
        }
        assert!(decode(&bits[..167]).is_err());
        assert!(nmea_0183_rs::ais::StaticDataReport::decode(&bits[..167]).is_none());
        let mut part_a = bits[..160].to_vec();
        super::set_ais_bits(&mut part_a, 38, 2, 0);
        assert!(matches!(
            nmea_0183_rs::ais::StaticDataReport::decode(&part_a),
            Some(nmea_0183_rs::ais::StaticDataReport::PartA { .. })
        ));
        assert!(decode(&part_a).is_ok());
    }
}
