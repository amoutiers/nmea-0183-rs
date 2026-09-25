use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Epv {
    pub status: Option<char>,
    pub equipment_type: Option<String>,
    pub equipment_id: Option<String>,
    pub property_id: Option<String>,
    pub value: Option<String>,
}

impl Epv {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            status: r.char(),
            equipment_type: r.string(),
            equipment_id: r.string(),
            property_id: r.string(),
            value: r.string(),
        })
    }
}

impl NmeaEncodable for Epv {
    const SENTENCE_TYPE: &str = "EPV";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.status);
        w.string(self.equipment_type.as_deref());
        w.string(self.equipment_id.as_deref());
        w.string(self.property_id.as_deref());
        w.string(self.value.as_deref());
        w.finish()
    }
}
