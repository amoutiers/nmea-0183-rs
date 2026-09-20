use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// FIR — Fire Detection.
///
/// Wire: `fire_type,time,system,division1,division2,detector_number,condition,ack_state,message`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Fir {
    /// Fire detector type identifier.
    pub fire_type: Option<char>,
    /// UTC time (hhmmss).
    pub time: Option<String>,
    /// System identifier.
    pub system: Option<String>,
    /// First division identifier.
    pub division1: Option<String>,
    /// Second division identifier.
    pub division2: Option<String>,
    /// Detector number.
    pub detector_number: Option<String>,
    /// Alarm condition (A=alarm, N=normal).
    pub condition: Option<char>,
    /// Acknowledgement state (A=acknowledged, V=not acknowledged).
    pub ack_state: Option<char>,
    /// Message text.
    pub message: Option<String>,
}

impl Fir {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let fire_type = r.char();
        let time = r.string();
        let system = r.string();
        let division1 = r.string();
        let division2 = r.string();
        let detector_number = r.string();
        let condition = r.char();
        let ack_state = r.char();
        let message = r.string();
        Some(Self {
            fire_type,
            time,
            system,
            division1,
            division2,
            detector_number,
            condition,
            ack_state,
            message,
        })
    }
}

impl NmeaEncodable for Fir {
    const SENTENCE_TYPE: &str = "FIR";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.fire_type);
        w.string(self.time.as_deref());
        w.string(self.system.as_deref());
        w.string(self.division1.as_deref());
        w.string(self.division2.as_deref());
        w.string(self.detector_number.as_deref());
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
    fn fir_empty() {
        let s = Fir {
            fire_type: None,
            time: None,
            system: None,
            division1: None,
            division2: None,
            detector_number: None,
            condition: None,
            ack_state: None,
            message: None,
        }
        .to_sentence("FR").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let fir = Fir::parse(&f.fields).expect("parse");
        assert!(fir.fire_type.is_none());
        assert!(fir.time.is_none());
        assert!(fir.message.is_none());
    }

    #[test]
    fn fir_encode_roundtrip() {
        let original = Fir {
            fire_type: Some('E'),
            time: Some("103000".to_string()),
            system: Some("FD".to_string()),
            division1: Some("PT".to_string()),
            division2: Some("000".to_string()),
            detector_number: Some("007".to_string()),
            condition: Some('A'),
            ack_state: Some('V'),
            message: Some("Fire Alarm".to_string()),
        };
        let sentence = original.to_sentence("FR").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Fir::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn fir_frfir_gonmea() {
        let f = parse_frame(
            "$FRFIR,E,103000,FD,PT,000,007,A,V,Fire Alarm : TEST PT7 Name TEST DZ2 Name*7A",
        )
        .expect("valid FIR");
        let fir = Fir::parse(&f.fields).expect("parse FIR");
        assert_eq!(fir.fire_type, Some('E'));
        assert_eq!(fir.time, Some("103000".to_string()));
        assert_eq!(fir.system, Some("FD".to_string()));
        assert_eq!(fir.division1, Some("PT".to_string()));
        assert_eq!(fir.division2, Some("000".to_string()));
        assert_eq!(fir.detector_number, Some("007".to_string()));
        assert_eq!(fir.condition, Some('A'));
        assert_eq!(fir.ack_state, Some('V'));
        assert_eq!(
            fir.message,
            Some("Fire Alarm : TEST PT7 Name TEST DZ2 Name".to_string())
        );
    }
}
