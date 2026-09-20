use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ACK — Alert Acknowledge.
///
/// Wire: `alert_id`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ack {
    /// Alert identifier (e.g. "001").
    pub alert_id: Option<String>,
}

impl Ack {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let alert_id = r.string();
        Self { alert_id }
    }
}

impl NmeaEncodable for Ack {
    const SENTENCE_TYPE: &str = "ACK";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.alert_id.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn ack_empty() {
        let s = Ack { alert_id: None }.to_sentence("VR").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let a = Ack::parse(&f.fields);
        assert!(a.alert_id.is_none());
    }

    #[test]
    fn ack_encode_roundtrip() {
        let original = Ack {
            alert_id: Some("001".to_string()),
        };
        let sentence = original.to_sentence("VR").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Ack::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn ack_vrack_001_gonmea() {
        let frame = parse_frame("$VRACK,001*50").expect("valid");
        let a = Ack::parse(&frame.fields);
        assert_eq!(a.alert_id.as_deref(), Some("001"));
    }
}
