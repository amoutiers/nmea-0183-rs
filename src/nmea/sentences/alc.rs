use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// A single alert entry within an ALC sentence.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AlcEntry {
    /// Manufacturer mnemonic code.
    pub manufacturer: Option<String>,
    /// Alert identifier.
    pub alert_id: Option<String>,
    /// Alert instance number.
    pub instance: Option<u8>,
    /// Alert revision counter.
    pub revision: Option<u8>,
}

/// ALC — Cyclic Alert List.
///
/// Wire: `num_frags,frag_num,msg_id,entries_num[,manufacturer,alert_id,instance,revision,…]`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Alc {
    /// Total number of sentences (fragments) needed for this message.
    pub num_frags: Option<u8>,
    /// Fragment number of this sentence.
    pub frag_num: Option<u8>,
    /// Sequential message identifier.
    pub msg_id: Option<u8>,
    /// Number of alert entries in this sentence.
    pub entries_num: Option<u8>,
    /// Alert entries (variable length).
    pub entries: Vec<AlcEntry>,
}

impl Alc {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let num_frags = r.u8();
        let frag_num = r.u8();
        let msg_id = r.u8();
        let entries_num = r.u8();
        let entries = fields
            .get(4..)
            .unwrap_or(&[])
            .chunks(4)
            .map(|group| {
                let mut r = FieldReader::new(group);
                AlcEntry {
                    manufacturer: r.string(),
                    alert_id: r.string(),
                    instance: r.u8(),
                    revision: r.u8(),
                }
            })
            .collect();
        Self {
            num_frags,
            frag_num,
            msg_id,
            entries_num,
            entries,
        }
    }
}

impl NmeaEncodable for Alc {
    const SENTENCE_TYPE: &str = "ALC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.num_frags);
        w.u8(self.frag_num);
        w.u8(self.msg_id);
        w.u8(self.entries_num);
        for entry in &self.entries {
            w.string(entry.manufacturer.as_deref());
            w.string(entry.alert_id.as_deref());
            w.u8(entry.instance);
            w.u8(entry.revision);
        }
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn alc_empty() {
        let s = Alc {
            num_frags: None,
            frag_num: None,
            msg_id: None,
            entries_num: None,
            entries: vec![],
        }
        .to_sentence("FB").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let a = Alc::parse(&f.fields);
        assert!(a.num_frags.is_none());
        assert!(a.entries.is_empty());
    }

    #[test]
    fn alc_encode_roundtrip() {
        let original = Alc {
            num_frags: Some(2),
            frag_num: Some(1),
            msg_id: Some(3),
            entries_num: Some(1),
            entries: vec![AlcEntry {
                manufacturer: Some("FEB".to_string()),
                alert_id: Some("01".to_string()),
                instance: Some(2),
                revision: Some(3),
            }],
        };
        let sentence = original.to_sentence("FB").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Alc::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn alc_fbalc_gonmea() {
        let f = parse_frame("$FBALC,02,01,03,01,FEB,01,02,03*0A").expect("valid ALC");
        let a = Alc::parse(&f.fields);
        assert_eq!(a.num_frags, Some(2));
        assert_eq!(a.frag_num, Some(1));
        assert_eq!(a.msg_id, Some(3));
        assert_eq!(a.entries_num, Some(1));
        assert_eq!(a.entries.len(), 1);
        assert_eq!(a.entries[0].manufacturer, Some("FEB".to_string()));
        assert_eq!(a.entries[0].alert_id, Some("01".to_string()));
        assert_eq!(a.entries[0].instance, Some(2));
        assert_eq!(a.entries[0].revision, Some(3));
    }

    #[test]
    fn alc_multiple_entries_gonmea() {
        let f = parse_frame("$FBALC,02,01,03,02,FEB,01,02,03,TEB,02,03,04*5F").expect("valid ALC");
        let a = Alc::parse(&f.fields);
        assert_eq!(a.entries_num, Some(2));
        assert_eq!(a.entries.len(), 2);
        assert_eq!(a.entries[0].manufacturer, Some("FEB".to_string()));
        assert_eq!(a.entries[0].alert_id, Some("01".to_string()));
        assert_eq!(a.entries[0].instance, Some(2));
        assert_eq!(a.entries[0].revision, Some(3));
        assert_eq!(a.entries[1].manufacturer, Some("TEB".to_string()));
        assert_eq!(a.entries[1].alert_id, Some("02".to_string()));
        assert_eq!(a.entries[1].instance, Some(3));
        assert_eq!(a.entries[1].revision, Some(4));
    }

    #[test]
    fn alc_no_entries_gonmea() {
        let f = parse_frame("$FBALC,02,01,03,00*4A").expect("valid ALC");
        let a = Alc::parse(&f.fields);
        assert_eq!(a.num_frags, Some(2));
        assert_eq!(a.msg_id, Some(3));
        assert_eq!(a.entries_num, Some(0));
        assert!(a.entries.is_empty());
    }
}
