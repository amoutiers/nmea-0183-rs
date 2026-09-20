use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// DPT — Depth of Water.
///
/// Wire: `depth,offset,rangescale`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dpt {
    /// Water depth below transducer in meters.
    pub depth: Option<f32>,
    /// Offset from transducer in meters (positive = waterline, negative = keel).
    pub offset: Option<f32>,
    /// Maximum range scale in use in meters.
    pub rangescale: Option<f32>,
}

impl Dpt {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            depth: r.f32(),
            offset: r.f32(),
            rangescale: r.f32(),
        })
    }
}

impl NmeaEncodable for Dpt {
    const SENTENCE_TYPE: &str = "DPT";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.depth);
        w.f32(self.offset);
        w.f32(self.rangescale);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn dpt_empty() {
        let f = parse_frame("$IIDPT,,,*6C").expect("valid");
        let d = Dpt::parse(&f.fields).expect("parse");
        assert!(d.depth.is_none());
        assert!(d.offset.is_none());
        assert!(d.rangescale.is_none());
    }

    #[test]
    fn dpt_gpsd() {
        let f = parse_frame("$INDPT,2.2,0.0*47").expect("valid DPT frame");
        let d = Dpt::parse(&f.fields).expect("parse DPT");
        assert!((d.depth.expect("depth present") - 2.2).abs() < 0.01);
    }

    #[test]
    fn dpt_negative_offset_signalk() {
        let f = parse_frame("$IIDPT,4.1,-1.0*69").expect("valid DPT frame");
        let d = Dpt::parse(&f.fields).expect("parse DPT");
        assert!((d.offset.expect("offset present") - (-1.0)).abs() < 0.01);
    }

    #[test]
    fn dpt_parse_signalk() {
        let f = parse_frame("$IIDPT,4.1,0.0*45").expect("valid DPT frame");
        let d = Dpt::parse(&f.fields).expect("parse DPT");
        assert!((d.depth.expect("depth present") - 4.1).abs() < 0.01);
    }

    #[test]
    fn dpt_encode_exact_fields() {
        let dpt = Dpt { depth: Some(4.1), offset: Some(0.0), rangescale: None };
        assert_eq!(dpt.encode().expect("encode"), vec!["4.1", "0", ""]);
    }

    #[test]
    fn dpt_encode_roundtrip() {
        let d = Dpt {
            depth: Some(4.1),
            offset: Some(1.0),
            rangescale: None,
        };
        let s = d.to_sentence("II").expect("encode");
        let f = parse_frame(s.trim()).expect("re-parse DPT frame");
        let d2 = Dpt::parse(&f.fields).expect("re-parse DPT");
        assert_eq!(d, d2);
    }
}
