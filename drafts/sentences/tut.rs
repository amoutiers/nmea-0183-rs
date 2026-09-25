use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// TUT — Transmission of Multi-language Text.
///
/// Wire: `source,total,sentence,sequence,translation_code,text_body`
#[derive(Debug, Clone, PartialEq)]
pub struct Tut {
    /// Talker ID of the equipment that originated the text.
    pub source_identifier: Option<String>,
    /// Total number of sentences, preserved as hexadecimal text.
    pub total_sentences: Option<String>,
    /// Current sentence number, preserved as hexadecimal text.
    pub sentence_number: Option<String>,
    /// Identifies the group of sentence fragments (0-9).
    pub sequential_message_identifier: Option<u8>,
    /// Character coding system for `text_body` (for example `U`, `A`, or `PXYZ`).
    pub translation_code: Option<String>,
    /// Encoded text, preserved in its wire hexadecimal form.
    pub text_body: Option<String>,
}

impl Tut {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            source_identifier: r.string(),
            total_sentences: r.string(),
            sentence_number: r.string(),
            sequential_message_identifier: r.u8(),
            translation_code: r.string(),
            text_body: r.string(),
        })
    }
}

impl NmeaEncodable for Tut {
    const SENTENCE_TYPE: &str = "TUT";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.source_identifier.as_deref());
        w.string(self.total_sentences.as_deref());
        w.string(self.sentence_number.as_deref());
        w.u8(self.sequential_message_identifier);
        w.string(self.translation_code.as_deref());
        w.string(self.text_body.as_deref());
        w.finish()
    }
}
