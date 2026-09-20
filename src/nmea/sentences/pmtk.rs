use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PMTK001 — MTK GNSS Module Acknowledgement.
///
/// Wire: `cmd,flag`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PMTK001"`.
/// Encode with `to_sentence("")`.
#[derive(Debug, Clone, PartialEq)]
pub struct Pmtk {
    /// Command packet number being acknowledged.
    pub cmd: Option<u32>,
    /// Acknowledgement flag (0=invalid, 1=unsupported, 2=failed, 3=succeeded).
    pub flag: Option<u8>,
}

impl Pmtk {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let cmd = r.u32();
        let flag = r.u8();
        Some(Self { cmd, flag })
    }
}

impl NmeaEncodable for Pmtk {
    const SENTENCE_TYPE: &str = "PMTK001";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u32(self.cmd);
        w.u8(self.flag);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn pmtk_empty() {
        let s = Pmtk { cmd: None, flag: None }.to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Pmtk::parse(&f.fields).expect("parse");
        assert!(p.cmd.is_none());
        assert!(p.flag.is_none());
    }

    #[test]
    fn pmtk_encode_roundtrip() {
        let original = Pmtk {
            cmd: Some(604),
            flag: Some(3),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Pmtk::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn pmtk_values() {
        let frame = parse_frame("$PMTK001,604,3*32").expect("valid");
        let p = Pmtk::parse(&frame.fields).expect("parse");
        assert_eq!(p.cmd, Some(604));
        assert_eq!(p.flag, Some(3));
    }
}
