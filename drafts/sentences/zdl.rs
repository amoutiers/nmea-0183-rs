use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Zdl {
    pub time: Option<String>,
    pub distance: Option<f32>,
    pub point_type: Option<char>,
}

impl Zdl {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            time: r.string(),
            distance: r.f32(),
            point_type: r.char(),
        })
    }
}

impl NmeaEncodable for Zdl {
    const SENTENCE_TYPE: &'static str = "ZDL";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.f32(self.distance);
        w.char(self.point_type);
        w.finish()
    }
}
