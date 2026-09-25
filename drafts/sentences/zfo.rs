use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Zfo {
    pub observation_time: Option<String>,
    pub elapsed_time: Option<String>,
    pub origin_waypoint: Option<String>,
}

impl Zfo {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            observation_time: r.string(),
            elapsed_time: r.string(),
            origin_waypoint: r.string(),
        })
    }
}

impl NmeaEncodable for Zfo {
    const SENTENCE_TYPE: &'static str = "ZFO";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.observation_time.as_deref());
        w.string(self.elapsed_time.as_deref());
        w.string(self.origin_waypoint.as_deref());
        w.finish()
    }
}
