use crate::ais::armor::encode_armor;
use crate::{EncodeError, encode_frame};

pub mod class_a;
pub mod class_b;
pub mod safety;
pub mod stations;

pub use class_a::*;
pub use class_b::*;
pub use safety::*;
pub use stations::*;

const MAX_FRAGMENT_PAYLOAD_CHARS: usize = 60;
const MAX_FRAGMENTS: usize = 5;
const MAX_PAYLOAD_BITS: usize = 1_152;

/// AIS VDL channel for an emitted AIVDM/AIVDO sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AisChannel {
    A,
    B,
}

impl AisChannel {
    fn as_str(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
        }
    }
}

/// AIS NMEA envelope type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AisSentenceKind {
    Vdm,
    Vdo,
}

impl AisSentenceKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Vdm => "VDM",
            Self::Vdo => "VDO",
        }
    }
}

/// NMEA framing options for a stateless AIS transmission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AisTransmitOptions {
    pub sentence_kind: AisSentenceKind,
    pub channel: AisChannel,
    pub sequence_id: Option<u8>,
}

impl AisTransmitOptions {
    pub const fn vdm(channel: AisChannel) -> Self {
        Self {
            sentence_kind: AisSentenceKind::Vdm,
            channel,
            sequence_id: None,
        }
    }

    pub const fn vdo(channel: AisChannel) -> Self {
        Self {
            sentence_kind: AisSentenceKind::Vdo,
            channel,
            sequence_id: None,
        }
    }

    pub const fn with_sequence_id(self, sequence_id: u8) -> Self {
        Self {
            sequence_id: Some(sequence_id),
            ..self
        }
    }
}

/// Encode an AIS model into one or more complete AIVDM/AIVDO sentences.
pub trait AisEncodable {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError>;
}

pub use crate::ais::messages::common::PositionTimestamp;

fn encode_payload(bits: &[u8], options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
    if bits.is_empty() {
        return Err(EncodeError::InvalidAisField("payload"));
    }
    if bits.len() > MAX_PAYLOAD_BITS {
        return Err(EncodeError::TooManyAisFragments);
    }
    if matches!(options.sequence_id, Some(id) if id > 9) {
        return Err(EncodeError::InvalidAisField("sequence_id"));
    }

    let (payload, fill_bits) = encode_armor(bits);
    let fragment_count = payload.len().div_ceil(MAX_FRAGMENT_PAYLOAD_CHARS);
    if fragment_count > MAX_FRAGMENTS {
        return Err(EncodeError::TooManyAisFragments);
    }
    let sequence_id = if fragment_count > 1 {
        options
            .sequence_id
            .ok_or(EncodeError::MissingAisSequenceId)?
    } else {
        0
    };

    let total = fragment_count.to_string();
    let sequence = sequence_id.to_string();
    let mut sentences = Vec::with_capacity(fragment_count);

    for (index, payload_fragment) in payload
        .as_bytes()
        .chunks(MAX_FRAGMENT_PAYLOAD_CHARS)
        .enumerate()
    {
        let fragment = core::str::from_utf8(payload_fragment)
            .map_err(|_| EncodeError::InvalidAisField("payload"))?;
        let number = (index + 1).to_string();
        let fill = if index + 1 == fragment_count {
            fill_bits.to_string()
        } else {
            String::from("0")
        };
        let sequence_field = if fragment_count > 1 {
            sequence.as_str()
        } else {
            ""
        };
        let fields = [
            total.as_str(),
            number.as_str(),
            sequence_field,
            options.channel.as_str(),
            fragment,
            fill.as_str(),
        ];
        sentences.push(encode_frame(
            '!',
            "AI",
            options.sentence_kind.as_str(),
            &fields,
        )?);
    }

    Ok(sentences)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EncodeError, parse_frame};

    #[test]
    fn payload_of_61_characters_is_fragmented_at_60() {
        let bits = vec![0; 61 * 6];
        let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(7);
        let lines = encode_payload(&bits, options).expect("encode AIS payload");

        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("!AIVDM,2,1,7,A,"));
        assert!(lines[1].starts_with("!AIVDM,2,2,7,A,"));
        assert!(lines.iter().all(|line| line.len() <= 82));
        assert!(lines.iter().all(|line| parse_frame(line).is_ok()));
    }

    #[test]
    fn payload_requiring_six_fragments_is_rejected() {
        let bits = vec![0; 301 * 6];
        let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(7);

        assert_eq!(
            encode_payload(&bits, options),
            Err(EncodeError::TooManyAisFragments)
        );
    }
}
