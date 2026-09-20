#![cfg(feature = "rmc")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Rmc;
use nmea_0183_rs::{EncodeError, NmeaSentence, parse_frame};

fn sentence(body: &str) -> String {
    let checksum = body[1..].bytes().fold(0u8, |acc, byte| acc ^ byte);
    format!("{body}*{checksum:02X}")
}

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$GPRMC,085412.000,A,5222.3198,N,00454.5784,E,0.58,251.34,030414,,,A*65")
            .expect("valid");
    let rmc = Rmc::parse(&frame.fields).expect("parse");
    let sentence = rmc.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rmc2 = Rmc::parse(&frame2.fields).expect("parse");
    assert_eq!(rmc, rmc2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$GPRMC,085412.000,A,5222.3198,N,00454.5784,E,0.58,251.34,030414,,,A*65")
            .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rmc(_)));
}

#[test]
fn rmc_values() {
    let frame =
        parse_frame("$GPRMC,085412.000,A,5222.3198,N,00454.5784,E,0.58,251.34,030414,,,A*65")
            .expect("valid");
    let rmc = Rmc::parse(&frame.fields).expect("parse");
    assert_eq!(rmc.time, Some("085412.000".to_string()));
    assert_eq!(rmc.status, Some('A'));
    assert!((rmc.lat.expect("lat") - 5222.3198).abs() < 1e-9);
    assert_eq!(rmc.ns, Some('N'));
    assert!((rmc.lon.expect("lon") - 454.5784).abs() < 1e-9);
    assert_eq!(rmc.ew, Some('E'));
    assert!((rmc.sog.expect("sog") - 0.58).abs() < 1e-4);
    assert!((rmc.cog.expect("cog") - 251.34).abs() < 1e-3);
    assert_eq!(rmc.date, Some("030414".to_string()));
    assert_eq!(rmc.mag_var, None);
    assert_eq!(rmc.mag_var_ew, None);
    assert_eq!(rmc.pos_mode, Some('A'));
    assert_eq!(rmc.nav_status, None);
    // half (b): canonical re-encode
    let s = rmc.to_sentence("GP").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(
        body,
        "GPRMC,085412.000,A,5222.3198,N,00454.5784,E,0.58,251.34,030414,,,A,"
    );
}

#[test]
fn roundtrip() {
    let original = Rmc {
        time: Some("120000.00".to_string()),
        status: Some('A'),
        lat: Some(4807.038),
        ns: Some('N'),
        lon: Some(1131.0),
        ew: Some('E'),
        sog: Some(5.5),
        cog: Some(54.7),
        date: Some("230394".to_string()),
        mag_var: Some(3.1),
        mag_var_ew: Some('E'),
        pos_mode: Some('A'),
        nav_status: None,
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rmc::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn rmc_rejects_multi_character_status() {
    let malformed = sentence("$GPRMC,120000.00,ACTIVE");
    let frame = parse_frame(&malformed).expect("valid frame");
    let parsed = Rmc::parse(&frame.fields).expect("typed parse");
    assert_eq!(parsed.status, None);

    let valid = sentence("$GPRMC,120000.00,A");
    let frame = parse_frame(&valid).expect("valid frame");
    let parsed = Rmc::parse(&frame.fields).expect("typed parse");
    assert_eq!(parsed.status, Some('A'));
}

#[test]
fn rmc_rejects_invalid_ddmm_coordinates_on_encode() {
    let mut invalid_lat = Rmc::parse(&[]).expect("empty RMC");
    invalid_lat.lat = Some(1260.0);
    assert_eq!(
        invalid_lat.to_sentence("GP"),
        Err(EncodeError::InvalidCoordinate)
    );

    let mut invalid_lon = Rmc::parse(&[]).expect("empty RMC");
    invalid_lon.lon = Some(18100.0);
    assert_eq!(
        invalid_lon.to_sentence("GP"),
        Err(EncodeError::InvalidCoordinate)
    );
}
