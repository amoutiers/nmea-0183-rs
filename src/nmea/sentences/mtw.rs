use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// MTW — Mean Temperature of Water.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mtw {
    /// Water temperature (unit in `units` field — typically 'C' for Celsius).
    pub temperature: Option<f32>,
    /// Temperature unit indicator ('C' = Celsius).
    pub units: Option<char>,
}

impl Mtw {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            temperature: r.f32(),
            units: r.char(),
        }
    }
}

impl NmeaEncodable for Mtw {
    const SENTENCE_TYPE: &str = "MTW";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.temperature);
        w.char(self.units);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn mtw_15_signalk() {
        let f = parse_frame("$YXMTW,15.2,C*14").expect("valid MTW frame");
        let m = Mtw::parse(&f.fields);
        assert!((m.temperature.expect("temperature present") - 15.2).abs() < 0.01);
        assert_eq!(m.units, Some('C'));
    }

    #[test]
    fn mtw_17_gpsd() {
        let f = parse_frame("$INMTW,17.9,C*1B").expect("valid MTW frame");
        let m = Mtw::parse(&f.fields);
        assert!((m.temperature.expect("temperature present") - 17.9).abs() < 0.01);
        assert_eq!(m.units, Some('C'));
    }

    #[test]
    fn mtw_empty() {
        let f = parse_frame("$IIMTW,,*4E").expect("valid MTW frame");
        let m = Mtw::parse(&f.fields);
        assert!(m.temperature.is_none());
        assert!(m.units.is_none());
    }

    #[test]
    fn mtw_encode_roundtrip() {
        let original = Mtw {
            temperature: Some(15.2),
            units: Some('C'),
        };
        let sentence = original.to_sentence("YX").expect("encode");
        let f = parse_frame(sentence.trim()).expect("re-parse MTW frame");
        let parsed = Mtw::parse(&f.fields);
        assert_eq!(original, parsed);
    }
}
