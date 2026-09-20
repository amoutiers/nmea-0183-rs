use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ALA — System Faults and Alarms.
///
/// Wire: `time,system,subsystem,instance,alarm_type,condition,ack_state,message`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ala {
    /// UTC time of alarm (hhmmss).
    pub time: Option<String>,
    /// System identifier.
    pub system: Option<String>,
    /// Subsystem identifier.
    pub subsystem: Option<String>,
    /// Instance identifier.
    pub instance: Option<String>,
    /// Alarm type code.
    pub alarm_type: Option<String>,
    /// Alarm condition (N=normal, A=alarm, W=warning).
    pub condition: Option<char>,
    /// Acknowledgement state (A=acknowledged, V=not acknowledged).
    pub ack_state: Option<char>,
    /// Alarm message text.
    pub message: Option<String>,
}

impl Ala {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let time = r.string();
        let system = r.string();
        let subsystem = r.string();
        let instance = r.string();
        let alarm_type = r.string();
        let condition = r.char();
        let ack_state = r.char();
        let message = r.string();
        Some(Self {
            time,
            system,
            subsystem,
            instance,
            alarm_type,
            condition,
            ack_state,
            message,
        })
    }
}

impl NmeaEncodable for Ala {
    const SENTENCE_TYPE: &str = "ALA";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.string(self.system.as_deref());
        w.string(self.subsystem.as_deref());
        w.string(self.instance.as_deref());
        w.string(self.alarm_type.as_deref());
        w.char(self.condition);
        w.char(self.ack_state);
        w.string(self.message.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn ala_empty() {
        let s = Ala {
            time: None,
            system: None,
            subsystem: None,
            instance: None,
            alarm_type: None,
            condition: None,
            ack_state: None,
            message: None,
        }
        .to_sentence("FR").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let a = Ala::parse(&f.fields).expect("parse");
        assert!(a.time.is_none());
        assert!(a.system.is_none());
        assert!(a.condition.is_none());
        assert!(a.message.is_none());
    }

    #[test]
    fn ala_encode_roundtrip() {
        let original = Ala {
            time: Some("143955".to_string()),
            system: Some("FR".to_string()),
            subsystem: Some("OT".to_string()),
            instance: Some("00".to_string()),
            alarm_type: Some("901".to_string()),
            condition: Some('N'),
            ack_state: Some('V'),
            message: Some("Test alarm".to_string()),
        };
        let sentence = original.to_sentence("FR").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Ala::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn ala_frala_gonmea() {
        let f = parse_frame("$FRALA,143955,FR,OT,00,901,N,V,Syst Fault : AutroSafe comm. OK*4F")
            .expect("valid ALA");
        let a = Ala::parse(&f.fields).expect("parse ALA");
        assert_eq!(a.time, Some("143955".to_string()));
        assert_eq!(a.system, Some("FR".to_string()));
        assert_eq!(a.subsystem, Some("OT".to_string()));
        assert_eq!(a.instance, Some("00".to_string()));
        assert_eq!(a.alarm_type, Some("901".to_string()));
        assert_eq!(a.condition, Some('N'));
        assert_eq!(a.ack_state, Some('V'));
        assert_eq!(
            a.message,
            Some("Syst Fault : AutroSafe comm. OK".to_string())
        );
    }
}
