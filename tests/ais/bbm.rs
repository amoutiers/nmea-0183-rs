#![cfg(feature = "bbm")]
use nmea_0183_rs::ais::sentences::{AisSentence, Bbm};
use nmea_0183_rs::parse_frame;

#[test]
fn bbm_values() {
    let frame = parse_frame("!AIBBM,26,2,1,3,8,177KQJ5000G?tO`K>RA1wUbN0TKH,0*2C").expect("valid");
    let bbm = Bbm::parse(&frame.fields);

    assert_eq!(bbm.num_frags, Some(26));
    assert_eq!(bbm.frag_num, Some(2));
    assert_eq!(bbm.msg_id, Some(1));
    assert_eq!(bbm.channel, Some('3'));
    assert_eq!(bbm.vdl_msg_num, Some(8));
    assert_eq!(bbm.payload.as_deref(), Some("177KQJ5000G?tO`K>RA1wUbN0TKH"));
    assert_eq!(bbm.fill_bits, Some(0));
}

#[test]
fn decode_encode() {
    let frame = parse_frame("!AIBBM,26,2,1,3,8,H77nSfPh4U=<E`H4U8G;:222220,2*6C").expect("valid");
    let bbm = Bbm::parse(&frame.fields);
    let sentence2 = bbm.to_sentence("AI").expect("encode");
    assert!(sentence2.starts_with("!AIBBM,"));
    let frame2 = parse_frame(sentence2.trim()).expect("re-parse");
    let bbm2 = Bbm::parse(&frame2.fields);
    assert_eq!(bbm, bbm2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("!AIBBM,26,2,1,3,8,,0*55").expect("valid");
    assert!(matches!(AisSentence::parse(&frame), AisSentence::Bbm(_)));
}

#[test]
fn roundtrip() {
    let original = Bbm {
        num_frags: Some(1),
        frag_num: Some(1),
        msg_id: Some(0),
        channel: Some('A'),
        vdl_msg_num: Some(6),
        payload: Some("testpayload".to_string()),
        fill_bits: Some(0),
    };
    let sentence = original.to_sentence("AI").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Bbm::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn bbm_default_and_strict_encoding() {
    use nmea_0183_rs::{ComplianceError, EncodeError, StrictEncodeError, parse_frame_strict};
    let value = Bbm::default();
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
    let empty: Bbm = Bbm::parse(&[]);
    assert_eq!(empty, Bbm::default());
    let malformed: Bbm = Bbm::parse(&["bad"]);
    assert_eq!(malformed, Bbm::default());
}
