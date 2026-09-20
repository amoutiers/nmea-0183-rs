use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ALR — Set Alarm State.
///
/// Wire: `time,alarm_id,condition,state,description`
///
/// Reference fixture: <https://github.com/adrianmo/go-nmea/blob/master/alr_test.go>.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Alr {
    /// Time of alarm condition change.
    pub time: Option<String>,
    /// Unique alarm number.
    pub alarm_id: Option<String>,
    /// Alarm condition ('A' = threshold exceeded, 'V' = not exceeded).
    pub condition: Option<char>,
    /// Alarm acknowledge state ('A' = acknowledged, 'V' = unacknowledged).
    pub state: Option<char>,
    /// Alarm description text.
    pub description: Option<String>,
}

impl Alr {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let time = r.string();
        let alarm_id = r.string();
        let condition = r.char();
        let state = r.char();
        let description = r.string();
        Self {
            time,
            alarm_id,
            condition,
            state,
            description,
        }
    }
}

impl NmeaEncodable for Alr {
    const SENTENCE_TYPE: &str = "ALR";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.string(self.alarm_id.as_deref());
        w.char(self.condition);
        w.char(self.state);
        w.string(self.description.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn alr_empty() {
        let s = Alr {
            time: None,
            alarm_id: None,
            condition: None,
            state: None,
            description: None,
        }
        .to_sentence("RA").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let a = Alr::parse(&f.fields);
        assert!(a.time.is_none());
        assert!(a.description.is_none());
    }

    #[test]
    fn alr_encode_roundtrip() {
        let original = Alr {
            time: Some("220516".to_string()),
            alarm_id: Some("001".to_string()),
            condition: Some('A'),
            state: Some('A'),
            description: Some("Bilge pump alarm1".to_string()),
        };
        let sentence = original.to_sentence("RA").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Alr::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn alr_raalr_gonmea() {
        let frame = parse_frame("$RAALR,220516,001,A,A,Bilge pump alarm1*4C").expect("valid");
        let a = Alr::parse(&frame.fields);
        assert_eq!(a.time.as_deref(), Some("220516"));
        assert_eq!(a.alarm_id.as_deref(), Some("001"));
        assert_eq!(a.condition, Some('A'));
        assert_eq!(a.state, Some('A'));
        assert_eq!(a.description.as_deref(), Some("Bilge pump alarm1"));
    }
}
