use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ROT — Rate of Turn.
///
/// Wire: `rot,valid`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rot {
    /// Rate of turn in degrees per minute (negative = port).
    pub rate_of_turn: Option<f32>,
    /// Data validity ('A' = valid, 'V' = invalid).
    pub valid: Option<char>,
}

impl Rot {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            rate_of_turn: r.f32(),
            valid: r.char(),
        }
    }
}

impl NmeaEncodable for Rot {
    const SENTENCE_TYPE: &str = "ROT";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.rate_of_turn);
        w.char(self.valid);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn rot_empty() {
        let f = parse_frame("$IIROT,,*49").expect("valid");
        let r = Rot::parse(&f.fields);
        assert!(r.rate_of_turn.is_none());
        assert!(r.valid.is_none());
    }

    #[test]
    fn rot_encode_roundtrip() {
        let rot = Rot {
            rate_of_turn: Some(35.6),
            valid: Some('A'),
        };
        let sentence = rot.to_sentence("GP").expect("encode");
        assert!(sentence.starts_with("$GPROT,"));
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let rot2 = Rot::parse(&frame.fields);
        assert_eq!(rot.rate_of_turn, rot2.rate_of_turn);
        assert_eq!(rot.valid, rot2.valid);
    }

    #[test]
    fn rot_negative_pynmeagps() {
        let frame = parse_frame("$IIROT,-7.3,A*0F").expect("valid");
        let rot = Rot::parse(&frame.fields);
        assert!((rot.rate_of_turn.expect("rot") - (-7.3)).abs() < 0.1);
        assert_eq!(rot.valid, Some('A'));
    }

    #[test]
    fn rot_positive_gpsd() {
        let frame = parse_frame("$GPROT,35.6,A*01").expect("valid");
        let rot = Rot::parse(&frame.fields);
        assert!((rot.rate_of_turn.expect("rot") - 35.6).abs() < 0.1);
        assert_eq!(rot.valid, Some('A'));
    }

    #[test]
    fn rot_zero_gpsd() {
        let frame = parse_frame("$HEROT,0.0,A*2B").expect("valid");
        let rot = Rot::parse(&frame.fields);
        assert!((rot.rate_of_turn.expect("rot") - 0.0).abs() < 0.01);
        assert_eq!(rot.valid, Some('A'));
    }
}
