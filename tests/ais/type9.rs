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

// Synthetic Type 9 integer speed and altitude sentinels.
#[test]
fn type9_altitude_and_speed_sentinels() {
    for (altitude, speed, expected_altitude, expected_speed) in [
        (0, 17, Some(0), Some(17.0)),
        (4094, 17, Some(4094), Some(17.0)),
        (4095, 17, None, Some(17.0)),
        (42, 0, Some(42), Some(0.0)),
        (42, 1022, Some(42), Some(1022.0)),
        (42, 1023, Some(42), None),
    ] {
        let mut bits = vec![0; 168];
        super::set_ais_bits(&mut bits, 0, 6, 9);
        super::set_ais_bits(&mut bits, 38, 12, altitude);
        super::set_ais_bits(&mut bits, 50, 10, speed);
        let wire = super::ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::SarAircraft(message) => {
                assert_eq!(message.altitude, expected_altitude);
                assert_eq!(message.sog, expected_speed);
            }
            other => panic!("unexpected {other:?}"),
        }
    }
}

// Synthetic one-hot probes for the public flags at their documented bit offsets.
#[test]
fn type9_flags_are_exact() {
    let kind = 9;
    for (set_bit, expected) in [(None, [false]), (Some(60), [true])] {
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
            AisMessage::SarAircraft(message) => assert_eq!(
                [message.position_accuracy],
                expected,
                "type {kind}, bit {set_bit:?}"
            ),
            other => panic!("unexpected {other:?}"),
        }
    }
}

// Synthetic suffix probes for the accepted minimum-length contract.
#[test]
fn type9_trailing_bits_preserve_report() {
    let decode = |bits: &[u8]| {
        let wire = super::ais_frame_from_bits(bits);
        nmea_0183_rs::ais::AisParser::new()
            .decode(&nmea_0183_rs::parse_frame(&wire).expect("frame"))
    };
    for kind in [9] {
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
