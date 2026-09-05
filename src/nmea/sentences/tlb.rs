use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// A single tracked target entry within a TLB sentence.
#[derive(Debug, Clone, PartialEq)]
pub struct TlbTarget {
    /// Target number.
    pub number: Option<u8>,
    /// Target label.
    pub label: Option<String>,
}

/// TLB — Target Label.
///
/// Wire: repeating pairs `number,label[,number,label,…]`
#[derive(Debug, Clone, PartialEq)]
pub struct Tlb {
    /// Target number/label pairs (variable length).
    pub targets: Vec<TlbTarget>,
}

impl Tlb {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let mut targets = Vec::new();
        loop {
            let number = r.u8();
            if number.is_none() {
                break;
            }
            targets.push(TlbTarget {
                number,
                label: r.string(),
            });
        }
        Some(Self { targets })
    }
}

impl NmeaEncodable for Tlb {
    const SENTENCE_TYPE: &str = "TLB";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        for t in &self.targets {
            w.u8(t.number);
            w.string(t.label.as_deref());
        }
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn tlb_empty() {
        let s = Tlb { targets: vec![] }.to_sentence("RA").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let t = Tlb::parse(&f.fields).expect("parse");
        assert!(t.targets.is_empty());
    }

    #[test]
    fn tlb_encode_roundtrip() {
        let original = Tlb {
            targets: vec![
                TlbTarget {
                    number: Some(1),
                    label: Some("ALPHA".to_string()),
                },
                TlbTarget {
                    number: Some(2),
                    label: Some("BETA".to_string()),
                },
            ],
        };
        let sentence = original.to_sentence("RA").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Tlb::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn tlb_ratlb_gonmea() {
        let f = parse_frame("$RATLB,1,XXX*20").expect("valid TLB");
        let t = Tlb::parse(&f.fields).expect("parse TLB");
        assert_eq!(t.targets.len(), 1);
        assert_eq!(t.targets[0].number, Some(1));
        assert_eq!(t.targets[0].label, Some("XXX".to_string()));
    }
}
