use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VWT — True Wind Speed and Angle.
///
/// Wire: `angle,angleLR,spdN,N,spdMS,M,spdKMH,K`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vwt {
    /// True wind angle in degrees (0–180).
    pub angle: Option<f32>,
    /// Wind direction relative to bow ('L' = left/port, 'R' = right/starboard).
    pub angle_lr: Option<char>,
    /// True wind speed in knots.
    pub speed_knots: Option<f32>,
    /// True wind speed in meters per second.
    pub speed_ms: Option<f32>,
    /// True wind speed in km/h.
    pub speed_kmh: Option<f32>,
}

impl Vwt {
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

impl NmeaEncodable for Vwt {
    const SENTENCE_TYPE: &str = "VWT";

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
    fn vwt_empty() {
        let f = Vwt {
            angle: None,
            angle_lr: None,
            speed_knots: None,
            speed_ms: None,
            speed_kmh: None,
        }
        .to_sentence("II").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let v = Vwt::parse(&frame.fields);
        assert!(v.angle.is_none());
        assert!(v.angle_lr.is_none());
        assert!(v.speed_knots.is_none());
    }

    #[test]
    fn vwt_encode_roundtrip() {
        let original = Vwt {
            angle: Some(30.0),
            angle_lr: Some('R'),
            speed_knots: Some(10.1),
            speed_ms: Some(5.2),
            speed_kmh: Some(18.7),
        };
        let sentence = original.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Vwt::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn vwt_full_signalk() {
        let frame = parse_frame("$IIVWT,030.,R,10.1,N,05.2,M,018.7,K*75").expect("valid VWT frame");
        let vwt = Vwt::parse(&frame.fields);
        assert!((vwt.angle.expect("angle") - 30.0).abs() < 0.1);
        assert_eq!(vwt.angle_lr, Some('R'));
        assert!((vwt.speed_knots.expect("kts") - 10.1).abs() < 0.1);
        assert!((vwt.speed_ms.expect("ms") - 5.2).abs() < 0.1);
        assert!((vwt.speed_kmh.expect("kmh") - 18.7).abs() < 0.1);
    }
}
