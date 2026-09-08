use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PHTRO — Vessel Pitch and Roll (Xsens).
///
/// Wire: `pitch,bow,roll,port`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PHTRO"`.
/// Encode with `to_sentence("")`.
#[derive(Debug, Clone, PartialEq)]
pub struct Phtro {
    /// Pitch angle in degrees.
    pub pitch: Option<f32>,
    /// Bow direction ('P' = bow up, 'A' = bow down).
    pub bow: Option<char>,
    /// Roll angle in degrees.
    pub roll: Option<f32>,
    /// Port direction ('P' = port down, 'S' = starboard down).
    pub port: Option<char>,
}

impl Phtro {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let pitch = r.f32();
        let bow = r.char();
        let roll = r.f32();
        let port = r.char();
        Some(Self {
            pitch,
            bow,
            roll,
            port,
        })
    }
}

impl NmeaEncodable for Phtro {
    const SENTENCE_TYPE: &str = "PHTRO";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.pitch);
        w.char(self.bow);
        w.f32(self.roll);
        w.char(self.port);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn phtro_empty() {
        let s = Phtro {
            pitch: None,
            bow: None,
            roll: None,
            port: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Phtro::parse(&f.fields).expect("parse");
        assert!(p.pitch.is_none());
        assert!(p.roll.is_none());
    }

    #[test]
    fn phtro_encode_roundtrip() {
        let original = Phtro {
            pitch: Some(10.37),
            bow: Some('P'),
            roll: Some(177.62),
            port: Some('T'),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Phtro::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn phtro_phtro_gonmea() {
        let frame = parse_frame("$PHTRO,10.37,P,177.62,T*65").expect("valid");
        let p = Phtro::parse(&frame.fields).expect("parse");
        assert!((p.pitch.expect("pitch") - 10.37).abs() < 0.01);
        assert_eq!(p.bow, Some('P'));
        assert!((p.roll.expect("roll") - 177.62).abs() < 0.01);
        assert_eq!(p.port, Some('T'));
    }
}
