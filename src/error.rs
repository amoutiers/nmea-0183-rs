/// Errors from frame-level parsing (checksum, delimiters, tag blocks).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameError {
    /// Input is empty or whitespace-only.
    Empty,
    /// First character is not `$` or `!`.
    InvalidPrefix(char),
    /// Checksum field is not valid hexadecimal.
    MalformedChecksum,
    /// Checksum mismatch.
    BadChecksum { expected: u8, computed: u8 },
    /// Tag block opened with `\` but not properly closed.
    MalformedTagBlock,
    /// Tag block checksum mismatch.
    BadTagChecksum { expected: u8, computed: u8 },
    /// Sentence too short to contain a valid address (minimum 3 chars, 4 for
    /// proprietary `P` addresses).
    TooShort,
    /// Address (talker + type) contains non-ASCII bytes (NMEA addresses are ASCII).
    NonAsciiAddress,
}

impl core::fmt::Display for FrameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty => write!(f, "empty input"),
            Self::InvalidPrefix(c) => write!(f, "invalid prefix '{c}', expected '$' or '!'"),
            Self::MalformedChecksum => write!(f, "checksum is not valid hexadecimal"),
            Self::BadChecksum { expected, computed } => {
                write!(
                    f,
                    "checksum mismatch: expected {expected:02X}, computed {computed:02X}"
                )
            }
            Self::MalformedTagBlock => write!(f, "malformed IEC 61162-450 tag block"),
            Self::BadTagChecksum { expected, computed } => {
                write!(
                    f,
                    "tag block checksum mismatch: expected {expected:02X}, computed {computed:02X}"
                )
            }
            Self::TooShort => write!(f, "sentence too short"),
            Self::NonAsciiAddress => write!(f, "address field contains non-ASCII bytes"),
        }
    }
}

impl std::error::Error for FrameError {}

/// Errors from encoding (invalid frame parts or field values).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodeError {
    /// Prefix is not `$` or `!`.
    InvalidPrefix(char),
    /// Talker or sentence type contains non-ASCII characters.
    NonAsciiAddress,
    /// Sentence type is empty.
    EmptySentenceType,
    /// The encoded address is too short for the frame parser.
    InvalidAddressLength { actual: usize },
    /// Address contains a non-alphanumeric ASCII character (except the `!**TTD` address).
    InvalidAddressCharacter(char),
    /// A field contains `,`, `*`, `\r`, `\n`, or a non-ASCII character.
    InvalidFieldCharacter(char),
    /// A tag contains non-ASCII characters, controls, `\\` or `*`.
    InvalidTagBlockCharacter(char),
    /// Legacy error for a sentence without its frame envelope.
    /// Current `NmeaSentence` and `AisSentence` variants retain that context
    /// and do not return this error.
    MissingFrameContext,
    /// Coordinate magnitude is NaN, infinite, or negative.
    InvalidCoordinate,
    /// A numeric NMEA field is NaN or infinite rather than absent.
    NonFiniteNumber,
    /// An AIS field is outside the range or format permitted by its bit layout.
    InvalidAisField(&'static str),
    /// AIS text exceeds the fixed-width field that carries it.
    AisTextTooLong {
        field: &'static str,
        max_chars: usize,
        actual_chars: usize,
    },
    /// A multi-fragment AIS payload requires a sequential message ID.
    MissingAisSequenceId,
    /// An AIS payload exceeds the five-fragment transmission limit.
    TooManyAisFragments,
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidPrefix(c) => write!(f, "invalid prefix '{c}', expected '$' or '!'"),
            Self::NonAsciiAddress => write!(f, "talker or sentence type is not ASCII"),
            Self::EmptySentenceType => write!(f, "sentence type is empty"),
            Self::InvalidAddressLength { actual } => {
                write!(f, "invalid encoded address length: {actual} bytes")
            }
            Self::InvalidAddressCharacter(c) => {
                write!(f, "address contains invalid character {c:?}")
            }
            Self::InvalidFieldCharacter(c) => {
                write!(f, "field contains invalid character {c:?}")
            }
            Self::InvalidTagBlockCharacter(c) => {
                write!(f, "tag block contains invalid character {c:?}")
            }
            Self::MissingFrameContext => {
                write!(f, "unknown sentence requires its original frame context")
            }
            Self::InvalidCoordinate => {
                write!(f, "coordinate magnitude is NaN, infinite, or negative")
            }
            Self::NonFiniteNumber => write!(f, "numeric field is not finite"),
            Self::InvalidAisField(field) => write!(f, "invalid AIS field {field}"),
            Self::AisTextTooLong {
                field,
                max_chars,
                actual_chars,
            } => write!(
                f,
                "AIS field {field} is too long: {actual_chars} characters, maximum {max_chars}"
            ),
            Self::MissingAisSequenceId => {
                write!(
                    f,
                    "multi-fragment AIS payload requires a sequential message ID"
                )
            }
            Self::TooManyAisFragments => {
                write!(
                    f,
                    "AIS payload exceeds the five-fragment transmission limit"
                )
            }
        }
    }
}

impl std::error::Error for EncodeError {}
