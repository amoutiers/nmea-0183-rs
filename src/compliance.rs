//! Strict validation for complete NMEA 0183 wire sentences.
//!
//! The existing frame parser intentionally accepts common device deviations.
//! This module provides an opt-in standards boundary without changing that
//! compatibility behavior.

use crate::{EncodeError, FrameError, NmeaFrame, parse_frame};

/// Maximum NMEA 0183 sentence length, including the leading delimiter and CRLF.
pub const MAX_SENTENCE_LEN: usize = 82;

/// Errors reported by strict sentence validation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplianceError {
    /// The permissive frame parser rejected the sentence or its tag block.
    Frame(FrameError),
    /// The sentence does not end with the required CRLF pair.
    MissingTerminator,
    /// The sentence exceeds the NMEA 0183 wire-length limit.
    SentenceTooLong { actual: usize, maximum: usize },
    /// No checksum delimiter and value are present.
    MissingChecksum,
    /// The checksum contains lowercase hexadecimal digits.
    LowercaseChecksum,
    /// The sentence does not begin with `$` or `!`.
    InvalidPrefix(char),
    /// The address has an invalid number of bytes.
    InvalidAddressLength { actual: usize },
    /// The address contains a character outside `[A-Z0-9]`.
    InvalidAddressCharacter(char),
    /// A data field contains a character that is not permitted on the wire.
    InvalidDataCharacter(char),
    /// A `^HH` escaped character is incomplete or is not uppercase hexadecimal.
    MalformedEscape { offset: usize },
}

impl core::fmt::Display for ComplianceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Frame(error) => write!(f, "invalid frame: {error}"),
            Self::MissingTerminator => write!(f, "sentence must end with CRLF"),
            Self::SentenceTooLong { actual, maximum } => {
                write!(f, "sentence is {actual} bytes; maximum is {maximum}")
            }
            Self::MissingChecksum => write!(f, "sentence checksum is required"),
            Self::LowercaseChecksum => {
                write!(f, "sentence checksum must use uppercase hexadecimal")
            }
            Self::InvalidPrefix(prefix) => {
                write!(f, "invalid prefix '{prefix}', expected '$' or '!'")
            }
            Self::InvalidAddressLength { actual } => {
                write!(f, "invalid address length: {actual} bytes")
            }
            Self::InvalidAddressCharacter(character) => {
                write!(f, "address contains invalid character {character:?}")
            }
            Self::InvalidDataCharacter(character) => {
                write!(f, "data contains invalid character {character:?}")
            }
            Self::MalformedEscape { offset } => {
                write!(f, "malformed ^HH escape at byte offset {offset}")
            }
        }
    }
}

impl std::error::Error for ComplianceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Frame(error) => Some(error),
            _ => None,
        }
    }
}

impl From<FrameError> for ComplianceError {
    fn from(error: FrameError) -> Self {
        Self::Frame(error)
    }
}

/// Errors returned while building and strictly validating an encoded sentence.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrictEncodeError {
    /// The compatibility encoder rejected an address or field.
    Encode(EncodeError),
    /// The encoded sentence violates a strict wire rule.
    Compliance(ComplianceError),
}

impl core::fmt::Display for StrictEncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Encode(error) => write!(f, "encoding failed: {error}"),
            Self::Compliance(error) => write!(f, "strict validation failed: {error}"),
        }
    }
}

impl std::error::Error for StrictEncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Encode(error) => Some(error),
            Self::Compliance(error) => Some(error),
        }
    }
}

impl From<EncodeError> for StrictEncodeError {
    fn from(error: EncodeError) -> Self {
        Self::Encode(error)
    }
}

impl From<ComplianceError> for StrictEncodeError {
    fn from(error: ComplianceError) -> Self {
        Self::Compliance(error)
    }
}

/// Validate a complete NMEA 0183 sentence using the strict wire rules.
pub fn validate_sentence(input: &str) -> Result<(), ComplianceError> {
    parse_frame_strict(input).map(|_| ())
}

/// Encode a sentence and require the resulting wire representation to satisfy
/// the strict frame rules.
pub fn encode_frame_strict(
    prefix: char,
    talker: &str,
    sentence_type: &str,
    fields: &[&str],
) -> Result<String, StrictEncodeError> {
    let sentence = crate::encode_frame(prefix, talker, sentence_type, fields)?;
    validate_sentence(&sentence)?;
    Ok(sentence)
}

