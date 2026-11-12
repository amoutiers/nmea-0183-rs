//! AIS Type 4 — Base Station Report.
#![cfg(feature = "ais")]

use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type_4_base_station_gpsd() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,403OviQuMGCqWrRO9>E6fE700@GO,0*4D").expect("valid Type 4 frame");
    let msg = expect_message(parser.decode(&frame).expect("Type 4 should decode"));

    match msg {
        AisMessage::BaseStation(report) => {
            assert!(report.mmsi > 0, "MMSI should be non-zero");
            if let (Some(lat), Some(lon)) = (report.latitude, report.longitude) {
                assert!((-90.0..=90.0).contains(&lat), "latitude {lat} out of range");
                assert!(
                    (-180.0..=180.0).contains(&lon),
                    "longitude {lon} out of range"
                );
            }
            if let Some(h) = report.hour {
                assert!(h < 24, "hour sentinel not filtered: {h}");
            }
            if let Some(m) = report.minute {
                assert!(m < 60, "minute sentinel not filtered: {m}");
            }
        }
        other => panic!("expected BaseStation, got {other:?}"),
    }
}

#[test]
fn type4_values() {
    let mut parser = AisParser::new();
    let frame =
        parse_frame("!AIVDM,1,1,,A,403OviQuMGCqWrRO9>E6fE700@GO,0*4D").expect("valid Type 4 frame");
    let msg = expect_message(parser.decode(&frame).expect("Type 4 should decode"));
    match msg {
        AisMessage::BaseStation(r) => {
            assert_eq!(r.mmsi, 3669702);
            assert_eq!(r.year, Some(2007));
            assert_eq!(r.month, Some(5));
            let lat = r.latitude.expect("latitude present");
            let lon = r.longitude.expect("longitude present");
            assert!((lat - 36.883766666666666).abs() < 0.00001, "lat was {lat}");
            assert!(
                (lon - (-76.35236166666667)).abs() < 0.00001,
                "lon was {lon}"
            );
        }
        other => panic!("expected BaseStation, got {other:?}"),
    }
}

// Synthetic one-hot probes for the public flags at their documented bit offsets.
#[test]
fn type4_flags_are_exact() {
    let kind = 4;
    for (set_bit, expected) in [
        (None, [false, false, false]),
        (Some(78), [true, false, false]),
        (Some(138), [false, true, false]),
        (Some(148), [false, false, true]),
    ] {
        let mut bits = vec![0; 168];
        super::set_ais_bits(&mut bits, 0, 6, kind);
        if let Some(bit) = set_bit {
            super::set_ais_bits(&mut bits, bit, 1, 1);
        }
        let wire = super::ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::BaseStation(message) => assert_eq!(
                [
                    message.position_accuracy,
                    message.transmission_control,
                    message.raim
                ],
                expected,
                "type {kind}, bit {set_bit:?}"
            ),
            other => panic!("unexpected {other:?}"),
        }
    }
}

#[test]
fn type4_day_availability() {
    for (raw, expected) in [(0, None), (1, Some(1)), (31, Some(31))] {
        let mut bits = vec![0; 168];
        super::set_ais_bits(&mut bits, 0, 6, 4);
        super::set_ais_bits(&mut bits, 56, 5, raw);
        let wire = super::ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::BaseStation(message) => assert_eq!(message.day, expected),
            other => panic!("unexpected {other:?}"),
        }
    }
}

// Synthetic suffix probes for the accepted minimum-length contract.
#[test]
fn type4_trailing_bits_preserve_report() {
    let decode = |bits: &[u8]| {
        let wire = super::ais_frame_from_bits(bits);
        nmea_0183_rs::ais::AisParser::new()
            .decode(&nmea_0183_rs::parse_frame(&wire).expect("frame"))
    };
    for kind in [4] {
        let mut bits: Vec<u8> = (0..168).map(|index| u8::from(index % 3 == 0)).collect();
        super::set_ais_bits(&mut bits, 0, 6, kind);
        super::set_ais_bits(&mut bits, 6, 2, 2);
        super::set_ais_bits(&mut bits, 8, 30, 123456789);
        let expected = super::expect_message(decode(&bits).expect("168 bits"));
        for length in [169, 174, 192] {
            let mut extended = bits.clone();
            extended.extend((168..length).map(|index| u8::from(index % 2 == 0)));
            assert_eq!(
                super::expect_message(decode(&extended).expect("suffix accepted")),
                expected
            );
        }
        assert!(decode(&bits[..167]).is_err());
    }
}
