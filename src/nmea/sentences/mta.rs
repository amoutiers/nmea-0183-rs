use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// MTA — Mean Temperature of Air.
///
/// Wire: `temp,tempu`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mta {
    /// Air temperature (unit in `units` field — typically 'C' for Celsius).
    pub temperature: Option<f32>,
    /// Temperature unit indicator ('C' = Celsius).
    pub units: Option<char>,
}

impl Mta {
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

impl NmeaEncodable for Mta {
    const SENTENCE_TYPE: &str = "MTA";

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
    fn mta_empty() {
        let f = Mta {
            temperature: None,
            units: None,
        }
        .to_sentence("II").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let m = Mta::parse(&frame.fields);
        assert!(m.temperature.is_none());
        assert!(m.units.is_none());
    }

    #[test]
    fn mta_encode_roundtrip() {
        let original = Mta {
            temperature: Some(17.2),
            units: Some('C'),
        };
        let sentence = original.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Mta::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn mta_signalk() {
        let frame = parse_frame("$IIMTA,17.2,C*01").expect("valid signalk MTA frame");
        let mta = Mta::parse(&frame.fields);
        assert!((mta.temperature.expect("temp") - 17.2).abs() < 0.01);
        assert_eq!(mta.units, Some('C'));
    }
}
