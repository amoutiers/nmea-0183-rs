use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// LR3 - AIS long-range reply sentence 3.
///
/// Wire: `sequence,mmsi,destination,eta_date,eta_time,draught,ship_cargo_type,ship_length,ship_breadth,ship_type,persons`
#[derive(Debug, Clone, PartialEq)]
pub struct Lr3 {
    /// Sequence number, retained as text to preserve its wire representation.
    pub sequence: Option<String>,
    /// MMSI of the responder, retained as text to preserve leading zeroes.
    pub mmsi: Option<String>,
    /// Voyage destination, 1 to 20 characters.
    pub destination: Option<String>,
    /// ETA date (`DDMMYY`), retained as text to preserve leading zeroes.
    pub eta_date: Option<String>,
    /// ETA time (`HHMMSS.SS`), retained as text to preserve its wire representation.
    pub eta_time: Option<String>,
    /// Maximum static draught in metres.
    pub draught: Option<f32>,
    /// Ship/cargo code, retained as text because public references disagree on its numeric form.
    pub ship_cargo_type: Option<String>,
    /// Ship length in metres.
    pub ship_length: Option<f32>,
    /// Ship breadth in metres.
    pub ship_breadth: Option<f32>,
    /// AIS ship type.
    pub ship_type: Option<f32>,
    /// Persons on board (0 to 8191).
    pub persons: Option<u16>,
}

impl Lr3 {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            sequence: r.string(),
            mmsi: r.string(),
            destination: r.string(),
            eta_date: r.string(),
            eta_time: r.string(),
            draught: r.f32(),
            ship_cargo_type: r.string(),
            ship_length: r.f32(),
            ship_breadth: r.f32(),
            ship_type: r.f32(),
            persons: r.u16(),
        })
    }
}

impl NmeaEncodable for Lr3 {
    const SENTENCE_TYPE: &str = "LR3";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.sequence.as_deref());
        w.string(self.mmsi.as_deref());
        w.string(self.destination.as_deref());
        w.string(self.eta_date.as_deref());
        w.string(self.eta_time.as_deref());
        w.f32(self.draught);
        w.string(self.ship_cargo_type.as_deref());
        w.f32(self.ship_length);
        w.f32(self.ship_breadth);
        w.f32(self.ship_type);
        w.u16(self.persons);
        w.finish()
    }
}
