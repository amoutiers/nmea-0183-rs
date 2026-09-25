use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub emitter_id: Option<String>,
    pub status: Option<char>,
    pub activation_time: Option<String>,
    pub position_source: Option<char>,
    pub activation_date: Option<u32>,
    pub position_time: Option<String>,
    pub lat: Option<f64>,
    pub ns: Option<char>,
    pub lon: Option<f64>,
    pub ew: Option<char>,
    pub cog: Option<f32>,
    pub sog: Option<f32>,
    pub mmsi: Option<String>,
    pub battery_status: Option<char>,
}

impl Mob {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            emitter_id: r.string(),
            status: r.char(),
            activation_time: r.string(),
            position_source: r.char(),
            activation_date: r.u32(),
            position_time: r.string(),
            lat: r.f64(),
            ns: r.char(),
            lon: r.f64(),
            ew: r.char(),
            cog: r.f32(),
            sog: r.f32(),
            mmsi: r.string(),
            battery_status: r.char(),
        })
    }
}

impl NmeaEncodable for Mob {
    const SENTENCE_TYPE: &str = "MOB";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.emitter_id.as_deref());
        w.char(self.status);
        w.string(self.activation_time.as_deref());
        w.char(self.position_source);
        w.u32(self.activation_date);
        w.string(self.position_time.as_deref());
        w.lat(self.lat);
        w.char(self.ns);
        w.lon(self.lon);
        w.char(self.ew);
        w.f32(self.cog);
        w.f32(self.sog);
        w.string(self.mmsi.as_deref());
        w.char(self.battery_status);
        w.finish()
    }
}
