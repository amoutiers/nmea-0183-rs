use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Spw {
    pub protected_sentence: Option<String>,
    pub identifier: Option<String>,
    pub password_level: Option<u8>,
    pub password: Option<String>,
}

impl Spw {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            protected_sentence: r.string(),
            identifier: r.string(),
            password_level: r.u8(),
            password: r.string(),
        })
    }
}

impl NmeaEncodable for Spw {
    const SENTENCE_TYPE: &'static str = "SPW";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.protected_sentence.as_deref());
        w.string(self.identifier.as_deref());
        w.u8(self.password_level);
        w.string(self.password.as_deref());
        w.finish()
    }
}
