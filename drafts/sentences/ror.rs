use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ROR — Rudder Order Status.
///
/// Wire: `starboard_order,starboard_status,port_order,port_status,source`
///
/// This draft targets the historical five-field format. The wider IEC 61162-1:2024
/// extension is intentionally out of scope.
#[derive(Debug, Clone, PartialEq)]
pub struct Ror {
    /// Starboard (or single) rudder order.
    pub starboard_order: Option<f32>,
    /// Starboard (or single) rudder order status.
    pub starboard_status: Option<char>,
    /// Port rudder order.
    pub port_order: Option<f32>,
    /// Port rudder order status.
    pub port_status: Option<char>,
    /// Command source location.
    pub source: Option<char>,
}

impl Ror {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            starboard_order: r.f32(),
            starboard_status: r.char(),
            port_order: r.f32(),
            port_status: r.char(),
            source: r.char(),
        })
    }
}

impl NmeaEncodable for Ror {
    const SENTENCE_TYPE: &str = "ROR";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.starboard_order);
        w.char(self.starboard_status);
        w.f32(self.port_order);
        w.char(self.port_status);
        w.char(self.source);
        w.finish()
    }
}
