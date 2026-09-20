use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// WCV — Waypoint Closure Velocity.
///
/// Wire: `vel,N,wpt[,mode]`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Wcv {
    /// Closure velocity toward waypoint in knots.
    pub vel: Option<f32>,
    /// Waypoint identifier.
    pub wpt: Option<String>,
    /// Mode indicator ('A' = autonomous, 'D' = differential, etc.).
    pub mode: Option<char>,
}

impl Wcv {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let vel = r.f32();
        r.skip(); // N
        let wpt = r.string();
        let mode = r.char();
        Some(Self { vel, wpt, mode })
    }
}

impl NmeaEncodable for Wcv {
    const SENTENCE_TYPE: &str = "WCV";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.vel);
        w.fixed('N');
        w.string(self.wpt.as_deref());
        w.char(self.mode);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn wcv_empty() {
        let f = Wcv {
            vel: None,
            wpt: None,
            mode: None,
        }
        .to_sentence("GP").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let w = Wcv::parse(&frame.fields).expect("parse");
        assert!(w.vel.is_none());
        assert!(w.wpt.is_none());
        assert!(w.mode.is_none());
    }

    #[test]
    fn wcv_encode_roundtrip() {
        let original = Wcv {
            vel: Some(5.3),
            wpt: Some("DEST".to_string()),
            mode: Some('A'),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Wcv::parse(&frame.fields).expect("re-parse WCV");
        assert_eq!(original, parsed);
    }

    #[test]
    fn wcv_synthetic() {
        let wcv = Wcv {
            vel: Some(5.3),
            wpt: Some("WAYPOINT".to_string()),
            mode: None,
        };
        let sentence = wcv.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let wcv2 = Wcv::parse(&frame.fields).expect("re-parse WCV");
        assert!((wcv2.vel.expect("vel") - 5.3).abs() < 0.01);
        assert_eq!(wcv2.wpt, Some("WAYPOINT".to_string()));
        assert!(wcv2.mode.is_none());
    }
}
