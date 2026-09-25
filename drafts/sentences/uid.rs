use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// UID — User identification code transmission.
///
/// Wire: `uid1[,uid2]`
#[derive(Debug, Clone, PartialEq)]
pub struct Uid {
    /// Primary user identification code.
    pub uid1: Option<String>,
    /// Optional additional user or project identification, retained verbatim.
    pub uid2: Option<String>,
}

impl Uid {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            uid1: r.string(),
            uid2: r.string(),
        })
    }
}

impl NmeaEncodable for Uid {
    const SENTENCE_TYPE: &str = "UID";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.uid1.as_deref());
        if self.uid2.is_some() {
            w.string(self.uid2.as_deref());
        }
        w.finish()
    }
}
