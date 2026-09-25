use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Ztg {
    pub observation_time: Option<String>,
    pub time_to_go: Option<String>,
    pub destination_waypoint: Option<String>,
}

impl Ztg {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            observation_time: r.string(),
            time_to_go: r.string(),
            destination_waypoint: r.string(),
        })
    }
}

impl NmeaEncodable for Ztg {
    const SENTENCE_TYPE: &'static str = "ZTG";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.observation_time.as_deref());
        w.string(self.time_to_go.as_deref());
        w.string(self.destination_waypoint.as_deref());
        w.finish()
    }
}
