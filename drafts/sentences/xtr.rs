use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Xtr {
    pub cross_track_error: Option<f32>,
    pub steer_direction: Option<char>,
    pub unit: Option<char>,
}

impl Xtr {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            cross_track_error: r.f32(),
            steer_direction: r.char(),
            unit: r.char(),
        })
    }
}

impl NmeaEncodable for Xtr {
    const SENTENCE_TYPE: &'static str = "XTR";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.cross_track_error);
        w.char(self.steer_direction);
        w.char(self.unit);
        w.finish()
    }
}
