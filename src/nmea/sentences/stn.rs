use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// STN: Multiple Data ID.
///
/// Wire: `talker_id`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Stn {
    /// Multiple data identifier, retained as text to preserve leading zeros.
    pub talker_id: Option<String>,
}

impl Stn {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            talker_id: r.string(),
        }
    }
}

impl NmeaEncodable for Stn {
    const SENTENCE_TYPE: &str = "STN";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.talker_id.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn stn_empty() {
        assert_eq!(Stn::parse(&[""]), Stn::default());
    }

    #[test]
    fn stn_encode_roundtrip() {
        let original = Stn { talker_id: Some("07".into()) };
        let sentence = original.to_sentence("GP").expect("encode STN");
        assert_eq!(sentence.split('*').next(), Some("$GPSTN,07"));
        let frame = parse_frame(&sentence).expect("re-parse STN");
        assert_eq!(Stn::parse(&frame.fields), original);
    }

    #[test]
    fn stn_missing_fields() {
        assert_eq!(Stn::parse(&[]), Stn::default());
    }

    #[test]
    fn stn_values_pynmeagps() {
        let frame = parse_frame("$GPSTN,34*75").expect("valid pynmeagps STN");
        assert_eq!(Stn::parse(&frame.fields).talker_id.as_deref(), Some("34"));
    }
}
