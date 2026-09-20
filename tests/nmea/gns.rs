#![cfg(feature = "gns")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Gns;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$GPGNS,111648.00,0235.0379,S,04422.1450,W,ANN,12,0.8,8.5,-22.3,,,S*5D")
            .expect("valid");
    let gns = Gns::parse(&frame.fields).expect("parse");
    let sentence = gns.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gns2 = Gns::parse(&frame2.fields).expect("parse");
    assert_eq!(gns, gns2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$GPGNS,111648.00,0235.0379,S,04422.1450,W,ANN,12,0.8,8.5,-22.3,,,S*5D")
            .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gns(_)));
}

#[test]
fn gns_values() {
    let frame =
        parse_frame("$GPGNS,111648.00,0235.0379,S,04422.1450,W,ANN,12,0.8,8.5,-22.3,,,S*5D")
            .expect("valid");
    let gns = Gns::parse(&frame.fields).expect("parse");
    assert_eq!(gns.time, Some("111648.00".to_string()));
    assert!((gns.lat.expect("lat") - 235.0379).abs() < 1e-9);
    assert_eq!(gns.ns, Some('S'));
    assert!((gns.lon.expect("lon") - 4422.145).abs() < 1e-9);
    assert_eq!(gns.ew, Some('W'));
    assert_eq!(gns.mode, Some("ANN".to_string()));
    assert_eq!(gns.num_sats, Some(12));
    assert!((gns.hdop.expect("hdop") - 0.8).abs() < 1e-4);
    assert!((gns.altitude.expect("alt") - 8.5).abs() < 1e-4);
    assert!((gns.geoid_sep.expect("geoid") - (-22.3)).abs() < 1e-3);
    assert_eq!(gns.dgps_age, None);
    assert_eq!(gns.dgps_station, None);
    assert_eq!(gns.nav_status, Some('S'));
    // half (b): canonical re-encode — "04422.1450" → "04422.145" (trailing zero dropped)
    let s = gns.to_sentence("GP").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(
        body,
        "GPGNS,111648.00,0235.0379,S,04422.145,W,ANN,12,0.8,8.5,-22.3,,,S"
    );
}

#[test]
fn roundtrip() {
    let original = Gns {
        time: Some("120000.00".to_string()),
        lat: Some(4807.038),
        ns: Some('N'),
        lon: Some(1131.0),
        ew: Some('E'),
        mode: Some("AAN".to_string()),
        num_sats: Some(10),
        hdop: Some(0.9),
        altitude: Some(100.5),
        geoid_sep: Some(-23.0),
        dgps_age: None,
        dgps_station: None,
        nav_status: Some('S'),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gns::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
