use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// CUR — Water Current Layer.
///
/// Wire: `validity,dataset,layer,depth,direction,T/R,speed,reference_depth,heading,T/M,speed_reference`
///
/// Some specification templates show a terminal comma after `speed_reference`.
/// Parsing ignores that extra empty field; encoding emits the eleven defined fields.
#[derive(Debug, Clone, PartialEq)]
pub struct Cur {
    /// `A` = valid, `V` = invalid.
    pub validity: Option<char>,
    /// Identifies measurements from the same measurement instance.
    pub data_set_number: Option<u8>,
    /// Current-measurement layer number.
    pub layer_number: Option<u8>,
    /// Current depth in metres.
    pub depth: Option<f32>,
    /// Current direction in degrees.
    pub direction: Option<f32>,
    /// Direction reference: true (`T`) or relative (`R`).
    pub direction_reference: Option<char>,
    /// Current speed in knots.
    pub speed: Option<f32>,
    /// Reference-layer depth in metres.
    pub reference_depth: Option<f32>,
    /// Heading in degrees.
    pub heading: Option<f32>,
    /// Heading reference: true (`T`) or magnetic (`M`).
    pub heading_reference: Option<char>,
    /// Speed reference: bottom track (`B`), water track (`W`), or positioning system (`P`).
    pub speed_reference: Option<char>,
}

impl Cur {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            validity: r.char(),
            data_set_number: r.u8(),
            layer_number: r.u8(),
            depth: r.f32(),
            direction: r.f32(),
            direction_reference: r.char(),
            speed: r.f32(),
            reference_depth: r.f32(),
            heading: r.f32(),
            heading_reference: r.char(),
            speed_reference: r.char(),
        })
    }
}

impl NmeaEncodable for Cur {
    const SENTENCE_TYPE: &str = "CUR";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.validity);
        w.u8(self.data_set_number);
        w.u8(self.layer_number);
        w.f32(self.depth);
        w.f32(self.direction);
        w.char(self.direction_reference);
        w.f32(self.speed);
        w.f32(self.reference_depth);
        w.f32(self.heading);
        w.char(self.heading_reference);
        w.char(self.speed_reference);
        w.finish()
    }
}
