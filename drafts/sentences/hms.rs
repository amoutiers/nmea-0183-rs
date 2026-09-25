use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HMS — Heading Monitor Set.
///
/// Wire: `sensor1,sensor2,max_difference_deg`
#[derive(Debug, Clone, PartialEq)]
pub struct Hms {
    /// Heading sensor 1 identifier, retained as text because its domain is not specified.
    pub sensor1: Option<String>,
    /// Heading sensor 2 identifier, retained as text because its domain is not specified.
    pub sensor2: Option<String>,
    /// Maximum permitted difference between sensors, in degrees.
    pub max_difference_deg: Option<f32>,
}

impl Hms {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            sensor1: r.string(),
            sensor2: r.string(),
            max_difference_deg: r.f32(),
        })
    }
}

impl NmeaEncodable for Hms {
    const SENTENCE_TYPE: &str = "HMS";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.sensor1.as_deref());
        w.string(self.sensor2.as_deref());
        w.f32(self.max_difference_deg);
        w.finish()
    }
}
