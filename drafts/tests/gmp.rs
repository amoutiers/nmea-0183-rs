use nmea_0183_rs::nmea::sentences::Gmp;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn gmp_nmea_301_fixture_roundtrip() {
    // Fixture published in NMEA 0183 v3.01, section 6.3, p. 44:
    // https://www.plaisance-pratique.com/IMG/pdf/NMEA0183-2.pdf
    const FIXTURE: &str =
        "$GNGMP,122310.2,UTM,M20,12345.56,65543.21,DA,14,0.9,1005.543,6.5,5.2,23*75";

    let body = FIXTURE
        .strip_prefix('$')
        .and_then(|sentence| sentence.split_once('*').map(|(body, _)| body))
        .expect("NMEA fixture has body and checksum");
    let checksum = body.bytes().fold(0u8, |checksum, byte| checksum ^ byte);
    assert_eq!(checksum, 0x75);

    let frame = parse_frame(FIXTURE).expect("valid NMEA 3.01 GMP frame");
    let gmp = Gmp::parse(&frame.fields).expect("parse GMP");

    assert_eq!(gmp.time.as_deref(), Some("122310.2"));
    assert_eq!(gmp.map_projection.as_deref(), Some("UTM"));
    assert_eq!(gmp.map_zone.as_deref(), Some("M20"));
    assert_eq!(gmp.northing, Some(12345.56));
    assert_eq!(gmp.easting, Some(65543.21));
    assert_eq!(gmp.mode.as_deref(), Some("DA"));
    assert_eq!(gmp.num_sats, Some(14));
    assert_eq!(gmp.hdop, Some(0.9));
    assert_eq!(gmp.altitude, Some(1005.543));
    assert_eq!(gmp.geoid_sep, Some(6.5));
    assert_eq!(gmp.dgps_age, Some(5.2));
    assert_eq!(gmp.dgps_station.as_deref(), Some("23"));

    let sentence = gmp.to_sentence("GN").expect("encode GMP");
    assert_eq!(sentence.trim(), FIXTURE);
    let frame = parse_frame(sentence.trim()).expect("re-parse GMP");
    assert_eq!(Gmp::parse(&frame.fields), Some(gmp));
}
