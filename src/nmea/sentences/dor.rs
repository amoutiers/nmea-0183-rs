use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// DOR — Door Status Detection.
///
/// Field layout follows NMEA 0183 DOR and the pynmeagps sentence definitions.
///
/// Wire: `door_type,time,system,division1,division2,door_number,door_status,switch_setting,message`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dor {
    /// Door type identifier character.
    pub door_type: Option<char>,
    /// UTC time (hhmmss).
    pub time: Option<String>,
    /// System identifier.
    pub system: Option<String>,
    /// First division identifier.
    pub division1: Option<String>,
    /// Second division identifier.
    pub division2: Option<String>,
    /// Door number.
    pub door_number: Option<String>,
    /// Door status (C=closed, O=open).
    pub door_status: Option<char>,
    /// Switch setting (C=closed, O=open).
    pub switch_setting: Option<char>,
    /// Message text.
    pub message: Option<String>,
}

impl Dor {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let door_type = r.char();
        let time = r.string();
        let system = r.string();
        let division1 = r.string();
        let division2 = r.string();
        let door_number = r.string();
        let door_status = r.char();
        let switch_setting = r.char();
        let message = r.string();
        Some(Self {
            door_type,
            time,
            system,
            division1,
            division2,
            door_number,
            door_status,
            switch_setting,
            message,
        })
    }
}

impl NmeaEncodable for Dor {
    const SENTENCE_TYPE: &str = "DOR";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.door_type);
        w.string(self.time.as_deref());
        w.string(self.system.as_deref());
        w.string(self.division1.as_deref());
        w.string(self.division2.as_deref());
        w.string(self.door_number.as_deref());
        w.char(self.door_status);
        w.char(self.switch_setting);
        w.string(self.message.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn dor_empty() {
        let s = Dor {
            door_type: None,
            time: None,
            system: None,
            division1: None,
            division2: None,
            door_number: None,
            door_status: None,
            switch_setting: None,
            message: None,
        }
        .to_sentence("FR")
        .expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let d = Dor::parse(&f.fields).expect("parse");
        assert!(d.door_type.is_none());
        assert!(d.time.is_none());
        assert!(d.message.is_none());
    }

    #[test]
    fn dor_encode_roundtrip() {
        let original = Dor {
            door_type: Some('E'),
            time: Some("233042".to_string()),
            system: Some("FD".to_string()),
            division1: Some("FP".to_string()),
            division2: Some("000".to_string()),
            door_number: Some("010".to_string()),
            door_status: Some('C'),
            switch_setting: Some('C'),
            message: Some("Door Closed".to_string()),
        };
        let sentence = original.to_sentence("FR").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Dor::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn dor_frdor_gonmea() {
        let f =
            parse_frame("$FRDOR,E,233042,FD,FP,000,010,C,C,Door Closed : TEST FPA Name*4D")
                .expect("valid DOR");
        let d = Dor::parse(&f.fields).expect("parse DOR");
        assert_eq!(d.door_type, Some('E'));
        assert_eq!(d.time, Some("233042".to_string()));
        assert_eq!(d.system, Some("FD".to_string()));
        assert_eq!(d.division1, Some("FP".to_string()));
        assert_eq!(d.division2, Some("000".to_string()));
        assert_eq!(d.door_number, Some("010".to_string()));
        assert_eq!(d.door_status, Some('C'));
        assert_eq!(d.switch_setting, Some('C'));
        assert_eq!(
            d.message,
            Some("Door Closed : TEST FPA Name".to_string())
        );
    }
}
