use nmea_0183_rs::{
    ComplianceError, FrameError, parse_frame, parse_frame_strict, validate_sentence,
};

fn sentence(body: &str) -> String {
    assert!(matches!(body.as_bytes().first(), Some(b'$' | b'!')));
    let checksum = body[1..].bytes().fold(0u8, |acc, byte| acc ^ byte);
    format!("{body}*{checksum:02X}\r\n")
}

fn tagged(content: &str, line: &str) -> String {
    let checksum = content.bytes().fold(0u8, |acc, byte| acc ^ byte);
    format!(r"\{content}*{checksum:02X}\{line}")
}

#[test]
fn strict_accepts_standard_query_proprietary_and_escaped_data() {
    for line in [
        sentence("$GPRMC,120000.00,A"),
        sentence("$CCGPQ,GGA"),
        sentence("$PABC,VALUE^2CWITH^24ESCAPES"),
        sentence("$PABCvendor,VALUE"),
        sentence("!AIVDM,1,1,,A,15N:;R0P00PD;88MD5MTDwwP2<0l,0"),
        sentence("!**TTD,1,1,0,ABC,0"),
    ] {
        validate_sentence(&line).expect("strictly valid sentence");
        parse_frame_strict(&line).expect("strict parse");
    }
}

#[test]
fn strict_length_excludes_tag_block() {
    let line = sentence(&format!("$GPTXT,01,01,01,{}", "A".repeat(61)));
    assert_eq!(line.len(), 82);
    let tagged = tagged("s:SOURCE", &line);
    assert!(tagged.len() > 82);
    validate_sentence(&tagged).expect("tag block excluded from sentence length");
}

#[test]
fn strict_rejects_missing_crlf_checksum_and_uppercase_violation() {
    assert_eq!(
        validate_sentence("$GPRMC,A*26"),
        Err(ComplianceError::MissingTerminator)
    );
    assert_eq!(
        validate_sentence("$GPRMC,A\r\n"),
        Err(ComplianceError::MissingChecksum)
    );
    assert_eq!(
        validate_sentence("$GPRMC,9*5e\r\n"),
        Err(ComplianceError::LowercaseChecksum)
    );
    assert!(matches!(
        validate_sentence(&sentence("$gpRMC,A")),
        Err(ComplianceError::InvalidAddressCharacter(_))
    ));
}

#[test]
fn strict_rejects_leading_and_trailing_whitespace() {
    let valid = sentence("$GPRMC,A");
    assert_eq!(
        validate_sentence(&format!(" {valid}")),
        Err(ComplianceError::InvalidPrefix(' '))
    );
    assert_eq!(
        validate_sentence(&format!("{} \r\n", valid.trim_end())),
        Err(ComplianceError::Frame(FrameError::MalformedChecksum))
    );
}

#[test]
fn strict_rejects_overlong_fixture_but_lenient_parser_keeps_it() {
    let fixture = "$GPGGA,172814.0,3723.46587704,N,12202.26957864,W,2,6,1.2,18.893,M,-25.669,M,2.0,0031*4F\r\n";
    assert!(parse_frame(fixture).is_ok());
    assert_eq!(
        validate_sentence(fixture),
        Err(ComplianceError::SentenceTooLong {
            actual: fixture.len(),
            maximum: 82,
        })
    );
}

#[test]
fn strict_accepts_exactly_82_and_rejects_83_bytes() {
    let exact = sentence(&format!("$GPTXT,01,01,01,{}", "A".repeat(61)));
    assert_eq!(exact.len(), 82);
    validate_sentence(&exact).expect("82 bytes");

    let over = sentence(&format!("$GPTXT,01,01,01,{}", "A".repeat(62)));
    assert_eq!(over.len(), 83);
    assert_eq!(
        validate_sentence(&over),
        Err(ComplianceError::SentenceTooLong {
            actual: 83,
            maximum: 82,
        })
    );
}

#[test]
fn strict_rejects_invalid_address_shapes() {
    for body in ["$RMC,A", "$GPRMCX,A", "$gpRMC,A", "$PabC,A", "$P12,A"] {
        let line = sentence(body);
        assert!(validate_sentence(&line).is_err(), "{body}");
    }
}

#[test]
fn strict_validates_reserved_characters_and_escape_codes() {
    for body in [
        "$GPTXT,01,01,01,raw$",
        "$GPTXT,01,01,01,raw!",
        r"$GPTXT,01,01,01,raw\slash",
        "$GPTXT,01,01,01,raw~tilde",
        "$GPTXT,01,01,01,^",
        "$GPTXT,01,01,01,^2",
        "$GPTXT,01,01,01,^2c",
        "$GPTXT,01,01,01,^GG",
    ] {
        let line = sentence(body);
        assert!(validate_sentence(&line).is_err(), "{body:?}");
    }

    validate_sentence(&sentence("$GPTXT,01,01,01,^24^21^5C^7E")).expect("uppercase ^HH escapes");
}

#[test]
fn strict_rejects_ascii_control_characters() {
    for byte in ['\0', '\t', '\u{7f}'] {
        let body = format!("$GPTXT,01,01,01,A{byte}B");
        assert!(
            matches!(
                validate_sentence(&sentence(&body)),
                Err(ComplianceError::InvalidDataCharacter(c)) if c == byte
            ),
            "control byte {byte:?}"
        );
    }
}
