use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HDT — Heading True.
///
/// Wire: `headingT,T`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Hdt {
    /// True heading in degrees.
    pub heading_true: Option<f32>,
}

impl Hdt {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let heading_true = r.f32();
        r.skip(); // T
        Some(Self { heading_true })
    }
}

impl NmeaEncodable for Hdt {
    const SENTENCE_TYPE: &str = "HDT";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.heading_true);
        w.fixed('T');
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn hdt_empty() {
        let f = parse_frame("$IIHDT,,*58").expect("valid");
        let t = Hdt::parse(&f.fields).expect("parse");
        assert!(t.heading_true.is_none());
    }

    #[test]
    fn hdt_full_signalk() {
        let frame = parse_frame("$GPHDT,123.456,T*32").expect("valid");
        let hdt = Hdt::parse(&frame.fields).expect("parse HDT");
        assert!((hdt.heading_true.expect("hdg") - 123.456).abs() < 0.01);
    }

    #[test]
    fn hdt_he_talker_gpsd() {
        let frame = parse_frame("$HEHDT,4.0,T*2B").expect("valid");
        let hdt = Hdt::parse(&frame.fields).expect("parse HDT");
        assert!((hdt.heading_true.expect("hdg") - 4.0).abs() < 0.01);
    }

    #[test]
    fn hdt_pynmeagps() {
        let frame = parse_frame("$GPHDT,274.07,T*03").expect("valid pynmeagps HDT");
        let hdt = Hdt::parse(&frame.fields).expect("parse HDT");
        assert!((hdt.heading_true.expect("hdg") - 274.07).abs() < 0.01);
    }
    #[test]
    fn hdt_encode_roundtrip() {
        let hdt = Hdt {
            heading_true: Some(123.456),
        };
        let sentence = hdt.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let hdt2 = Hdt::parse(&frame.fields).expect("re-parse HDT");
        assert_eq!(hdt, hdt2);
    }
}