/// Parse a complete sentence only when it satisfies the strict wire rules.
pub fn parse_frame_strict(input: &str) -> Result<NmeaFrame<'_>, ComplianceError> {
    let sentence = sentence_part(input)?;
    validate_envelope(sentence)?;
    parse_frame(input).map_err(ComplianceError::Frame)
}

fn sentence_part(input: &str) -> Result<&str, ComplianceError> {
    let without_terminator = input
        .strip_suffix("\r\n")
        .ok_or(ComplianceError::MissingTerminator)?;

    let sentence = if let Some(tag) = without_terminator.strip_prefix('\\') {
        let closing = tag
            .find('\\')
            .ok_or(ComplianceError::Frame(FrameError::MalformedTagBlock))?;
        &tag[closing + 1..]
    } else {
        without_terminator
    };

    if sentence.is_empty() {
        return Err(ComplianceError::Frame(FrameError::TooShort));
    }
    Ok(sentence)
}

fn validate_envelope(sentence: &str) -> Result<(), ComplianceError> {
    let actual = sentence.len() + 2;
    if actual > MAX_SENTENCE_LEN {
        return Err(ComplianceError::SentenceTooLong {
            actual,
            maximum: MAX_SENTENCE_LEN,
        });
    }

    let prefix = sentence
        .chars()
        .next()
        .ok_or(ComplianceError::MissingChecksum)?;
    if !matches!(prefix, '$' | '!') {
        return Err(ComplianceError::InvalidPrefix(prefix));
    }

    let star = sentence
        .rfind('*')
        .ok_or(ComplianceError::MissingChecksum)?;
    let checksum = &sentence[star + 1..];
    if checksum.len() != 2 || !checksum.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(ComplianceError::Frame(FrameError::MalformedChecksum));
    }
    if !checksum
        .bytes()
        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'A'..=b'F'))
    {
        return Err(ComplianceError::LowercaseChecksum);
    }

    validate_address_and_data(&sentence[1..star], prefix)
}

fn validate_address_and_data(body: &str, prefix: char) -> Result<(), ComplianceError> {
    let (address, data) = match body.split_once(',') {
        Some((address, data)) => (address, Some(data)),
        None => (body, None),
    };
    validate_address(address, prefix)?;
    if let Some(data) = data {
        validate_data(data, address.len() + 2)?;
    }
    Ok(())
}

fn validate_address(address: &str, prefix: char) -> Result<(), ComplianceError> {
    if prefix == '!' && address == "**TTD" {
        return Ok(());
    }
    if !address.is_ascii() {
        let character = address
            .chars()
            .find(|character| !character.is_ascii())
            .unwrap_or('\0');
        return Err(ComplianceError::InvalidAddressCharacter(character));
    }

    if let Some(proprietary) = address.strip_prefix('P') {
        if address.len() < 4 {
            return Err(ComplianceError::InvalidAddressLength {
                actual: address.len(),
            });
        }
        if let Some(byte) = proprietary
            .as_bytes()
            .iter()
            .take(3)
            .copied()
            .find(|byte| !byte.is_ascii_uppercase() && !byte.is_ascii_digit())
        {
            return Err(ComplianceError::InvalidAddressCharacter(char::from(byte)));
        }
        return validate_data(&address[4..], 5);
    }

    if address.len() != 5 {
        return Err(ComplianceError::InvalidAddressLength {
            actual: address.len(),
        });
    }
    if let Some(byte) = address
        .bytes()
        .find(|byte| !byte.is_ascii_uppercase() && !byte.is_ascii_digit())
    {
        return Err(ComplianceError::InvalidAddressCharacter(char::from(byte)));
    }
    Ok(())
}

fn validate_data(data: &str, base_offset: usize) -> Result<(), ComplianceError> {
    let bytes = data.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        match byte {
            b',' => index += 1,
            b'^' => {
                let escape =
                    bytes
                        .get(index + 1..index + 3)
                        .ok_or(ComplianceError::MalformedEscape {
                            offset: base_offset + index,
                        })?;
                if !escape
                    .iter()
                    .all(|byte| byte.is_ascii_digit() || matches!(byte, b'A'..=b'F'))
                {
                    return Err(ComplianceError::MalformedEscape {
                        offset: base_offset + index,
                    });
                }
                index += 3;
            }
            0x20..=0x7e if !matches!(byte, b'$' | b'!' | b'*' | b'\\' | b'~') => {
                index += 1;
            }
            _ => {
                let character = data[index..].chars().next().unwrap_or(char::from(byte));
                return Err(ComplianceError::InvalidDataCharacter(character));
            }
        }
    }
    Ok(())
}
