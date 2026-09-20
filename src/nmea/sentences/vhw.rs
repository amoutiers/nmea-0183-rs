use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VHW — Water Speed and Heading.
///
/// Wire: `headingT,T,headingM,M,speedKts,N,speedKmh,K`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vhw {
    /// Heading true in degrees.
    pub heading_true: Option<f32>,
    /// Heading magnetic in degrees.
    pub heading_mag: Option<f32>,
    /// Speed through water in knots.
    pub speed_kts: Option<f32>,
    /// Speed through water in km/h.
    pub speed_kmh: Option<f32>,
}

impl Vhw {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let heading_true = r.f32();
        r.skip();
        let heading_mag = r.f32();
        r.skip();
        let speed_kts = r.f32();
        r.skip();
        let speed_kmh = r.f32();
        r.skip(); // K
        Self {
            heading_true,
            heading_mag,
            speed_kts,
            speed_kmh,
        }
    }
}

impl NmeaEncodable for Vhw {
    const SENTENCE_TYPE: &str = "VHW";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.heading_true);
        w.fixed('T');
        w.f32(self.heading_mag);
        w.fixed('M');
        w.f32(self.speed_kts);
        w.fixed('N');
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
    fn vhw_empty() {
        let f = parse_frame("$IIVHW,,,,,,,,*49").expect("valid");
        let v = Vhw::parse(&f.fields);
        assert!(v.heading_true.is_none());
        assert!(v.heading_mag.is_none());
        assert!(v.speed_kts.is_none());
        assert!(v.speed_kmh.is_none());
    }

    #[test]
    fn vhw_full_signalk() {
        let f = parse_frame("$SDVHW,182.5,T,181.8,M,0.0,N,0.0,K*4C").expect("valid VHW frame");
        let v = Vhw::parse(&f.fields);
        assert!((v.heading_true.expect("heading_true present") - 182.5).abs() < 0.01);
    }

    #[test]
    fn vhw_partial_signalk() {
        let f = parse_frame("$IIVHW,,T,,M,06.12,N,11.33,K*50").expect("valid VHW frame");
        let v = Vhw::parse(&f.fields);
        assert!(v.heading_true.is_none());
        assert!((v.speed_kts.expect("speed_kts present") - 6.12).abs() < 0.01);
    }

    #[test]
    fn vhw_encode_roundtrip() {
        let original = Vhw {
            heading_true: Some(182.5),
            heading_mag: Some(181.8),
            speed_kts: Some(12.5),
            speed_kmh: Some(23.1),
        };
        let sentence = original.to_sentence("SD").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse VHW sentence");
        let parsed = Vhw::parse(&frame.fields);

        assert_eq!(original, parsed);
    }
}
