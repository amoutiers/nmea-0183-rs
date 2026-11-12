#![cfg(feature = "ais")]
use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type11_utc_date_response_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,B,;4R33:1uUK2F`q?mOt@@GoQ00000,0*5D").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::UtcDateResponse(utc) = msg {
        assert!(utc.mmsi > 0);
        if let Some(lat) = utc.latitude {
            assert!((-90.0..=90.0).contains(&lat));
        }
        if let Some(lon) = utc.longitude {
            assert!((-180.0..=180.0).contains(&lon));
        }
        if let Some(h) = utc.hour {
            assert!(h < 24, "hour sentinel not filtered");
        }
    } else {
        panic!("expected UtcDateResponse (type 11), got {msg:?}");
    }
}

#[test]
fn type11_values() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,B,;4R33:1uUK2F`q?mOt@@GoQ00000,0*5D").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    match msg {
        AisMessage::UtcDateResponse(u) => {
            assert_eq!(u.mmsi, 304137000);
            assert_eq!(u.year, Some(2009));
            assert_eq!(u.month, Some(5));
            let lat = u.latitude.expect("latitude present");
            let lon = u.longitude.expect("longitude present");
            assert!((lat - 28.409116666666666).abs() < 0.00001, "lat was {lat}");
            assert!(
                (lon - (-94.40768333333334)).abs() < 0.00001,
                "lon was {lon}"
            );
        }
        other => panic!("expected UtcDateResponse, got {other:?}"),
    }
}

// Synthetic one-hot probes for the public flags at their documented bit offsets.
#[test]
fn type11_flags_are_exact() {
    let kind = 11;
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
            AisMessage::UtcDateResponse(message) => assert_eq!(
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
fn type11_day_availability() {
    for (raw, expected) in [(0, None), (1, Some(1)), (31, Some(31))] {
        let mut bits = vec![0; 168];
        super::set_ais_bits(&mut bits, 0, 6, 11);
        super::set_ais_bits(&mut bits, 56, 5, raw);
        let wire = super::ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::UtcDateResponse(message) => assert_eq!(message.day, expected),
            other => panic!("unexpected {other:?}"),
        }
    }
}

// Synthetic suffix probes for the accepted minimum-length contract.
#[test]
fn type11_trailing_bits_preserve_report() {
    let decode = |bits: &[u8]| {
        let wire = super::ais_frame_from_bits(bits);
        nmea_0183_rs::ais::AisParser::new()
            .decode(&nmea_0183_rs::parse_frame(&wire).expect("frame"))
    };
    for kind in [11] {
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
