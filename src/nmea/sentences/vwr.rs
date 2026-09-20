use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VWR — Relative Wind Speed and Angle.
///
/// Wire: `angle,angleLR,spdN,N,spdMS,M,spdKMH,K`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vwr {
    /// Relative wind angle in degrees (0–180).
    pub angle: Option<f32>,
    /// Wind direction relative to bow ('L' = left/port, 'R' = right/starboard).
    pub angle_lr: Option<char>,
    /// Relative wind speed in knots.
    pub speed_knots: Option<f32>,
    /// Relative wind speed in meters per second.
    pub speed_ms: Option<f32>,
    /// Relative wind speed in km/h.
    pub speed_kmh: Option<f32>,
}

impl Vwr {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let angle = r.f32();
        let angle_lr = r.char();
        let speed_knots = r.f32();
        r.skip(); // N
        let speed_ms = r.f32();
        r.skip(); // M
        let speed_kmh = r.f32();
        r.skip(); // K
        Self {
            angle,
            angle_lr,
            speed_knots,
            speed_ms,
            speed_kmh,
        }
    }
}

impl NmeaEncodable for Vwr {
    const SENTENCE_TYPE: &str = "VWR";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.angle);
        w.char(self.angle_lr);
        w.f32(self.speed_knots);
        w.fixed('N');
        w.f32(self.speed_ms);
        w.fixed('M');
        w.f32(self.speed_kmh);
        w.fixed('K');
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn vwr_empty() {
        let f = Vwr {
            angle: None,
            angle_lr: None,
            speed_knots: None,
            speed_ms: None,
            speed_kmh: None,
        }
        .to_sentence("II").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let v = Vwr::parse(&frame.fields);
        assert!(v.angle.is_none());
        assert!(v.angle_lr.is_none());
        assert!(v.speed_knots.is_none());
    }

    #[test]
    fn vwr_encode_roundtrip() {
        let original = Vwr {
            angle: Some(75.0),
            angle_lr: Some('R'),
            speed_knots: Some(1.0),
            speed_ms: Some(0.51),
            speed_kmh: Some(1.85),
        };
        let sentence = original.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Vwr::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn vwr_full_gonmea() {
        let frame = parse_frame("$IIVWR,75,R,1.0,N,0.51,M,1.85,K*6C").expect("valid VWR frame");
        let vwr = Vwr::parse(&frame.fields);
        assert!((vwr.angle.expect("angle") - 75.0).abs() < 0.1);
        assert_eq!(vwr.angle_lr, Some('R'));
        assert!((vwr.speed_knots.expect("kts") - 1.0).abs() < 0.1);
        assert!((vwr.speed_ms.expect("ms") - 0.51).abs() < 0.01);
        assert!((vwr.speed_kmh.expect("kmh") - 1.85).abs() < 0.01);
    }

    #[test]
    fn vwr_partial_gonmea() {
        let frame = parse_frame("$IIVWR,024,L,018,N,,,,*5e").expect("valid go-nmea VWR frame");
        let vwr = Vwr::parse(&frame.fields);
        assert!((vwr.angle.expect("angle") - 24.0).abs() < 0.1);
        assert_eq!(vwr.angle_lr, Some('L'));
        assert!((vwr.speed_knots.expect("kts") - 18.0).abs() < 0.1);
        assert!(vwr.speed_ms.is_none());
        assert!(vwr.speed_kmh.is_none());
    }
}
