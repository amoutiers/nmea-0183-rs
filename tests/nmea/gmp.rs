#![cfg(feature = "gmp")]

use nmea_0183_rs::nmea::{NmeaEncodable, sentences::Gmp};
use nmea_0183_rs::{NmeaSentence, parse_frame};

const FIXTURE: &str = "$GNGMP,122310.2,UTM,M20,12345.56,65543.21,DA,14,0.9,1005.543,6.5,5.2,23*75";
// NMEA 0183 v3.01, section 6.3, p. 44.
// https://www.plaisance-pratique.com/IMG/pdf/NMEA0183-2.pdf

fn assert_close(actual: Option<f32>, expected: f32) {
    assert!((actual.expect("value present") - expected).abs() < 0.001);
}

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid GMP frame");
    let original = Gmp::parse(&frame.fields);
    let sentence = original.to_sentence("GN").expect("encode GMP");
    let encoded = parse_frame(sentence.trim()).expect("re-parse GMP");
    assert_eq!(Gmp::parse(&encoded.fields), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid GMP frame");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gmp(_)));
}

#[test]
fn gmp_values_nmea_301() {
    let frame = parse_frame(FIXTURE).expect("valid NMEA 3.01 GMP frame");
    let gmp = Gmp::parse(&frame.fields);

    assert_eq!(gmp.time.as_deref(), Some("122310.2"));
    assert_eq!(gmp.map_projection.as_deref(), Some("UTM"));
    assert_eq!(gmp.map_zone.as_deref(), Some("M20"));
    assert_close(gmp.northing, 12345.56);
    assert_close(gmp.easting, 65543.21);
    assert_eq!(gmp.mode.as_deref(), Some("DA"));
    assert_eq!(gmp.num_sats, Some(14));
    assert_close(gmp.hdop, 0.9);
    assert_close(gmp.altitude, 1005.543);
    assert_close(gmp.geoid_sep, 6.5);
    assert_close(gmp.dgps_age, 5.2);
    assert_eq!(gmp.dgps_station.as_deref(), Some("23"));
}

#[test]
fn roundtrip() {
    let original = Gmp {
        time: Some("122310.2".into()),
        map_projection: Some("UTM".into()),
        map_zone: Some("M20".into()),
        northing: Some(12345.56),
        easting: Some(65543.21),
        mode: Some("DA".into()),
        num_sats: Some(14),
        hdop: Some(0.9),
        altitude: Some(1005.543),
        geoid_sep: Some(6.5),
        dgps_age: Some(5.2),
        dgps_station: Some("23".into()),
    };

    let sentence = original.to_sentence("GN").expect("encode GMP");
    let frame = parse_frame(sentence.trim()).expect("re-parse GMP");
    assert_eq!(Gmp::parse(&frame.fields), original);
}
