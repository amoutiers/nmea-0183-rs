use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ZDL: Time and Distance to Variable Point.
///
/// Wire: `time,distance,point_type`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Zdl {
    /// Time to the variable point in hhmmss.ss format (00 to 99 hours).
    pub time: Option<String>,
    /// Distance to the variable point in nautical miles.
    pub distance: Option<f32>,
    /// Point type: collision ('C'), turning ('T'), reference ('R'), or wheel-over ('W').
    pub point_type: Option<char>,
}

impl Zdl {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            time: r.string(),
            distance: r.f32(),
            point_type: r.char(),
        }
    }
}

impl NmeaEncodable for Zdl {
    const SENTENCE_TYPE: &str = "ZDL";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.f32(self.distance);
        w.char(self.point_type);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn zdl_empty() {
        assert_eq!(Zdl::parse(&["", "", ""]), Zdl::default());
    }

    #[test]
    fn zdl_encode_roundtrip() {
        let original = Zdl {
            time: Some("990001.25".into()),
            distance: Some(0.125),
            point_type: Some('R'),
        };
        let sentence = original.to_sentence("GP").expect("encode ZDL");
        assert_eq!(sentence.split('*').next(), Some("$GPZDL,990001.25,0.125,R"));
        let frame = parse_frame(&sentence).expect("re-parse ZDL");
        assert_eq!(Zdl::parse(&frame.fields), original);
    }

    #[test]
    fn zdl_malformed_fields() {
        assert_eq!(Zdl::parse(&["", "invalid", "ROUTE"]), Zdl::default());
        assert_eq!(Zdl::parse(&["", "inf", ""]), Zdl::default());
    }

    #[test]
    fn zdl_missing_fields() {
        assert_eq!(Zdl::parse(&[]), Zdl::default());
        assert_eq!(Zdl::parse(&["383401"]), Zdl {
            time: Some("383401".into()),
            ..Default::default()
        });
    }

    #[test]
    fn zdl_values_marnav() {
        let frame = parse_frame("$GPZDL,383401,12.3,R*28").expect("valid marnav ZDL");
        let value = Zdl::parse(&frame.fields);
        assert_eq!(value.time.as_deref(), Some("383401"));
        assert!((value.distance.expect("distance") - 12.3).abs() < 1e-5);
        assert_eq!(value.point_type, Some('R'));
    }
}
