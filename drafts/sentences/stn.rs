use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Stn {
    pub talker_id: Option<String>,
}

impl Stn {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            talker_id: r.string(),
        })
    }
}

impl NmeaEncodable for Stn {
    const SENTENCE_TYPE: &'static str = "STN";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.talker_id.as_deref());
        w.finish()
    }
}
