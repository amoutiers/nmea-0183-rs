use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VPW — Speed Measured Parallel to Wind (VMG to wind).
///
/// Wire: `spdN,N,spdM,M`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vpw {
    /// Speed parallel to wind (VMG to wind) in knots.
    pub speed_knots: Option<f32>,
    /// Speed parallel to wind (VMG to wind) in meters per second.
    pub speed_ms: Option<f32>,
}

impl Vpw {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let speed_knots = r.f32();
        r.skip(); // N
        let speed_ms = r.f32();
        r.skip(); // M
        Some(Self {
            speed_knots,
            speed_ms,
        })
    }
}

impl NmeaEncodable for Vpw {
    const SENTENCE_TYPE: &str = "VPW";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.speed_knots);
        w.fixed('N');
        w.f32(self.speed_ms);
        w.fixed('M');
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn vpw_empty() {
        let f = Vpw {
            speed_knots: None,
            speed_ms: None,
        }
        .to_sentence("II").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let v = Vpw::parse(&frame.fields).expect("parse");
        assert!(v.speed_knots.is_none());
        assert!(v.speed_ms.is_none());
    }

    #[test]
    fn vpw_encode_roundtrip() {
        let original = Vpw {
            speed_knots: Some(4.5),
            speed_ms: Some(6.7),
        };
        let sentence = original.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Vpw::parse(&frame.fields).expect("re-parse VPW");
        assert_eq!(original, parsed);
    }

    #[test]
    fn vpw_full_gonmea() {
        let frame = parse_frame("$IIVPW,4.5,N,6.7,M*52").expect("valid VPW frame");
        let vpw = Vpw::parse(&frame.fields).expect("parse VPW");
        assert!((vpw.speed_knots.expect("kts") - 4.5).abs() < 0.1);
        assert!((vpw.speed_ms.expect("ms") - 6.7).abs() < 0.1);
    }

    #[test]
    fn vpw_missing_ms_signalk() {
        let frame = parse_frame("$IIVPW,4.5,N,,*30").expect("valid SignalK VPW frame");
        let vpw = Vpw::parse(&frame.fields).expect("parse VPW");
        assert!((vpw.speed_knots.expect("kts") - 4.5).abs() < 0.1);
        assert!(vpw.speed_ms.is_none());
    }
}
