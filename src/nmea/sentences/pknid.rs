use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PKNID — Kenwood Unit Identification (Normal format).
///
/// Proprietary Kenwood sentence.
/// Wire: `$PKNID,version,unit_id,status,extension`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PKNID"`.
#[derive(Debug, Clone, PartialEq)]
pub struct Pknid {
    /// Protocol version.
    pub version: Option<String>,
    /// Unit identifier.
    pub unit_id: Option<String>,
    /// Status.
    pub status: Option<String>,
    /// Extension data.
    pub extension: Option<String>,
}

impl Pknid {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let version = r.string();
        let unit_id = r.string();
        let status = r.string();
        let extension = r.string();
        Some(Self {
            version,
            unit_id,
            status,
            extension,
        })
    }
}

impl NmeaEncodable for Pknid {
    const SENTENCE_TYPE: &str = "PKNID";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.version.as_deref());
        w.string(self.unit_id.as_deref());
        w.string(self.status.as_deref());
        w.string(self.extension.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn pknid_empty() {
        let s = Pknid {
            version: None,
            unit_id: None,
            status: None,
            extension: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Pknid::parse(&f.fields).expect("parse");
        assert!(p.version.is_none());
        assert!(p.extension.is_none());
    }

    #[test]
    fn pknid_encode_roundtrip() {
        let original = Pknid {
            version: Some("00".to_string()),
            unit_id: Some("U00001".to_string()),
            status: Some("015".to_string()),
            extension: Some("00".to_string()),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Pknid::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn pknid_values() {
        let f = parse_frame("$PKNID,00,U00001,015,00,*24").expect("valid PKNID");
        let p = Pknid::parse(&f.fields).expect("parse PKNID");
        assert_eq!(p.version, Some("00".to_string()));
        assert_eq!(p.unit_id, Some("U00001".to_string()));
        assert_eq!(p.status, Some("015".to_string()));
        assert_eq!(p.extension, Some("00".to_string()));
    }
}
