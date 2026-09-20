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
        sentence_type: String,
        fields: Vec<String>,
    },
}

impl AisSentence {
    /// Parse an AIS application-layer NMEA frame into a typed sentence variant.
    pub fn parse(frame: &NmeaFrame<'_>) -> Self {
        match (frame.prefix, frame.sentence_type) {
            #[cfg(feature = "abm")]
            ('!', Abm::SENTENCE_TYPE) => match Abm::parse(&frame.fields) {
                Some(v) => return Self::Abm(v),
                None => return Self::from_frame(frame),
            },
            #[cfg(feature = "bbm")]
            ('!', Bbm::SENTENCE_TYPE) => match Bbm::parse(&frame.fields) {
                Some(v) => return Self::Bbm(v),
                None => return Self::from_frame(frame),
            },
            _ => {}
        }

        Self::from_frame(frame)
    }

    /// Encode a typed ABM/BBM variant with the given talker.
    ///
    /// `Unknown` returns [`crate::EncodeError::MissingFrameContext`]. Retain the
    /// original [`NmeaFrame`] and use its `to_sentence()` method for that case.
    pub fn to_sentence(&self, talker: &str) -> Result<String, crate::EncodeError> {
        match self {
            #[cfg(feature = "abm")]
            Self::Abm(value) => value.to_sentence(talker),
            #[cfg(feature = "bbm")]
            Self::Bbm(value) => value.to_sentence(talker),
            Self::Unknown { .. } => Err(crate::EncodeError::MissingFrameContext),
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
            sentence_type: frame.sentence_type.to_string(),
            fields: frame.fields.iter().map(|f| f.to_string()).collect(),
        }
    }
}
