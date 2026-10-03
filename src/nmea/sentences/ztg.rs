use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ZTG: UTC and Time to Destination Waypoint.
///
/// Wire: `observation_time,time_to_go,destination_waypoint`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ztg {
    /// UTC time of observation in hhmmss.ss format.
    pub observation_time: Option<String>,
    /// Predicted time to the waypoint in hhmmss.ss format (00 to 99 hours).
    pub time_to_go: Option<String>,
    /// Destination waypoint identifier.
    pub destination_waypoint: Option<String>,
}

impl Ztg {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or empty fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            observation_time: r.string(),
            time_to_go: r.string(),
            destination_waypoint: r.string(),
        }
    }
}

impl NmeaEncodable for Ztg {
    const SENTENCE_TYPE: &str = "ZTG";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.observation_time.as_deref());
        w.string(self.time_to_go.as_deref());
        w.string(self.destination_waypoint.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn ztg_empty() {
        let frame = parse_frame("$GPZTG,,,*72").expect("valid empty ZTG");
        assert_eq!(Ztg::parse(&frame.fields), Ztg::default());
    }

    #[test]
    fn ztg_encode_roundtrip() {
        let original = Ztg {
            observation_time: Some("235959.99".into()),
            time_to_go: Some("990000.5".into()),
            destination_waypoint: Some("DEST".into()),
        };
        let sentence = original.to_sentence("GP").expect("encode ZTG");
        let frame = parse_frame(&sentence).expect("re-parse ZTG");
        assert_eq!(Ztg::parse(&frame.fields), original);
    }

    #[test]
    fn ztg_missing_fields() {
        assert_eq!(Ztg::parse(&[]), Ztg::default());
        assert_eq!(
            Ztg::parse(&["123456.1"]),
            Ztg {
                observation_time: Some("123456.1".into()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn ztg_values_marnav() {
        let frame = parse_frame("$GPZTG,123456.1,000010,POINT1*16")
            .expect("valid marnav ZTG vector");
        assert_eq!(
            Ztg::parse(&frame.fields),
            Ztg {
                observation_time: Some("123456.1".into()),
                time_to_go: Some("000010".into()),
                destination_waypoint: Some("POINT1".into()),
            }
        );
    }
}
