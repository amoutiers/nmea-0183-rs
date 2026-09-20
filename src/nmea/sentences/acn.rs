use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ACN — Alert Command.
///
/// Wire: `time,manufacturer,alert_id,instance,command,state`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Acn {
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
    /// Alert state ('A'=active, 'S'=silenced, 'U'=unacknowledged, 'R'=rectified).
    pub state: Option<char>,
}

impl Acn {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let time = r.string();
        let manufacturer = r.string();
        let alert_id = r.string();
        let instance = r.u8();
        let command = r.char();
        let state = r.char();
        Some(Self {
            time,
            manufacturer,
            alert_id,
            instance,
            command,
            state,
        })
    }
}

impl NmeaEncodable for Acn {
    const SENTENCE_TYPE: &str = "ACN";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.string(self.manufacturer.as_deref());
        w.string(self.alert_id.as_deref());
        w.u8(self.instance);
        w.char(self.command);
        w.char(self.state);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn acn_empty() {
        let s = Acn {
            time: None,
            manufacturer: None,
            alert_id: None,
            instance: None,
            command: None,
            state: None,
        }
        .to_sentence("RA").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let a = Acn::parse(&f.fields).expect("parse");
        assert!(a.time.is_none());
        assert!(a.command.is_none());
    }

    #[test]
    fn acn_encode_roundtrip() {
        let original = Acn {
            time: Some("220516".to_string()),
            manufacturer: Some("TCK".to_string()),
            alert_id: Some("002".to_string()),
            instance: Some(1),
            command: Some('A'),
            state: Some('C'),
        };
        let sentence = original.to_sentence("RA").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Acn::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn acn_raacn_gonmea() {
        let frame = parse_frame("$RAACN,220516,TCK,002,1,A,C*00").expect("valid");
        let a = Acn::parse(&frame.fields).expect("parse");
        assert_eq!(a.time.as_deref(), Some("220516"));
        assert_eq!(a.manufacturer.as_deref(), Some("TCK"));
        assert_eq!(a.alert_id.as_deref(), Some("002"));
        assert_eq!(a.instance, Some(1));
        assert_eq!(a.command, Some('A'));
        assert_eq!(a.state, Some('C'));
    }
}
