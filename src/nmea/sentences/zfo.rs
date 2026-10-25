use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ZFO: UTC and Time from Origin Waypoint.
///
/// Wire: `observation_time,elapsed_time,origin_waypoint`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Zfo {
    /// UTC time of observation in hhmmss.ss format.
    pub observation_time: Option<String>,
    /// Elapsed time from the origin waypoint in hhmmss.ss format (00 to 99 hours).
    pub elapsed_time: Option<String>,
    /// Origin waypoint identifier.
    pub origin_waypoint: Option<String>,
}

impl Zfo {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            observation_time: r.string(),
            elapsed_time: r.string(),
            origin_waypoint: r.string(),
        }
    }
}

impl NmeaEncodable for Zfo {
    const SENTENCE_TYPE: &str = "ZFO";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.observation_time.as_deref());
        w.string(self.elapsed_time.as_deref());
        w.string(self.origin_waypoint.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn zfo_empty() {
        assert_eq!(Zfo::parse(&["", "", ""]), Zfo::default());
    }

    #[test]
    fn zfo_encode_roundtrip() {
        let original = Zfo {
            observation_time: Some("235959.99".into()),
            elapsed_time: Some("990001.25".into()),
            origin_waypoint: Some("ORIGIN".into()),
        };
        let sentence = original.to_sentence("GP").expect("encode ZFO");
        assert_eq!(sentence.split('*').next(), Some("$GPZFO,235959.99,990001.25,ORIGIN"));
        let frame = parse_frame(&sentence).expect("re-parse ZFO");
        assert_eq!(Zfo::parse(&frame.fields), original);
    }

    #[test]
    fn zfo_missing_fields() {
        assert_eq!(Zfo::parse(&[]), Zfo::default());
        assert_eq!(Zfo::parse(&["123456.1"]), Zfo {
            observation_time: Some("123456.1".into()),
            ..Default::default()
        });
    }

    #[test]
    fn zfo_values_marnav() {
        let frame = parse_frame("$GPZFO,123456.1,000010,POINT1*0C").expect("valid marnav ZFO");
        let value = Zfo::parse(&frame.fields);
        assert_eq!(value.observation_time.as_deref(), Some("123456.1"));
        assert_eq!(value.elapsed_time.as_deref(), Some("000010"));
        assert_eq!(value.origin_waypoint.as_deref(), Some("POINT1"));
    }
}
