use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PGRME — Garmin Estimated Position Error.
///
/// Wire: `h_err,M,v_err,M,s_err,M`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PGRME"`.
/// Encode with `to_sentence("")` (talker is ignored for proprietary types).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pgrme {
    /// Horizontal position error in metres.
    pub horizontal: Option<f32>,
    /// Vertical position error in metres.
    pub vertical: Option<f32>,
    /// Spherical (3D) position error in metres.
    pub spherical: Option<f32>,
}

impl Pgrme {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let horizontal = r.f32();
        r.skip(); // M
        let vertical = r.f32();
        r.skip(); // M
        let spherical = r.f32();
        r.skip(); // M
        Some(Self {
            horizontal,
            vertical,
            spherical,
        })
    }
}

impl NmeaEncodable for Pgrme {
    const SENTENCE_TYPE: &str = "PGRME";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.horizontal);
        w.fixed('M');
        w.f32(self.vertical);
        w.fixed('M');
        w.f32(self.spherical);
        w.fixed('M');
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn pgrme_empty() {
        let s = Pgrme {
            horizontal: None,
            vertical: None,
            spherical: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Pgrme::parse(&f.fields).expect("parse");
        assert!(p.horizontal.is_none());
        assert!(p.vertical.is_none());
        assert!(p.spherical.is_none());
    }

    #[test]
    fn pgrme_encode_roundtrip() {
        let original = Pgrme {
            horizontal: Some(3.3),
            vertical: Some(4.9),
            spherical: Some(6.0),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Pgrme::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn pgrme_pgrme_gonmea() {
        let frame = parse_frame("$PGRME,3.3,M,4.9,M,6.0,M*25").expect("valid");
        let p = Pgrme::parse(&frame.fields).expect("parse");
        assert!((p.horizontal.expect("horizontal") - 3.3).abs() < 0.01);
        assert!((p.vertical.expect("vertical") - 4.9).abs() < 0.01);
        assert!((p.spherical.expect("spherical") - 6.0).abs() < 0.01);
    }
}
