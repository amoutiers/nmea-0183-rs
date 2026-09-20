#![cfg(feature = "abm")]
use nmea_0183_rs::ais::sentences::{Abm, AisSentence};
use nmea_0183_rs::parse_frame;

#[test]
fn abm_values() {
    let frame = parse_frame("!AIABM,26,2,1,3381581370,3,8,177KQJ5000G?tO`K>RA1wUbN0TKH,0*02")
        .expect("valid");
    let abm = Abm::parse(&frame.fields).expect("parse");

    assert_eq!(abm.num_frags, Some(26));
    assert_eq!(abm.frag_num, Some(2));
    assert_eq!(abm.msg_id, Some(1));
    assert_eq!(abm.mmsi, Some(3381581370));
    assert_eq!(abm.channel, Some('3'));
    assert_eq!(abm.vdl_msg_num, Some(8));
    assert_eq!(abm.payload.as_deref(), Some("177KQJ5000G?tO`K>RA1wUbN0TKH"));
    assert_eq!(abm.fill_bits, Some(0));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("!AIABM,26,2,1,3381581370,3,8,177KQJ5000G?tO`K>RA1wUbN0TKH,0*02")
        .expect("valid");
    let abm = Abm::parse(&frame.fields).expect("parse");
    let sentence2 = abm.to_sentence("AI").expect("encode");
    assert!(sentence2.starts_with("!AIABM,"));
    let frame2 = parse_frame(sentence2.trim()).expect("re-parse");
    let abm2 = Abm::parse(&frame2.fields).expect("parse");
    assert_eq!(abm, abm2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("!AIABM,26,2,1,3381581370,3,8,177KQJ5000G?tO`K>RA1wUbN0TKH,0*02")
        .expect("valid");
    assert!(matches!(AisSentence::parse(&frame), AisSentence::Abm(_)));
}

#[test]
fn dispatch_rejects_dollar_prefixed_abm() {
    let frame = nmea_0183_rs::NmeaFrame {
        prefix: '$',
        talker: "AI",
        sentence_type: "ABM",
        fields: vec![],
        tag_block: None,
    };
    assert!(matches!(
        AisSentence::parse(&frame),
        AisSentence::Unknown { .. }
    ));
}

#[test]
fn roundtrip() {
    let original = Abm {
        num_frags: Some(1),
        frag_num: Some(1),
        msg_id: Some(0),
        mmsi: Some(123456789),
        channel: Some('1'),
        vdl_msg_num: Some(6),
        payload: Some("testpayload".to_string()),
        fill_bits: Some(0),
    };
    let sentence = original.to_sentence("AI").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Abm::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}
