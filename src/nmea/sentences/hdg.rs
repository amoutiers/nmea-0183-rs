use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HDG — Heading with Deviation & Variation.
///
/// Wire: `headingM,deviation,deviationEW,variation,variationEW`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Hdg {
    /// Magnetic sensor heading in degrees.
    pub heading_mag: Option<f32>,
    /// Magnetic deviation in degrees.
    pub deviation: Option<f32>,
    /// Magnetic deviation direction ('E' = east, 'W' = west).
    pub deviation_ew: Option<char>,
    /// Magnetic variation in degrees.
    pub variation: Option<f32>,
    /// Magnetic variation direction ('E' = east, 'W' = west).
    pub variation_ew: Option<char>,
}

impl Hdg {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            heading_mag: r.f32(),
            deviation: r.f32(),
            deviation_ew: r.char(),
            variation: r.f32(),
            variation_ew: r.char(),
        })
    }
}

impl NmeaEncodable for Hdg {
    const SENTENCE_TYPE: &str = "HDG";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.heading_mag);
        w.f32(self.deviation);
        w.char(self.deviation_ew);
        w.f32(self.variation);
        w.char(self.variation_ew);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn hdg_empty() {
        let frame = parse_frame("$SDHDG,,,,,*70").expect("valid");
        let hdg = Hdg::parse(&frame.fields).expect("parse HDG");
        assert!(hdg.heading_mag.is_none());
        assert!(hdg.deviation.is_none());
        assert!(hdg.deviation_ew.is_none());
        assert!(hdg.variation.is_none());
        assert!(hdg.variation_ew.is_none());
    }

    #[test]
    fn hdg_full_deviation_and_variation_signalk() {
        let frame = parse_frame("$INHDG,180,5,W,10,W*6D").expect("valid");
        let hdg = Hdg::parse(&frame.fields).expect("parse HDG");
        assert!((hdg.heading_mag.expect("hdg") - 180.0).abs() < 0.1);
        assert!((hdg.deviation.expect("dev") - 5.0).abs() < 0.1);
        assert_eq!(hdg.deviation_ew, Some('W'));
        assert!((hdg.variation.expect("var") - 10.0).abs() < 0.1);
        assert_eq!(hdg.variation_ew, Some('W'));
    }

    #[test]
    fn hdg_heading_only_signalk() {
        let frame = parse_frame("$HCHDG,51.5,,,,*73").expect("valid");
        let hdg = Hdg::parse(&frame.fields).expect("parse HDG");
        assert!((hdg.heading_mag.expect("hdg") - 51.5).abs() < 0.1);
        assert!(hdg.deviation.is_none());
        assert!(hdg.deviation_ew.is_none());
        assert!(hdg.variation.is_none());
        assert!(hdg.variation_ew.is_none());
    }

    #[test]
    fn hdg_pynmeagps() {
        let frame = parse_frame("$IIHDG,70.6,,,,W*2F").expect("valid pynmeagps HDG frame");
        let hdg = Hdg::parse(&frame.fields).expect("parse HDG");
        assert!((hdg.heading_mag.expect("hdg") - 70.6).abs() < 0.1);
        assert!(hdg.deviation.is_none());
        assert_eq!(hdg.variation_ew, Some('W'));
    }

    #[test]
    fn hdg_encode_roundtrip() {
        let hdg = Hdg {
            heading_mag: Some(181.9),
            deviation: Some(2.5),
            deviation_ew: Some('E'),
            variation: Some(0.6),
            variation_ew: Some('E'),
        };
        let sentence = hdg.to_sentence("SD").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let hdg2 = Hdg::parse(&frame.fields).expect("re-parse HDG");
        assert_eq!(hdg, hdg2);
    }

    #[test]
    fn hdg_with_variation_signalk() {
        let frame = parse_frame("$SDHDG,181.9,,,0.6,E*32").expect("valid");
        let hdg = Hdg::parse(&frame.fields).expect("parse HDG");
        assert!((hdg.heading_mag.expect("hdg") - 181.9).abs() < 0.1);
        assert!(hdg.deviation.is_none());
        assert!(hdg.deviation_ew.is_none());
        assert!((hdg.variation.expect("var") - 0.6).abs() < 0.1);
        assert_eq!(hdg.variation_ew, Some('E'));
    }
}
