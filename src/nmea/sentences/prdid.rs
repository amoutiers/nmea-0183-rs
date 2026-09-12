use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PRDID — Vessel Pitch, Roll and Heading.
///
/// Wire: `pitch,roll,heading`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PRDID"`.
/// Encode with `to_sentence("")`.
#[derive(Debug, Clone, PartialEq)]
pub struct Prdid {
    /// Pitch angle in degrees (positive = bow up).
    pub pitch: Option<f32>,
    /// Roll angle in degrees (positive = starboard down).
    pub roll: Option<f32>,
    /// True heading in degrees.
    pub heading: Option<f32>,
}

impl Prdid {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let pitch = r.f32();
        let roll = r.f32();
        let heading = r.f32();
        Some(Self {
            pitch,
            roll,
            heading,
        })
    }
}

impl NmeaEncodable for Prdid {
    const SENTENCE_TYPE: &str = "PRDID";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.pitch);
        w.f32(self.roll);
        w.f32(self.heading);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn prdid_empty() {
        let s = Prdid {
            pitch: None,
            roll: None,
            heading: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Prdid::parse(&f.fields).expect("parse");
        assert!(p.pitch.is_none());
        assert!(p.roll.is_none());
        assert!(p.heading.is_none());
    }

    #[test]
    fn prdid_encode_roundtrip() {
        let original = Prdid {
            pitch: Some(-10.37),
            roll: Some(2.34),
            heading: Some(230.34),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Prdid::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn prdid_prdid_gonmea() {
        let frame = parse_frame("$PRDID,-10.37,2.34,230.34*62").expect("valid");
        let p = Prdid::parse(&frame.fields).expect("parse");
        assert!((p.pitch.expect("pitch") - (-10.37)).abs() < 0.01);
        assert!((p.roll.expect("roll") - 2.34).abs() < 0.01);
        assert!((p.heading.expect("heading") - 230.34).abs() < 0.01);
    }
}
