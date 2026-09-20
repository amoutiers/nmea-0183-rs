//! AIS application-layer NMEA sentence types.

use crate::NmeaFrame;

#[cfg(feature = "abm")]
mod abm;
#[cfg(feature = "bbm")]
mod bbm;
mod field;

#[cfg(feature = "abm")]
pub use abm::*;
#[cfg(feature = "bbm")]
pub use bbm::*;

/// Unified enum covering supported AIS application-layer NMEA sentence types.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum AisSentence {
    #[cfg(feature = "abm")]
    Abm(Abm),
    #[cfg(feature = "bbm")]
    Bbm(Bbm),
    Unknown {
        prefix: char,
        talker: String,
        sentence_type: String,
        fields: Vec<String>,
        tag_block: Option<String>,
    },
}

impl AisSentence {
    /// Parse an AIS application-layer NMEA frame into a typed sentence variant.
    pub fn parse(frame: &NmeaFrame<'_>) -> Self {
        match (frame.prefix, frame.sentence_type) {
            #[cfg(feature = "abm")]
            ('!', Abm::SENTENCE_TYPE) => return Self::Abm(Abm::parse(&frame.fields)),
            #[cfg(feature = "bbm")]
            ('!', Bbm::SENTENCE_TYPE) => return Self::Bbm(Bbm::parse(&frame.fields)),
            _ => {}
        }

        Self::from_frame(frame)
    }

    /// Encode a typed ABM/BBM variant with the given talker.
    ///
    /// `Unknown` preserves its captured envelope and ignores the given talker.
    /// Typed variants do not retain tags: keep the original [`NmeaFrame`]
    /// to re-emit their full envelope.
    pub fn to_sentence(&self, talker: &str) -> Result<String, crate::EncodeError> {
        match self {
            #[cfg(feature = "abm")]
            Self::Abm(value) => value.to_sentence(talker),
            #[cfg(feature = "bbm")]
            Self::Bbm(value) => value.to_sentence(talker),
            Self::Unknown {
                prefix,
                talker,
                sentence_type,
                fields,
                tag_block,
            } => NmeaFrame {
                prefix: *prefix,
                talker,
                sentence_type,
                fields: fields.iter().map(String::as_str).collect(),
                tag_block: tag_block.as_deref(),
            }
            .to_sentence(),
        }
    }

    /// Encode a typed variant and validate its strict frame envelope.
    /// This does not validate the application fields or reassemble fragments.
    pub fn to_sentence_strict(&self, talker: &str) -> Result<String, crate::StrictEncodeError> {
        let sentence = self.to_sentence(talker)?;
        crate::validate_sentence(&sentence)?;
        Ok(sentence)
    }

    fn from_frame(frame: &NmeaFrame<'_>) -> Self {
        Self::Unknown {
            prefix: frame.prefix,
            talker: frame.talker.to_string(),
            sentence_type: frame.sentence_type.to_string(),
            fields: frame.fields.iter().map(|f| f.to_string()).collect(),
            tag_block: frame.tag_block.map(str::to_owned),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_sentences_preserve_envelope() {
        for (enabled, input) in [
            (cfg!(feature = "abm"), "!AIABM,,"),
            (cfg!(feature = "bbm"), "!AIBBM,,"),
        ] {
            if enabled {
                continue;
            }
            let frame = crate::parse_frame(input).expect("frame");
            let value = AisSentence::parse(&frame);
            assert!(matches!(value, AisSentence::Unknown { .. }));
            let line = value.to_sentence("ignored").expect("encode");
            assert_eq!(crate::parse_frame(&line).expect("reparse"), frame);
        }
    }
}
