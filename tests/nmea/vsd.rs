#![cfg(feature = "vsd")]

use nmea_0183_rs::nmea::sentences::Vsd;
use nmea_0183_rs::{NmeaEncodable, NmeaFrame, NmeaSentence, parse_frame};

const RAW_VSD: &str = "$RAVSD,0,4.5,6,@@@@@@@@@@@@@@@@@@@@,220516,01,02,8,*6E";

#[test]
fn dispatch() {
    let frame = parse_frame(RAW_VSD).expect("valid VSD frame");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vsd(_)));
}

#[test]
fn decode_encode() {
    let frame = parse_frame(RAW_VSD).expect("valid VSD frame");
    let vsd = Vsd::parse(&frame.fields).expect("parse VSD");
    let encoded = vsd.to_sentence("RA").expect("encode VSD");
    assert!(encoded.starts_with("$RAVSD,"));
    let reparsed = parse_frame(encoded.trim()).expect("re-parse VSD");
    assert_eq!(Vsd::parse(&reparsed.fields), Some(vsd));
}

#[test]
fn nmea_dispatch_rejects_ais_prefix() {
    let frame = NmeaFrame {
        prefix: '!',
        talker: "AI",
        sentence_type: "MWD",
        fields: vec!["270.0", "T", "", "M", "10.0", "N", "", "M"],
        tag_block: None,
    };
    assert!(matches!(
        NmeaSentence::parse(&frame),
        NmeaSentence::Unknown { .. }
    ));
}

#[test]
fn roundtrip() {
    let original = Vsd {
        type_of_ship: Some(0),
        draught: Some(4.5),
        persons: Some(6),
        destination: Some("PORT".to_string()),
        arrival_time: Some("220516".to_string()),
        arrival_day: Some(1),
        arrival_month: Some(2),
        nav_status: Some(8),
        regional: None,
    };
    let wire = original.to_sentence("AI").expect("encode VSD");
    assert!(wire.starts_with("$AIVSD,"));
    let frame = parse_frame(wire.trim()).expect("re-parse VSD");
    assert_eq!(Vsd::parse(&frame.fields), Some(original));
}

#[test]
fn vsd_values() {
    let frame = parse_frame(RAW_VSD).expect("valid VSD frame");
    let vsd = Vsd::parse(&frame.fields).expect("parse VSD");
    assert_eq!(vsd.type_of_ship, Some(0));
    assert!((vsd.draught.expect("draught") - 4.5).abs() < 0.0001);
    assert_eq!(vsd.persons, Some(6));
    assert_eq!(vsd.destination.as_deref(), Some("@@@@@@@@@@@@@@@@@@@@"));
    assert_eq!(vsd.arrival_time.as_deref(), Some("220516"));
    assert_eq!(vsd.arrival_day, Some(1));
    assert_eq!(vsd.arrival_month, Some(2));
    assert_eq!(vsd.nav_status, Some(8));
    assert!(vsd.regional.is_none());
}

#[test]
fn preserves_full_protocol_person_count_range() {
    for expected in [0_u32, 255, 256, 300, 8191] {
        let persons = expected.to_string();
        let fields = [
            "60",
            "4.5",
            persons.as_str(),
            "PORT",
            "220516",
            "1",
            "2",
            "0",
            "",
        ];
        let parsed = Vsd::parse(&fields).expect("parse VSD");
        assert_eq!(parsed.persons.map(u32::from), Some(expected));
        let encoded = parsed.encode().expect("encode VSD");
        assert_eq!(encoded[2], persons);
    }

    let missing = Vsd::parse(&["60", "4.5", ""]).expect("parse missing persons");
    assert_eq!(missing.persons, None);
    let malformed = Vsd::parse(&["60", "4.5", "not-a-number"]).expect("parse malformed persons");
    assert_eq!(malformed.persons, None);
}
