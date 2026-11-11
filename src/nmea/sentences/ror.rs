use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ROR — Rudder Order Status.
///
/// Wire: `starboard_order,starboard_status,port_order,port_status,source,center_order,center_status,bow_order,bow_status`
#[derive(Debug, Clone, PartialEq, Default)]
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
    /// Center rudder order and its status.
    pub center_order: Option<f32>,
    pub center_status: Option<char>,
    /// Bow or other rudder order and its status.
    pub bow_order: Option<f32>,
    pub bow_status: Option<char>,
}

impl Ror {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            starboard_order: r.f32(),
            starboard_status: r.char(),
            port_order: r.f32(),
            port_status: r.char(),
            source: r.char(),
            center_order: r.f32(),
            center_status: r.char(),
            bow_order: r.f32(),
            bow_status: r.char(),
        }
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
        w.f32(self.center_order);
        w.char(self.center_status);
        w.f32(self.bow_order);
        w.char(self.bow_status);
        w.finish()
    }
}
