use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ARC — Alert Response Command.
///
/// Wire: `time,manufacturer,alert_id,instance,command`
///
/// Reference fixture: <https://github.com/adrianmo/go-nmea/blob/master/arc_test.go>.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Arc {
    /// Time of message.
    pub time: Option<String>,
    /// Manufacturer mnemonic code.
    pub manufacturer: Option<String>,
    /// Alert identifier.
    pub alert_id: Option<String>,
    /// Alert instance.
    pub instance: Option<u8>,
    /// Alert command ('A'=acknowledge, 'Q'=request, 'O'=responsibility transfer).
    pub command: Option<char>,
}

impl Arc {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let time = r.string();
        let manufacturer = r.string();
        let alert_id = r.string();
        let instance = r.u8();
        let command = r.char();
        Some(Self {
            time,
            manufacturer,
            alert_id,
            instance,
            command,
        })
    }
}

impl NmeaEncodable for Arc {
    const SENTENCE_TYPE: &str = "ARC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.string(self.manufacturer.as_deref());
        w.string(self.alert_id.as_deref());
        w.u8(self.instance);
        w.char(self.command);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn arc_empty() {
        let s = Arc {
            time: None,
            manufacturer: None,
            alert_id: None,
            instance: None,
            command: None,
        }
        .to_sentence("RA").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let a = Arc::parse(&f.fields).expect("parse");
        assert!(a.time.is_none());
        assert!(a.command.is_none());
    }

    #[test]
    fn arc_encode_roundtrip() {
        let original = Arc {
            time: Some("220516".to_string()),
            manufacturer: Some("TCK".to_string()),
            alert_id: Some("002".to_string()),
            instance: Some(1),
            command: Some('A'),
        };
        let sentence = original.to_sentence("RA").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Arc::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn arc_raarc_gonmea() {
        let frame = parse_frame("$RAARC,220516,TCK,002,1,A*73").expect("valid");
        let a = Arc::parse(&frame.fields).expect("parse");
        assert_eq!(a.time.as_deref(), Some("220516"));
        assert_eq!(a.manufacturer.as_deref(), Some("TCK"));
        assert_eq!(a.alert_id.as_deref(), Some("002"));
        assert_eq!(a.instance, Some(1));
        assert_eq!(a.command, Some('A'));
    }
}
