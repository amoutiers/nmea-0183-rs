use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// SSD - AIS ship static data.
///
/// Wire: `call_sign,ship_name,dimension_a,dimension_b,dimension_c,dimension_d,dte_indicator,source`
#[derive(Debug, Clone, PartialEq)]
pub struct Ssd {
    /// Ship's call sign. `None` is a null field, meaning the previous value is unchanged;
    /// `Some("@@@@@@@")` means the call sign is unavailable.
    pub call_sign: Option<String>,
    /// Ship's name. `None` is a null field, meaning the previous value is unchanged;
    /// `Some("@@@@@@@@@@@@@@@@@@@@")` means the name is unavailable.
    pub ship_name: Option<String>,
    /// Distance in metres from the bow to the position reference point (0 to 511).
    pub dimension_a: Option<u16>,
    /// Distance in metres from the stern to the position reference point (0 to 511).
    pub dimension_b: Option<u16>,
    /// Distance in metres from the port beam to the position reference point (0 to 63).
    pub dimension_c: Option<u8>,
    /// Distance in metres from the starboard beam to the position reference point (0 to 63).
    pub dimension_d: Option<u8>,
    /// Data terminal equipment indicator: `0` supports communication, `1` is unknown or unavailable.
    pub dte_indicator: Option<u8>,
    /// Talker ID of the equipment at this location.
    pub source: Option<String>,
}

impl Ssd {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            call_sign: r.string(),
            ship_name: r.string(),
            dimension_a: r.u16(),
            dimension_b: r.u16(),
            dimension_c: r.u8(),
            dimension_d: r.u8(),
            dte_indicator: r.u8(),
            source: r.string(),
        })
    }
}

impl NmeaEncodable for Ssd {
    const SENTENCE_TYPE: &str = "SSD";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.call_sign.as_deref());
        w.string(self.ship_name.as_deref());
        w.u16(self.dimension_a);
        w.u16(self.dimension_b);
        w.u8(self.dimension_c);
        w.u8(self.dimension_d);
        w.u8(self.dte_indicator);
        w.string(self.source.as_deref());
        w.finish()
    }
}
