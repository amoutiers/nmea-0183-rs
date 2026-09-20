#![cfg(feature = "rte")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Rte;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$IIRTE,4,1,c,Rte 1,411,412,413,414,415*6F").expect("valid");
    let rte = Rte::parse(&frame.fields);
    let sentence = rte.to_sentence("II").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let rte2 = Rte::parse(&frame2.fields);
    assert_eq!(rte, rte2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$IIRTE,4,1,c,Rte 1,411,412,413,414,415*6F").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rte(_)));
}

#[test]
fn roundtrip() {
    let original = Rte {
        num_sentences: Some(4),
        sentence_num: Some(1),
        mode: Some('c'),
        name: Some("Rte 1".to_string()),
        idents: vec![
            "411".to_string(),
            "412".to_string(),
            "413".to_string(),
            "414".to_string(),
            "415".to_string(),
        ],
    };
    let sentence = original.to_sentence("II").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Rte::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn rte_values() {
    let frame = parse_frame("$IIRTE,4,1,c,Rte 1,411,412,413,414,415*6F").expect("valid RTE frame");
    let x = Rte::parse(&frame.fields);
    assert_eq!(x.num_sentences, Some(4));
    assert_eq!(x.sentence_num, Some(1));
    assert_eq!(x.mode, Some('c'));
    assert_eq!(x.name.as_deref(), Some("Rte 1"));
    assert_eq!(
        x.idents,
        vec![
            "411".to_string(),
            "412".to_string(),
            "413".to_string(),
            "414".to_string(),
            "415".to_string(),
        ]
    );
}

#[test]
fn roundtrip_preserves_empty_waypoint_positions() {
    for idents in [
        vec![],
        vec![""],
        vec!["", "B"],
        vec!["A", "", "B"],
        vec!["A", "", "", "B", ""],
    ] {
        let original = Rte {
            num_sentences: Some(1),
            sentence_num: Some(1),
            mode: Some('c'),
            name: Some("R".to_string()),
            idents: idents.into_iter().map(str::to_owned).collect(),
        };
        let line = original.to_sentence("GP").expect("encode route");
        let parsed = Rte::parse(&parse_frame(&line).expect("frame").fields);
        assert_eq!(parsed, original);
    }
}
