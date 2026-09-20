use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HBT — Heartbeat Supervision.
///
/// Wire: `interval,operation_status,msg_id`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Hbt {
    /// Configured repeat interval in seconds.
    pub interval: Option<f32>,
    /// Equipment operation status ('A' = normal).
    pub operation_status: Option<char>,
    /// Sequential message identifier (0–9).
    pub msg_id: Option<u8>,
}

impl Hbt {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let interval = r.f32();
        let operation_status = r.char();
        let msg_id = r.u8();
        Self {
            interval,
            operation_status,
            msg_id,
        }
    }
}

impl NmeaEncodable for Hbt {
    const SENTENCE_TYPE: &str = "HBT";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.interval);
        w.char(self.operation_status);
        w.u8(self.msg_id);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn hbt_empty() {
        let s = Hbt {
            interval: None,
            operation_status: None,
            msg_id: None,
        }
        .to_sentence("HC").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let h = Hbt::parse(&f.fields);
        assert!(h.interval.is_none());
        assert!(h.operation_status.is_none());
        assert!(h.msg_id.is_none());
    }

    #[test]
    fn hbt_encode_roundtrip() {
        let original = Hbt {
            interval: Some(1.5),
            operation_status: Some('A'),
            msg_id: Some(1),
        };
        let sentence = original.to_sentence("HC").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Hbt::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn hbt_hchbt_gonmea() {
        let frame = parse_frame("$HCHBT,1.5,A,1*23").expect("valid");
        let h = Hbt::parse(&frame.fields);
        assert!((h.interval.expect("interval") - 1.5).abs() < 0.01);
        assert_eq!(h.operation_status, Some('A'));
        assert_eq!(h.msg_id, Some(1));
    }
}
