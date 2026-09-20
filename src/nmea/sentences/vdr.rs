use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VDR — Set and Drift.
///
/// Wire: `dirT,T,dirM,M,spd,N`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vdr {
    /// Set (current direction) true in degrees.
    pub direction_true: Option<f32>,
    /// Set (current direction) magnetic in degrees.
    pub direction_mag: Option<f32>,
    /// Drift (current speed) in knots.
    pub speed_knots: Option<f32>,
}

impl Vdr {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let direction_true = r.f32();
        r.skip(); // T
        let direction_mag = r.f32();
        r.skip(); // M
        let speed_knots = r.f32();
        r.skip(); // N
        Self {
            direction_true,
            direction_mag,
            speed_knots,
        }
    }
}

impl NmeaEncodable for Vdr {
    const SENTENCE_TYPE: &str = "VDR";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.direction_true);
        w.fixed('T');
        w.f32(self.direction_mag);
        w.fixed('M');
        w.f32(self.speed_knots);
        w.fixed('N');
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn vdr_empty() {
        let f = Vdr {
            direction_true: None,
            direction_mag: None,
            speed_knots: None,
        }
        .to_sentence("II").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let v = Vdr::parse(&frame.fields);
        assert!(v.direction_true.is_none());
        assert!(v.direction_mag.is_none());
        assert!(v.speed_knots.is_none());
    }

    #[test]
    fn vdr_encode_roundtrip() {
        let original = Vdr {
            direction_true: Some(10.1),
            direction_mag: Some(12.3),
            speed_knots: Some(1.2),
        };
        let sentence = original.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Vdr::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn vdr_full_gonmea() {
        let frame = parse_frame("$IIVDR,10.1,T,12.3,M,1.2,N*3A").expect("valid VDR frame");
        let vdr = Vdr::parse(&frame.fields);
        assert!((vdr.direction_true.expect("dirT") - 10.1).abs() < 0.1);
        assert!((vdr.direction_mag.expect("dirM") - 12.3).abs() < 0.1);
        assert!((vdr.speed_knots.expect("spd") - 1.2).abs() < 0.1);
    }
}
