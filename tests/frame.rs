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
        let sentence = nmea_0183_rs::nmea::sentences::Dbt::parse(&[]);
        assert!(sentence.to_sentence(talker).is_err());
    }
    #[cfg(feature = "abm")]
    {
        let sentence = nmea_0183_rs::ais::sentences::Abm::parse(&[]);
        assert!(sentence.to_sentence(talker).is_err());
    }
    #[cfg(feature = "bbm")]
    {
        let sentence = nmea_0183_rs::ais::sentences::Bbm::parse(&[]);
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

#[test]
fn encode_accepts_minimum_proprietary_address() {
    let wire = encode_frame('$', "", "PABC", &["1"]).expect("minimum proprietary address");
    assert_eq!(wire, "$PABC,1*0D\r\n");
    let frame = parse_frame(&wire).expect("reparse");
    assert_eq!(frame.talker, "");
    assert_eq!(frame.sentence_type, "PABC");
    assert_eq!(frame.fields, ["1"]);
}

#[test]
fn public_errors_have_informative_display() {
    let data = FrameError::NonAsciiData.to_string();
    assert!(data.contains("data"));
    assert!(data.contains("non-ASCII"));
    use nmea_0183_rs::EncodeError;
    let checksum = FrameError::BadChecksum {
        expected: 0x12,
        computed: 0x34,
    }
    .to_string();
    assert!(checksum.contains("12"));
    assert!(checksum.contains("34"));
    assert!(
        EncodeError::InvalidAisField("communication_state")
            .to_string()
            .contains("communication_state")
    );
}

// Synthetic framing counterexamples; XOR the raw UTF-8 bytes independently.
fn ascii_test_sentence(body: &str) -> String {
    let checksum = body[1..].bytes().fold(0u8, |acc, byte| acc ^ byte);
    format!("{body}*{checksum:02X}\r\n")
}

#[test]
fn frame_rejects_non_ascii_data() {
    for address in ["$GPTXT", "!AIVDM", "$PABC", "$GPXYZ"] {
        for value in ["é", "漢", "🙂", "e\u{301}", "\u{80}"] {
            for fields in [
                format!("{value},A,B"),
                format!("A,{value},B"),
                format!("A,B,{value}"),
            ] {
                let body = format!("{address},{fields}");
                for line in [body.clone(), ascii_test_sentence(&body)] {
                    assert_eq!(
                        parse_frame(&line),
                        Err(FrameError::NonAsciiData),
                        "{line:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn frame_rejects_unicode_trailing_whitespace() {
    for whitespace in ['\u{a0}', '\u{2003}', '\u{202f}', '\u{3000}', '\u{85}'] {
        for value in [whitespace.to_string(), format!("A{whitespace}")] {
            for ending in ["", "\r\n", " \t\r\n"] {
                let body = format!("$GPXYZ,{value}{ending}");
                for line in [body.clone(), ascii_test_sentence(&body)] {
                    assert_eq!(
                        parse_frame(&line),
                        Err(FrameError::NonAsciiData),
                        "{line:?}"
                    );
                }
            }
        }
    }
    for body in ["$GPXYZ,A\u{a0},", "$GPXYZ,A\r\n\u{a0}"] {
        assert_eq!(parse_frame(body), Err(FrameError::NonAsciiData), "{body:?}");
    }
}

#[test]
fn frame_keeps_ascii_fields_and_empty_fields() {
    for (body, expected) in [
        ("$GPXYZ", vec![]),
        ("$GPXYZ,", vec![""]),
        ("$GPXYZ,,", vec!["", ""]),
        ("$GPXYZ,A,,B,", vec!["A", "", "B", ""]),
    ] {
        for line in [
            body.to_string(),
            format!("{body}\r\n"),
            ascii_test_sentence(body),
            ascii_test_sentence(body).trim_end().to_string(),
        ] {
            assert_eq!(
                parse_frame(&line).expect("ASCII frame").fields,
                expected,
                "{line:?}"
            );
        }
    }
}

#[test]
fn frame_keeps_compatible_ascii_data() {
    let fields = ["raw$!\\~ text", "A\0B", "A\tB", "A\rB", "A\nB", "A\u{7f}B"];
    let body = format!("$GPXYZ,{}", fields.join(","));
    for line in [body.clone(), ascii_test_sentence(&body)] {
        assert_eq!(parse_frame(&line).expect("compatible ASCII").fields, fields);
    }
    assert_eq!(
        parse_frame(" \t$GPXYZ,A \t\r\n")
            .expect("ASCII trim")
            .fields,
        ["A"]
    );
    let body = "$GPXYZ,raw*text";
    assert_eq!(parse_frame(body), Err(FrameError::MalformedChecksum));
    assert_eq!(
        parse_frame(&ascii_test_sentence(body))
            .expect("last star")
            .fields,
        ["raw*text"]
    );
}

#[test]
fn frame_keeps_ascii_escapes_verbatim() {
    let fields = ["^2C", "^E9", "^00", "^7F", "^GG", "^2c", "^", "^2"];
    let body = format!("$GPXYZ,{}", fields.join(","));
    for line in [body.clone(), ascii_test_sentence(&body)] {
        assert_eq!(parse_frame(&line).expect("literal escapes").fields, fields);
    }
}

#[test]
fn frame_keeps_permissive_envelope() {
    for (address, talker, sentence_type) in [
        ("$RMC", "", "RMC"),
        ("$gpRMC", "gp", "RMC"),
        ("$GPRMCX", "GPR", "MCX"),
        ("$PABC", "", "PABC"),
    ] {
        let body = format!("{address},A");
        for line in [body.clone(), ascii_test_sentence(&body)] {
            let frame = parse_frame(&line).expect("noncanonical ASCII address");
            assert_eq!((frame.talker, frame.sentence_type), (talker, sentence_type));
            assert_eq!(frame.fields, ["A"]);
        }
    }
    let wire = ascii_test_sentence("!**TTD,1,1,0,ABC,0");
    let frame = parse_frame(&wire).expect("TTD address with trailing checksum");
    assert_eq!(
        (frame.prefix, frame.talker, frame.sentence_type),
        ('!', "**", "TTD")
    );
    assert_eq!(
        parse_frame("$GPRMC,9*5e")
            .expect("lowercase checksum")
            .fields,
        ["9"]
    );
    let body = format!("$GPXYZ,{}", "A".repeat(90));
    assert_eq!(
        parse_frame(&ascii_test_sentence(&body))
            .expect("over 82 bytes")
            .fields,
        ["A".repeat(90)]
    );
}

#[test]
fn frame_keeps_whitespace_outside_data() {
    assert_eq!(
        parse_frame("\u{a0}$GPXYZ,A")
            .expect("leading Unicode whitespace")
            .fields,
        ["A"]
    );
    assert_eq!(parse_frame("\u{a0}"), Err(FrameError::Empty));
    assert!(
        parse_frame("$GPXYZ\u{a0}")
            .expect("fieldless address")
            .fields
            .is_empty()
    );
    assert_eq!(parse_frame("\\s:é\\\u{a0}"), Err(FrameError::TooShort));
    for ending in ["\u{a0}", "\r\n\u{a0}"] {
        assert_eq!(
            parse_frame(&format!("$GPRMC,9*5E{ending}"))
                .expect("after checksum")
                .fields,
            ["9"]
        );
    }
}

#[test]
fn frame_preserves_error_precedence() {
    let body = "$GPXYZ,é";
    let computed = body[1..].bytes().fold(0u8, |acc, byte| acc ^ byte);
    let expected = computed ^ 0xff;
    assert_eq!(
        parse_frame(&format!("{body}*{expected:02X}")),
        Err(FrameError::BadChecksum { expected, computed })
    );
    assert_eq!(
        parse_frame(&format!("{body}*GG")),
        Err(FrameError::MalformedChecksum)
    );
    assert_eq!(
        parse_frame(&ascii_test_sentence(body)),
        Err(FrameError::NonAsciiData)
    );
    assert_eq!(parse_frame(body), Err(FrameError::NonAsciiData));
    assert_eq!(parse_frame("$GP,é\u{a0}"), Err(FrameError::TooShort));
    assert_eq!(parse_frame("$Aé,é\u{a0}"), Err(FrameError::NonAsciiAddress));
    assert_eq!(
        parse_frame("XGPXYZ,é\u{a0}"),
        Err(FrameError::InvalidPrefix('X'))
    );
    assert_eq!(
        parse_frame("\\s:tag$GPXYZ,é\u{a0}"),
        Err(FrameError::MalformedTagBlock)
    );
}

#[test]
fn frame_keeps_tag_block_contract() {
    for content in ["s:é", "s:A\tB"] {
        let computed = content.bytes().fold(0u8, |acc, byte| acc ^ byte);
        for tag_checksum in [false, true] {
            let tag = if tag_checksum {
                format!("\\{content}*{computed:02X}\\")
            } else {
                format!("\\{content}\\")
            };
            for sentence_checksum in [false, true] {
                for (body, unicode) in [("$GPXYZ,A", false), ("$GPXYZ,é\u{a0}", true)] {
                    let sentence = if sentence_checksum {
                        ascii_test_sentence(body)
                    } else {
                        body.to_string()
                    };
                    let line = format!("{tag}{sentence}");
                    if unicode {
                        assert_eq!(
                            parse_frame(&line),
                            Err(FrameError::NonAsciiData),
                            "{line:?}"
                        );
                    } else {
                        let frame = parse_frame(&line).expect("permissive tag content");
                        assert_eq!(frame.tag_block, Some(content));
                        assert_eq!(frame.fields, ["A"]);
                    }
                }
            }
        }
        let expected = computed ^ 0xff;
        assert_eq!(
            parse_frame(&format!("\\{content}*{expected:02X}\\$GPXYZ,é")),
            Err(FrameError::BadTagChecksum { expected, computed })
        );
        assert_eq!(
            parse_frame(&format!("\\{content}*GG\\$GPXYZ,é")),
            Err(FrameError::MalformedTagBlock)
        );
    }
}
