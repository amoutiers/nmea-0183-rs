#![cfg(feature = "nmea")]
use nmea_0183_rs::nmea::{Dpt, Pashr, Rmc, Ttd};
use nmea_0183_rs::{ComplianceError, EncodeError, NmeaSentence, StrictEncodeError, parse_frame};

#[test]
fn partial_construction_and_enum_encoding() {
    let rmc = Rmc {
        sog: Some(2.5),
        ..Default::default()
    };
    assert_eq!(rmc.status, None);
    assert_eq!(nmea_0183_rs::nmea::Gsa::default().prns, [None; 12]);
    let value = NmeaSentence::Dpt(Dpt {
        depth: Some(4.1),
        ..Default::default()
    });
    let line = value.to_sentence_strict("SD").expect("encode enum");
    assert_eq!(
        NmeaSentence::parse(&parse_frame(&line).expect("frame")),
        value
    );
    assert_eq!(value.to_sentence("SD").expect("compatible encode"), line);
    assert!(matches!(
        value.to_sentence_strict("sd"),
        Err(StrictEncodeError::Compliance(
            ComplianceError::InvalidAddressCharacter('s')
        ))
    ));
    let invalid = NmeaSentence::Dpt(Dpt {
        depth: Some(f32::NAN),
        ..Default::default()
    });
    assert_eq!(
        invalid.to_sentence_strict("SD"),
        Err(StrictEncodeError::Encode(EncodeError::NonFiniteNumber))
    );
}

#[test]
fn unknown_preserves_its_owned_envelope() {
    for input in [
        "!AIXYZ,1,,",
        "$GPXYZ,1,,",
        "$PTEST,1,,",
        "\\s:receiver\\!AIXYZ,1,,",
        "\\\\$GPXYZ,1,,",
    ] {
        let value = {
            let owned = input.to_owned();
            NmeaSentence::parse(&parse_frame(&owned).expect("frame"))
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
    let value = NmeaSentence::parse(&parse_frame("$gpXYZ,1").expect("compatible"));
    assert!(value.to_sentence("GP").is_ok());
    assert!(matches!(
        value.to_sentence_strict("GP"),
        Err(StrictEncodeError::Compliance(_))
    ));
}

#[test]
fn enum_encoding_preserves_ttd_and_proprietary_addresses() {
    for (value, talker, prefix) in [
        (NmeaSentence::Ttd(Ttd::default()), "**", "!**TTD,"),
        (NmeaSentence::Pashr(Pashr::default()), "ignored", "$PASHR,"),
    ] {
        let line = value.to_sentence_strict(talker).expect("encode");
        assert!(line.starts_with(prefix));
        assert_eq!(
            NmeaSentence::parse(&parse_frame(&line).expect("frame")),
            value
        );
    }
}

#[test]
fn sentence_parsers_are_infallible_and_lenient() {
    use nmea_0183_rs::nmea::Dbt;
    let empty: Dbt = Dbt::parse(&[]);
    assert_eq!(empty, Dbt::default());
    let malformed: Dbt = Dbt::parse(&["bad", "f", "", "M", "", "F"]);
    assert_eq!(malformed, Dbt::default());
}

#[test]
fn unknown_rejects_invalid_tag() {
    let value = NmeaSentence::Unknown {
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
