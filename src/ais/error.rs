/// Errors from AIS fragment reassembly and message decoding.
///
/// Frame and checksum errors are reported separately by [`crate::parse_frame`].
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AisDecodeError {
    /// A VDM/VDO frame has fewer than six fields.
    MissingFragmentFields { actual: usize },
    /// A fragmentation field cannot be parsed or is outside its supported range.
    InvalidFragmentField { field: &'static str },
    /// A continuation has no matching assembly or conflicts with its sequence.
    UnexpectedFragment,
    /// The assembled payload exceeds the reassembly limit, measured in bytes.
    PayloadTooLong { actual: usize, maximum: usize },
    /// The payload contains invalid armor or impossible fill bits.
    InvalidArmor,
    /// A known message cannot be decoded, or its type cannot be read.
    InvalidMessage { msg_type: Option<u8> },
}

impl core::fmt::Display for AisDecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingFragmentFields { actual } => {
                write!(f, "expected at least 6 AIS fragment fields, got {actual}")
            }
            Self::InvalidFragmentField { field } => {
                write!(f, "invalid AIS fragment field: {field}")
            }
            Self::UnexpectedFragment => write!(f, "unexpected AIS fragment"),
            Self::PayloadTooLong { actual, maximum } => {
                write!(f, "AIS payload is {actual} bytes; maximum is {maximum}")
            }
            Self::InvalidArmor => write!(f, "invalid AIS armor"),
            Self::InvalidMessage { msg_type } => {
                write!(f, "cannot decode AIS message type {msg_type:?}")
            }
        }
    }
}

impl std::error::Error for AisDecodeError {}
