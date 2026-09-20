use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// RTE — Routes.
///
/// Wire: `num_sentences,sentence_num,mode,name[,ident,…]`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rte {
    /// Total number of sentences needed for this route.
    pub num_sentences: Option<u8>,
    /// Sentence number in sequence.
    pub sentence_num: Option<u8>,
    /// Message mode (c=complete, w=working route).
    pub mode: Option<char>,
    /// Route name.
    pub name: Option<String>,
    /// Waypoint identifiers in wire order, including empty positions.
    pub idents: Vec<String>,
}

impl Rte {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let num_sentences = r.u8();
        let sentence_num = r.u8();
        let mode = r.char();
        let name = r.string();
        let idents = fields.iter().skip(4).map(|field| (*field).to_owned()).collect();
        Self {
            num_sentences,
            sentence_num,
            mode,
            name,
            idents,
        }
    }
}

impl NmeaEncodable for Rte {
    const SENTENCE_TYPE: &str = "RTE";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.num_sentences);
        w.u8(self.sentence_num);
        w.char(self.mode);
        w.string(self.name.as_deref());
        for ident in &self.idents {
            w.string(Some(ident.as_str()));
        }
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn rte_empty() {
        let s = Rte {
            num_sentences: None,
            sentence_num: None,
            mode: None,
            name: None,
            idents: vec![],
        }
        .to_sentence("II").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let r = Rte::parse(&f.fields);
        assert!(r.num_sentences.is_none());
        assert!(r.name.is_none());
        assert!(r.idents.is_empty());
    }

    #[test]
    fn rte_encode_roundtrip() {
        let original = Rte {
            num_sentences: Some(4),
            sentence_num: Some(1),
            mode: Some('c'),
            name: Some("Rte 1".to_string()),
            idents: vec![
                "411".to_string(),
                "412".to_string(),
                "413".to_string(),
                "414".to_string(),
                "415".to_string(),
            ],
        };
        let sentence = original.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Rte::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn rte_iirte_gonmea() {
        let f = parse_frame("$IIRTE,4,1,c,Rte 1,411,412,413,414,415*6F").expect("valid RTE");
        let r = Rte::parse(&f.fields);
        assert_eq!(r.num_sentences, Some(4));
        assert_eq!(r.sentence_num, Some(1));
        assert_eq!(r.mode, Some('c'));
        assert_eq!(r.name, Some("Rte 1".to_string()));
        assert_eq!(r.idents.len(), 5);
        assert_eq!(r.idents[0], "411");
        assert_eq!(r.idents[4], "415");
    }
}
