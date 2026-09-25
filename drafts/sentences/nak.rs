use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Nak {
    pub talker_id: Option<String>,
    pub formatter: Option<String>,
    pub identifier: Option<String>,
    pub reason: Option<String>,
    pub text: Option<String>,
}

impl Nak {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            talker_id: r.string(),
            formatter: r.string(),
            identifier: r.string(),
            reason: r.string(),
            text: r.string(),
        })
    }
}

impl NmeaEncodable for Nak {
    const SENTENCE_TYPE: &'static str = "NAK";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.talker_id.as_deref());
        w.string(self.formatter.as_deref());
        w.string(self.identifier.as_deref());
        w.string(self.reason.as_deref());
        w.string(self.text.as_deref());
        w.finish()
    }
}
