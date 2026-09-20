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
    use nmea_0183_rs::{ComplianceError, StrictEncodeError, parse_frame_strict};
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
}

#[test]
fn parser_is_infallible_and_lenient() {
    let empty: Bbm = Bbm::parse(&[]);
    assert_eq!(empty, Bbm::default());
    let malformed: Bbm = Bbm::parse(&["bad"]);
    assert_eq!(malformed, Bbm::default());
}

#[test]
fn unknown_preserves_its_owned_envelope() {
    for input in [
        "!AIXYZ,1,,",
        "$GPXYZ,1,,",
        "$PTEST,1,,",
        "\\s:receiver\\!AIXYZ,1,,",
        "\\\\!AIXYZ,1,,",
    ] {
        let value = {
            let owned = input.to_owned();
            AisSentence::parse(&parse_frame(&owned).expect("frame"))
        };
        let line = value.to_sentence("ignored").expect("unknown encode");
        assert_eq!(
            parse_frame(&line).expect("reparse"),
            parse_frame(input).expect("original")
        );
        assert_eq!(
            value.to_sentence_strict("ignored").expect("strict unknown"),
            line
        );
    }
    let value = AisSentence::parse(&parse_frame("!aiXYZ,1").expect("compatible"));
    assert!(value.to_sentence("AI").is_ok());
    assert!(matches!(
        value.to_sentence_strict("AI"),
        Err(nmea_0183_rs::StrictEncodeError::Compliance(_))
    ));
}

#[test]
fn unknown_rejects_invalid_tag() {
    let value = AisSentence::Unknown {
        prefix: '!',
        talker: "AI".to_owned(),
        sentence_type: "XYZ".to_owned(),
        fields: vec!["1".to_owned()],
        tag_block: Some("s:bad*tag".to_owned()),
    };
    assert_eq!(
        value.to_sentence("ignored"),
        Err(nmea_0183_rs::EncodeError::InvalidTagBlockCharacter('*'))
    );
}

#[test]
fn audit_channel_requires_exactly_one_character() {
    for (channel, expected) in [
        ("", None),
        ("12", None),
        ("AB", None),
        ("é1", None),
        ("1", Some('1')),
        ("3", Some('3')),
        ("A", Some('A')),
    ] {
        let value = Bbm::parse(&["1", "1", "0", channel, "6", "payload", "0"]);
        assert_eq!(value.channel, expected, "channel {channel:?}");
        assert_eq!(value.vdl_msg_num, Some(6));
        let encoded = value.to_sentence("AI").expect("encode");
        let frame = parse_frame(&encoded).expect("frame");
        assert_eq!(Bbm::parse(&frame.fields), value);
    }
}
