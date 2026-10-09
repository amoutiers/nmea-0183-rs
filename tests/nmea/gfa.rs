#![cfg(feature = "gfa")]

use nmea_0183_rs::nmea::{NmeaEncodable, sentences::Gfa};
use nmea_0183_rs::{NmeaSentence, parse_frame};

const FIXTURE: &str = "$GNGFA,224229.00,0001.7,0002.9,000.43,000.22,014.4868,000.83,0010.0,SCC*0C";
// John Deere/NavCom Onyx Technical Reference Manual Rev. A, table 156, p. 293.
// https://www.deere.com/assets/pdfs/navcom/User%20Manuals/96-312008-3001%20RevA17i_Onyx%20TRM.pdf

fn assert_close(actual: Option<f32>, expected: f32) {
    assert!((actual.expect("value present") - expected).abs() < 0.001);
}

#[test]
fn decode_encode() {
    let frame = parse_frame(FIXTURE).expect("valid GFA frame");
    let original = Gfa::parse(&frame.fields);
    let sentence = original.to_sentence("GN").expect("encode GFA");
    let encoded = parse_frame(sentence.trim()).expect("re-parse GFA");
    assert_eq!(Gfa::parse(&encoded.fields), original);
}

#[test]
fn dispatch() {
    let frame = parse_frame(FIXTURE).expect("valid GFA frame");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gfa(_)));
}

#[test]
fn gfa_values_deere_navcom() {
    let frame = parse_frame(FIXTURE).expect("valid Deere/NavCom GFA frame");
    let gfa = Gfa::parse(&frame.fields);

    assert_eq!(gfa.time.as_deref(), Some("224229.00"));
    assert_close(gfa.horizontal_protection_level, 1.7);
    assert_close(gfa.vertical_protection_level, 2.9);
    assert_close(gfa.semi_major_std_dev, 0.43);
    assert_close(gfa.semi_minor_std_dev, 0.22);
    assert_close(gfa.semi_major_orientation, 14.4868);
    assert_close(gfa.altitude_std_dev, 0.83);
    assert_close(gfa.selected_accuracy_level, 10.0);
    assert_eq!(gfa.integrity_status.as_deref(), Some("SCC"));
}

#[test]
fn roundtrip() {
    let original = Gfa {
        time: Some("224229.00".into()),
        horizontal_protection_level: Some(1.7),
        vertical_protection_level: Some(2.9),
        semi_major_std_dev: Some(0.43),
        semi_minor_std_dev: Some(0.22),
        semi_major_orientation: Some(14.4868),
        altitude_std_dev: Some(0.83),
        selected_accuracy_level: Some(10.0),
        integrity_status: Some("SCC".into()),
    };

    let sentence = original.to_sentence("GN").expect("encode GFA");
    let frame = parse_frame(sentence.trim()).expect("re-parse GFA");
    assert_eq!(Gfa::parse(&frame.fields), original);
}
