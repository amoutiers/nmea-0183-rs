use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

// Edition 2024 reserves `gen`; wire this file with `mod r#gen;`.
#[derive(Debug, Clone, PartialEq)]
pub struct Gen {
    pub index: Option<String>,
    pub time: Option<String>,
    pub data: Vec<String>,
}

impl Gen {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let index = r.string();
        let time = r.string();
        let data = fields
            .iter()
            .skip(2)
            .filter(|field| !field.is_empty())
            .map(|field| (*field).to_string())
            .collect();
        Some(Self { index, time, data })
    }
}

impl NmeaEncodable for Gen {
    const SENTENCE_TYPE: &str = "GEN";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.index.as_deref());
        w.string(self.time.as_deref());
        for field in &self.data {
            w.string(Some(field));
        }
        w.finish()
    }
}
