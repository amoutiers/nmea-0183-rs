#![cfg(feature = "abm")]
use nmea_0183_rs::ais::sentences::{Abm, AisSentence};
use nmea_0183_rs::parse_frame;

#[test]
fn abm_values() {
    let frame = parse_frame("!AIABM,26,2,1,3381581370,3,8,177KQJ5000G?tO`K>RA1wUbN0TKH,0*02")
        .expect("valid");
    let abm = Abm::parse(&frame.fields);

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
    let abm = Abm::parse(&frame.fields);
    let sentence2 = abm.to_sentence("AI").expect("encode");
    assert!(sentence2.starts_with("!AIABM,"));
    let frame2 = parse_frame(sentence2.trim()).expect("re-parse");
    let abm2 = Abm::parse(&frame2.fields);
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
    let parsed = Abm::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn abm_default_and_strict_encoding() {
    use nmea_0183_rs::{ComplianceError, EncodeError, StrictEncodeError, parse_frame_strict};
    let value = Abm::default();
    let line = value.to_sentence_strict("AI").expect("strict envelope");
    assert!(parse_frame_strict(&line).is_ok());
    let sentence = AisSentence::parse(&parse_frame(&line).expect("frame"));
    assert_eq!(
        sentence.to_sentence_strict("AI").expect("enum encode"),
        line
    );
    assert_eq!(sentence.to_sentence("AI").expect("compatible enum"), line);
    assert!(matches!(
        sentence.to_sentence_strict("ai"),
        Err(StrictEncodeError::Compliance(
            ComplianceError::InvalidAddressCharacter('a')
        ))
    ));
    let unknown = AisSentence::parse(&parse_frame("!AIXYZ,1").expect("unknown"));
    assert_eq!(
        unknown.to_sentence("AI"),
        Err(EncodeError::MissingFrameContext)
    );
    assert_eq!(
        unknown.to_sentence_strict("AI"),
        Err(StrictEncodeError::Encode(EncodeError::MissingFrameContext))
    );
}

#[test]
fn parser_is_infallible_and_lenient() {
    let empty: Abm = Abm::parse(&[]);
    assert_eq!(empty, Abm::default());
    let malformed: Abm = Abm::parse(&["bad"]);
    assert_eq!(malformed, Abm::default());
}
