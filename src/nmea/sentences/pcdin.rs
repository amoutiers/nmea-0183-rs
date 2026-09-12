use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PCDIN — SeaSmart.Net NMEA 2000 to NMEA 0183 Protocol.
///
/// Wire: `pgn,timestamp,source,data`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PCDIN"`.
/// Encode with `to_sentence("")`.
#[derive(Debug, Clone, PartialEq)]
pub struct Pcdin {
    /// PGN (Parameter Group Number) in hex.
    pub pgn: Option<String>,
    /// Timestamp in hex milliseconds.
    pub timestamp: Option<String>,
    /// Source address in hex.
    pub source: Option<String>,
    /// Raw data payload in hex.
    pub data: Option<String>,
}

impl Pcdin {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let pgn = r.string();
        let timestamp = r.string();
        let source = r.string();
        let data = r.string();
        Some(Self {
            pgn,
            timestamp,
            source,
            data,
        })
    }
}

impl NmeaEncodable for Pcdin {
    const SENTENCE_TYPE: &str = "PCDIN";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.pgn.as_deref());
        w.string(self.timestamp.as_deref());
        w.string(self.source.as_deref());
        w.string(self.data.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn pcdin_empty() {
        let s = Pcdin {
            pgn: None,
            timestamp: None,
            source: None,
            data: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Pcdin::parse(&f.fields).expect("parse");
        assert!(p.pgn.is_none());
        assert!(p.data.is_none());
    }

    #[test]
    fn pcdin_encode_roundtrip() {
        let original = Pcdin {
            pgn: Some("01F112".to_string()),
            timestamp: Some("000C72EA".to_string()),
            source: Some("09".to_string()),
            data: Some("28C36A0000B40AFD".to_string()),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Pcdin::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn pcdin_pcdin_gonmea() {
        let frame =
            parse_frame("$PCDIN,01F112,000C72EA,09,28C36A0000B40AFD*56").expect("valid");
        let p = Pcdin::parse(&frame.fields).expect("parse");
        assert_eq!(p.pgn.as_deref(), Some("01F112"));
        assert_eq!(p.timestamp.as_deref(), Some("000C72EA"));
        assert_eq!(p.source.as_deref(), Some("09"));
        assert_eq!(p.data.as_deref(), Some("28C36A0000B40AFD"));
    }
}
