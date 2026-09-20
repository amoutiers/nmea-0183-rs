use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// EVE — General Event Message.
///
/// Wire: `time,tag_code,message`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Eve {
    /// Event time.
    pub time: Option<String>,
    /// Tag/code identifier.
    pub tag_code: Option<String>,
    /// Event message text.
    pub message: Option<String>,
}

impl Eve {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let time = r.string();
        let tag_code = r.string();
        let message = r.string();
        Self {
            time,
            tag_code,
            message,
        }
    }
}

impl NmeaEncodable for Eve {
    const SENTENCE_TYPE: &str = "EVE";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.string(self.tag_code.as_deref());
        w.string(self.message.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn eve_empty() {
        let s = Eve {
            time: None,
            tag_code: None,
            message: None,
        }
        .to_sentence("FR").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let e = Eve::parse(&f.fields);
        assert!(e.time.is_none());
        assert!(e.message.is_none());
    }

    #[test]
    fn eve_encode_roundtrip() {
        let original = Eve {
            time: Some("000001".to_string()),
            tag_code: Some("DZ00513".to_string()),
            message: Some("Fire Alarm".to_string()),
        };
        let sentence = original.to_sentence("FR").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Eve::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn eve_freve_gonmea() {
        let frame =
            parse_frame("$FREVE,000001,DZ00513,Fire Alarm On: TEST DZ201 Name*0A").expect("valid");
        let e = Eve::parse(&frame.fields);
        assert_eq!(e.time.as_deref(), Some("000001"));
        assert_eq!(e.tag_code.as_deref(), Some("DZ00513"));
        assert_eq!(
            e.message.as_deref(),
            Some("Fire Alarm On: TEST DZ201 Name")
        );
    }
}
