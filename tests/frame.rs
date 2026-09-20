use nmea_0183_rs::{FrameError, encode_frame, parse_frame};

#[test]
fn ais_multi_fragment_fixture_signalk() {
    let frame = parse_frame(
        "!AIVDM,2,1,0,A,53brRt4000010SG700iE@LE8@Tp4000000000153P615t0Ht0SCkjH4jC1C,0*25",
    )
    .expect("AIS multi-fragment fixture");
    assert_eq!(frame.prefix, '!');
    assert_eq!(frame.sentence_type, "VDM");
    assert_eq!(frame.fields[1], "1"); // fragment number
}

#[test]
fn apb_fixture_signalk() {
    let frame = parse_frame("$GPAPB,A,A,0.10,R,N,V,V,011,M,DEST,011,M,011,M*3C")
        .expect("SignalK APB fixture");
    assert_eq!(frame.sentence_type, "APB");
    assert_eq!(frame.fields[9], "DEST");
}

#[test]
fn dbt_sounder_fixture_gpsd() {
    let frame = parse_frame("$SDDBT,7.7,f,2.3,M,1.3,F*05").expect("GPSD DBT sounder fixture");
    assert_eq!(frame.sentence_type, "DBT");
    assert_eq!(frame.fields[2], "2.3"); // meters
}

#[test]
fn dpt_fixtures_signalk() {
    let fixtures = [
        ("$IIDPT,4.1,0.0*45", "4.1", "0.0"),
        ("$IIDPT,4.1,1.0*44", "4.1", "1.0"),
        ("$IIDPT,4.1,-1.0*69", "4.1", "-1.0"),
    ];
    for (fix, depth, offset) in &fixtures {
        let frame = parse_frame(fix).unwrap_or_else(|e| panic!("failed to parse {fix}: {e}"));
        assert_eq!(frame.sentence_type, "DPT");
        assert_eq!(frame.fields[0], *depth);
        assert_eq!(frame.fields[1], *offset);
    }
}

#[test]
fn encode_ais_prefix() {
    let result =
        encode_frame('!', "AI", "VDM", &["1", "1", "", "A", "payload", "0"]).expect("valid");
    assert!(result.starts_with("!AIVDM,"));
    let frame = parse_frame(result.trim()).expect("re-parse AIS encoded");
    assert_eq!(frame.prefix, '!');
    assert_eq!(frame.sentence_type, "VDM");
}

#[test]
fn encode_no_fields() {
    let result = encode_frame('$', "GP", "RMC", &[]).expect("valid");
    assert!(result.starts_with("$GPRMC*"));
    assert!(result.ends_with("\r\n"));
}

#[test]
fn encode_rejects_address_characters() {
    for c in [
        ',', '*', '\r', '\n', '$', '!', '\\', '\0', ' ', '\t', '\u{7f}',
    ] {
        let talker = format!("G{c}P");
        let sentence_type = format!("RM{c}C");
        for prefix in ['$', '!'] {
            for fields in [&[][..], &["1"][..]] {
                assert!(
                    encode_frame(prefix, &talker, "RMC", fields).is_err(),
                    "talker {talker:?}"
                );
                assert!(
                    encode_frame(prefix, "GP", &sentence_type, fields).is_err(),
                    "type {sentence_type:?}"
                );
            }
        }
    }
    assert!(encode_frame('$', "**", "TTD", &[]).is_err());
    assert!(encode_frame('!', "**", "VDM", &[]).is_err());
}

#[test]
fn encode_rejects_address_injection_gpsd() {
    let fixture = "$SDDBT,7.7,f,2.3,M,1.3,F*05";
    parse_frame(fixture).expect("valid GPSD fixture checksum");
    let address = format!("{}\r\n$GP", fixture.strip_prefix('$').expect("prefix"));
    assert!(encode_frame('$', &address, "RMC", &[]).is_err());
    assert!(encode_frame('$', "", &address, &[]).is_err());
}

#[test]
fn encode_supported_addresses_roundtrip() {
    for (prefix, talker, sentence_type) in [
        ('$', "GP", "RMC"),
        ('$', "04", "HDM"),
        ('$', "gp", "rmc"),
        ('$', "", "PMTK001"),
        ('$', "", "RMC"),
        ('!', "**", "TTD"),
    ] {
        let encoded = encode_frame(prefix, talker, sentence_type, &["1"]).expect("encode");
        let frame = parse_frame(&encoded).expect("valid checksum");
        assert_eq!(
            (frame.prefix, frame.talker, frame.sentence_type),
            (prefix, talker, sentence_type)
        );
        assert_eq!(frame.fields, ["1"]);
    }
}

