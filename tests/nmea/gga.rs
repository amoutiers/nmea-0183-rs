#![cfg(feature = "gga")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Gga;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame(
        "$GPGGA,172814.0,3723.46587704,N,12202.26957864,W,2,6,1.2,18.893,M,-25.669,M,2.0,0031*4F",
    )
    .expect("valid");
    let gga = Gga::parse(&frame.fields).expect("parse");
    let sentence = gga.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gga2 = Gga::parse(&frame2.fields).expect("parse");
    assert_eq!(gga, gga2);
}

#[test]
fn dispatch() {
    let frame = parse_frame(
        "$GPGGA,172814.0,3723.46587704,N,12202.26957864,W,2,6,1.2,18.893,M,-25.669,M,2.0,0031*4F",
    )
    .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gga(_)));
}

#[test]
fn gga_values() {
    let frame = parse_frame(
        "$GPGGA,172814.0,3723.46587704,N,12202.26957864,W,2,6,1.2,18.893,M,-25.669,M,2.0,0031*4F",
    )
    .expect("valid");
    let gga = Gga::parse(&frame.fields).expect("parse");
    assert_eq!(gga.time, Some("172814.0".to_string()));
    assert!((gga.lat.expect("lat") - 3723.46587704).abs() < 1e-9);
    assert_eq!(gga.ns, Some('N'));
    assert!((gga.lon.expect("lon") - 12202.26957864).abs() < 1e-9);
    assert_eq!(gga.ew, Some('W'));
    assert_eq!(gga.quality, Some(2));
    assert_eq!(gga.num_sats, Some(6));
    assert!((gga.hdop.expect("hdop") - 1.2).abs() < 1e-4);
    assert!((gga.altitude.expect("alt") - 18.893).abs() < 1e-3);
    assert_eq!(gga.alt_unit, Some('M'));
    assert!((gga.geoid_sep.expect("geoid") - (-25.669)).abs() < 1e-3);
    assert_eq!(gga.geoid_unit, Some('M'));
    assert!((gga.dgps_age.expect("dgps_age") - 2.0).abs() < 1e-4);
    assert_eq!(gga.dgps_station, Some("0031".to_string()));
    // half (b): canonical re-encode — "2.0" dgps_age → "2"
    let s = gga.to_sentence("GP").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(
        body,
        "GPGGA,172814.0,3723.46587704,N,12202.26957864,W,2,6,1.2,18.893,M,-25.669,M,2,0031"
    );
}

#[test]
fn roundtrip() {
    let original = Gga {
        time: Some("120000.00".to_string()),
        lat: Some(4807.038),
        ns: Some('N'),
        lon: Some(1131.0),
        ew: Some('E'),
        quality: Some(1),
        num_sats: Some(8),
        hdop: Some(0.9),
        altitude: Some(545.4),
        alt_unit: Some('M'),
        geoid_sep: Some(46.9),
        geoid_unit: Some('M'),
        dgps_age: None,
        dgps_station: None,
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gga::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
