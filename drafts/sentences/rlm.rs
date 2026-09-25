use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Rlm {
    pub beacon: Option<String>,
    pub time: Option<String>,
    pub code: Option<char>,
    pub body: Option<String>,
}

impl Rlm {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            beacon: r.string(),
            time: r.string(),
            code: r.char(),
            body: r.string(),
        })
    }
}

impl NmeaEncodable for Rlm {
    const SENTENCE_TYPE: &'static str = "RLM";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.beacon.as_deref());
        w.string(self.time.as_deref());
        w.char(self.code);
        w.string(self.body.as_deref());
        w.finish()
    }
}
