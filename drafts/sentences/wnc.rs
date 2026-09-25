use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Wnc {
    pub distance_nm: Option<f32>,
    pub nm_unit: Option<char>,
    pub distance_km: Option<f32>,
    pub km_unit: Option<char>,
    pub destination_waypoint: Option<String>,
    pub origin_waypoint: Option<String>,
}

impl Wnc {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            distance_nm: r.f32(),
            nm_unit: r.char(),
            distance_km: r.f32(),
            km_unit: r.char(),
            destination_waypoint: r.string(),
            origin_waypoint: r.string(),
        })
    }
}

impl NmeaEncodable for Wnc {
    const SENTENCE_TYPE: &'static str = "WNC";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.distance_nm);
        w.char(self.nm_unit);
        w.f32(self.distance_km);
        w.char(self.km_unit);
        w.string(self.destination_waypoint.as_deref());
        w.string(self.origin_waypoint.as_deref());
        w.finish()
    }
}
