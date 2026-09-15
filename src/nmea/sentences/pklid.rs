use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PKLID — Kenwood Unit Identification (Long format).
///
/// Proprietary Kenwood sentence.
/// Wire: `$PKLID,version,fleet,unit_id,status,extension`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PKLID"`.
#[derive(Debug, Clone, PartialEq)]
pub struct Pklid {
    /// Protocol version.
    pub version: Option<String>,
    /// Fleet identifier.
    pub fleet: Option<String>,
    /// Unit identifier.
    pub unit_id: Option<String>,
    /// Status.
    pub status: Option<String>,
    /// Extension data.
    pub extension: Option<String>,
}

impl Pklid {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let version = r.string();
        let fleet = r.string();
        let unit_id = r.string();
        let status = r.string();
        let extension = r.string();
        Some(Self {
            version,
            fleet,
            unit_id,
            status,
            extension,
        })
    }
}

impl NmeaEncodable for Pklid {
    const SENTENCE_TYPE: &str = "PKLID";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.version.as_deref());
        w.string(self.fleet.as_deref());
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
    fn pklid_empty() {
        let s = Pklid {
            version: None,
            fleet: None,
            unit_id: None,
            status: None,
            extension: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Pklid::parse(&f.fields).expect("parse");
        assert!(p.version.is_none());
        assert!(p.extension.is_none());
    }

    #[test]
    fn pklid_encode_roundtrip() {
        let original = Pklid {
            version: Some("00".to_string()),
            fleet: Some("100".to_string()),
            unit_id: Some("2000".to_string()),
            status: Some("15".to_string()),
            extension: Some("00".to_string()),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Pklid::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn pklid_kenwood_gonmea() {
        let f = parse_frame("$PKLID,00,100,2000,15,00,*6D").expect("valid PKLID");
        let p = Pklid::parse(&f.fields).expect("parse PKLID");
        assert_eq!(p.version, Some("00".to_string()));
        assert_eq!(p.fleet, Some("100".to_string()));
        assert_eq!(p.unit_id, Some("2000".to_string()));
        assert_eq!(p.status, Some("15".to_string()));
        assert_eq!(p.extension, Some("00".to_string()));
    }
}
