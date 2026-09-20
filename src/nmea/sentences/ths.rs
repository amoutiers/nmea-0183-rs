use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// THS — True Heading and Status.
///
/// Wire: `heading_true,mode`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ths {
    /// True heading in degrees (0–359.9).
    pub heading_true: Option<f32>,
    /// Mode indicator ('A' = autonomous, 'E' = estimated, 'M' = manual,
    /// 'S' = simulated, 'V' = void/invalid).
    pub mode: Option<char>,
}

impl Ths {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            heading_true: r.f32(),
            mode: r.char(),
        }
    }
}

impl NmeaEncodable for Ths {
    const SENTENCE_TYPE: &str = "THS";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.heading_true);
        w.char(self.mode);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn ths_autonomous_gonmea() {
        let frame = parse_frame("$INTHS,123.456,A*20").expect("valid go-nmea THS frame");
        let ths = Ths::parse(&frame.fields);
        assert!((ths.heading_true.expect("hdg") - 123.456).abs() < 0.001);
        assert_eq!(ths.mode, Some('A'));
    }

    #[test]
    fn ths_empty() {
        let f = Ths {
            heading_true: None,
            mode: None,
        }
        .to_sentence("GP").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let t = Ths::parse(&frame.fields);
        assert!(t.heading_true.is_none());
        assert!(t.mode.is_none());
    }

    #[test]
    fn ths_encode_roundtrip() {
        let original = Ths {
            heading_true: Some(77.52),
            mode: Some('E'),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Ths::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn ths_invalid_gonmea() {
        let frame = parse_frame("$INTHS,,V*1E").expect("valid go-nmea THS void frame");
        let ths = Ths::parse(&frame.fields);
        assert!(ths.heading_true.is_none());
        assert_eq!(ths.mode, Some('V'));
    }

    #[test]
    fn ths_manual_gonmea() {
        let frame = parse_frame("$INTHS,123.456,M*2C").expect("valid go-nmea THS frame");
        let ths = Ths::parse(&frame.fields);
        assert!((ths.heading_true.expect("hdg") - 123.456).abs() < 0.001);
        assert_eq!(ths.mode, Some('M'));
    }

    #[test]
    fn ths_pynmeagps() {
        let frame = parse_frame("$GPTHS,77.52,E*34").expect("valid pynmeagps THS frame");
        let ths = Ths::parse(&frame.fields);
        assert!((ths.heading_true.expect("hdg") - 77.52).abs() < 0.01);
        assert_eq!(ths.mode, Some('E'));
    }

    #[test]
    fn ths_simulator_gonmea() {
        let frame = parse_frame("$INTHS,123.456,S*32").expect("valid go-nmea THS frame");
        let ths = Ths::parse(&frame.fields);
        assert!((ths.heading_true.expect("hdg") - 123.456).abs() < 0.001);
        assert_eq!(ths.mode, Some('S'));
    }
}