#[test]
#[cfg(any(feature = "dbt", feature = "abm", feature = "bbm"))]
fn encode_typed_rejects_address_injection() {
    let talker = "AI\r\n$GP";
    #[cfg(feature = "dbt")]
    {
        use nmea_0183_rs::NmeaEncodable;
        let sentence = nmea_0183_rs::nmea::sentences::Dbt::parse(&[]).expect("parse");
        assert!(sentence.to_sentence(talker).is_err());
    }
    #[cfg(feature = "abm")]
    {
        let sentence = nmea_0183_rs::ais::sentences::Abm::parse(&[]).expect("parse");
        assert!(sentence.to_sentence(talker).is_err());
    }
    #[cfg(feature = "bbm")]
    {
        let sentence = nmea_0183_rs::ais::sentences::Bbm::parse(&[]).expect("parse");
        assert!(sentence.to_sentence(talker).is_err());
    }
}

#[test]
fn encode_then_parse_roundtrip() {
    let encoded = encode_frame(
        '$',
        "WI",
        "MWD",
        &["270.0", "T", "268.5", "M", "12.4", "N", "6.4", "M"],
    )
    .expect("valid");
    assert!(encoded.starts_with("$WIMWD,"));
    assert!(encoded.ends_with("\r\n"));

    let frame = parse_frame(encoded.trim()).expect("re-parse encoded sentence");
    assert_eq!(frame.talker, "WI");
    assert_eq!(frame.sentence_type, "MWD");
    assert_eq!(frame.fields.len(), 8);
    assert_eq!(frame.fields[0], "270.0");
    assert_eq!(frame.fields[7], "M");
}

#[test]
fn error_bad_checksum() {
    let result = parse_frame("$GPRMC,175957.917,A*FF");
    assert!(
        matches!(result, Err(FrameError::BadChecksum { .. })),
        "expected BadChecksum, got {result:?}"
    );
}

#[test]
fn error_empty_input() {
    assert_eq!(parse_frame(""), Err(FrameError::Empty));
    assert_eq!(parse_frame("   "), Err(FrameError::Empty));
}

#[test]
fn error_invalid_prefix() {
    let result = parse_frame("GPRMC,175957.917,A*00");
    assert!(
        matches!(result, Err(FrameError::InvalidPrefix(_))),
        "expected InvalidPrefix, got {result:?}"
    );
}

#[test]
fn hdt_fixture_gpsd() {
    let frame = parse_frame("$HEHDT,4.0,T*2B").expect("GPSD HDT saab-r4 fixture");
    assert_eq!(frame.talker, "HE");
    assert_eq!(frame.sentence_type, "HDT");
    assert_eq!(frame.fields[0], "4.0");
    assert_eq!(frame.fields[1], "T");
}

#[test]
fn no_checksum_sentence_accepted() {
    let result = parse_frame("$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A");
    assert!(
        result.is_ok(),
        "sentence without checksum should be accepted"
    );
    let frame = result.expect("valid frame");
    assert_eq!(frame.sentence_type, "RMC");
    assert_eq!(frame.fields[0], "175957.917");
}

#[test]
fn parse_valid_ais_sentence() {
    let frame =
        parse_frame("!AIVDM,1,1,,A,13u@Dt002s000000000000000000,0*60").expect("valid AIS sentence");
    assert_eq!(frame.prefix, '!');
    assert_eq!(frame.talker, "AI");
    assert_eq!(frame.sentence_type, "VDM");
    assert_eq!(frame.fields[0], "1");
    assert_eq!(frame.fields[1], "1");
}

#[test]
fn parse_valid_nmea_sentence() {
    let frame = parse_frame("$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*77")
        .expect("valid NMEA sentence");
    assert_eq!(frame.prefix, '$');
    assert_eq!(frame.talker, "GP");
    assert_eq!(frame.sentence_type, "RMC");
    assert_eq!(frame.fields[0], "175957.917");
    assert_eq!(frame.fields[1], "A");
    assert!(frame.tag_block.is_none());
}

#[test]
fn parse_with_tag_block() {
    let content = "s:FooBar,c:1234567890";
    let checksum = content.bytes().fold(0u8, |acc, byte| acc ^ byte);
    let line = format!(
        "\\{content}*{checksum:02X}\\$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*77"
    );
    let frame = parse_frame(&line).expect("sentence with tag block");
    assert!(frame.tag_block.is_some());
    let tag = frame.tag_block.expect("tag_block present");
    assert!(tag.contains("FooBar"));
    assert!(!tag.contains('*'));
    assert_eq!(frame.prefix, '$');
    assert_eq!(frame.sentence_type, "RMC");
}

#[test]
fn roundtrip_preserves_empty_fields() {
    let encoded = encode_frame('$', "GP", "APB", &["", "", "", "", "", "", ""]).expect("valid");
    let frame = parse_frame(encoded.trim()).expect("re-parse with empty fields");
    assert_eq!(frame.sentence_type, "APB");
    assert!(
        frame.fields.iter().all(|f| f.is_empty()),
        "all fields should be empty"
    );
}

#[test]
fn wind_fixture_signalk() {
    let frame = parse_frame("$WIMWD,270.0,T,268.5,M,12.4,N,6.4,M*63").expect("SignalK MWD fixture");
    assert_eq!(frame.talker, "WI");
    assert_eq!(frame.sentence_type, "MWD");
    assert_eq!(frame.fields[0], "270.0");
    assert_eq!(frame.fields[4], "12.4");
}

#[test]
fn encode_rejects_unparseable_address_lengths() {
    use nmea_0183_rs::{EncodeError, encode_frame};
    for (talker, kind, actual) in [("", "X", 1), ("", "XY", 2), ("", "PXX", 3), ("P", "XX", 3)] {
        assert_eq!(
            encode_frame('$', talker, kind, &[]),
            Err(EncodeError::InvalidAddressLength { actual })
        );
    }
    for (prefix, talker, kind) in [
        ('$', "", "RMC"),
        ('$', "", "PASHR"),
        ('$', "GP", "RMC"),
        ('!', "**", "TTD"),
        ('$', "gp", "rmc"),
    ] {
        let line = encode_frame(prefix, talker, kind, &[]).expect("encode");
        assert!(parse_frame(&line).is_ok());
    }
}

#[test]
fn frame_reencoding_preserves_unknown_envelope() {
    use nmea_0183_rs::NmeaFrame;
    for fields in [vec![], vec![""], vec!["1", "", "3", ""]] {
        for tag_block in [None, Some(""), Some("s:receiver,c:123")] {
            let original = NmeaFrame {
                prefix: '!',
                talker: "AI",
                sentence_type: "XYZ",
                fields: fields.clone(),
                tag_block,
            };
            let line = original.to_sentence().expect("encode frame");
            assert_eq!(parse_frame(&line).expect("parse frame"), original);
            assert!(line.ends_with("\r\n"));
            if tag_block == Some("s:receiver,c:123") {
                assert!(line.starts_with("\\s:receiver,c:123*"));
            }
        }
    }
    let original = parse_frame("\\s:receiver\\$GPXYZ,1,").expect("compatible frame");
    let line = original.to_sentence().expect("encode");
    assert!(line.starts_with("\\s:receiver*"));
    assert_eq!(parse_frame(&line).expect("reparse"), original);
}

#[test]
fn frame_reencoding_rejects_injected_tag_characters() {
    use nmea_0183_rs::{EncodeError, NmeaFrame};
    for (tag, invalid) in [
        ("s:a\\b", '\\'),
        ("s:a*b", '*'),
        ("s:a\nb", '\n'),
        ("s:a\rb", '\r'),
        ("s:a\0b", '\0'),
        ("s:é", 'é'),
        ("s:\u{7f}", '\u{7f}'),
    ] {
        let original = NmeaFrame {
            prefix: '$',
            talker: "GP",
            sentence_type: "XYZ",
            fields: vec!["1"],
            tag_block: Some(tag),
        };
        assert_eq!(
            original.to_sentence(),
            Err(EncodeError::InvalidTagBlockCharacter(invalid))
        );
    }
}

#[test]
fn frame_reencoding_validates_fields_and_keeps_proprietary_address() {
    use nmea_0183_rs::{EncodeError, NmeaFrame};
    let proprietary = parse_frame("$PASHR,1,").expect("frame");
    let line = proprietary.to_sentence().expect("encode");
    assert!(line.starts_with("$PASHR,"));
    assert_eq!(parse_frame(&line).expect("reparse"), proprietary);
    let invalid = NmeaFrame {
        fields: vec!["injected,field"],
        ..proprietary
    };
    assert_eq!(
        invalid.to_sentence(),
        Err(EncodeError::InvalidFieldCharacter(','))
    );
}
