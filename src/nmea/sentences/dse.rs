use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// A single dataset within a DSE sentence.
#[derive(Debug, Clone, PartialEq)]
pub struct DseDataSet {
    /// Data code.
    pub code: Option<String>,
    /// Data value.
    pub data: Option<String>,
}

/// DSE — Expanded DSC.
///
/// Wire: `total,number,ack,mmsi[,code,data,…]`
/// Field layout follows the NMEA 0183 definitions catalogued by
/// [pynmeagps](https://github.com/semuconsulting/pynmeagps/blob/master/src/pynmeagps/nmeatypes_get.py).
#[derive(Debug, Clone, PartialEq)]
pub struct Dse {
    /// Total number of sentences.
    pub total: Option<u8>,
    /// Sentence number.
    pub number: Option<u8>,
    /// Acknowledgement (A=acknowledged, B=not acknowledged).
    pub ack: Option<char>,
    /// MMSI of sending station.
    pub mmsi: Option<String>,
    /// Expanded data sets (variable length, code+data pairs).
    pub datasets: Vec<DseDataSet>,
}

impl Dse {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let total = r.u8();
        let number = r.u8();
        let ack = r.char();
        let mmsi = r.string();
        let datasets = fields
            .get(4..)
            .unwrap_or(&[])
            .chunks(2)
            .map(|group| {
                let mut r = FieldReader::new(group);
                DseDataSet {
                    code: r.string(),
                    data: r.string(),
                }
            })
            .collect();
        Some(Self {
            total,
            number,
            ack,
            mmsi,
            datasets,
        })
    }
}

impl NmeaEncodable for Dse {
    const SENTENCE_TYPE: &str = "DSE";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.total);
        w.u8(self.number);
        w.char(self.ack);
        w.string(self.mmsi.as_deref());
        for ds in &self.datasets {
            w.string(ds.code.as_deref());
            w.string(ds.data.as_deref());
        }
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn dse_empty() {
        let s = Dse {
            total: None,
            number: None,
            ack: None,
            mmsi: None,
            datasets: vec![],
        }
        .to_sentence("CD")
        .expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let d = Dse::parse(&f.fields).expect("parse");
        assert!(d.total.is_none());
        assert!(d.mmsi.is_none());
        assert!(d.datasets.is_empty());
    }

    #[test]
    fn dse_encode_roundtrip() {
        let original = Dse {
            total: Some(1),
            number: Some(1),
            ack: Some('A'),
            mmsi: Some("3380400790".to_string()),
            datasets: vec![DseDataSet {
                code: Some("00".to_string()),
                data: Some("46504437".to_string()),
            }],
        };
        let sentence = original.to_sentence("CD").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Dse::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn dse_cddse_gonmea() {
        let f = parse_frame("$CDDSE,1,1,A,3380400790,00,46504437*15").expect("valid DSE");
        let d = Dse::parse(&f.fields).expect("parse DSE");
        assert_eq!(d.total, Some(1));
        assert_eq!(d.number, Some(1));
        assert_eq!(d.ack, Some('A'));
        assert_eq!(d.mmsi, Some("3380400790".to_string()));
        assert_eq!(d.datasets.len(), 1);
        assert_eq!(d.datasets[0].code, Some("00".to_string()));
        assert_eq!(d.datasets[0].data, Some("46504437".to_string()));
    }

    #[test]
    fn dse_multiple_datasets_gonmea() {
        let f =
            parse_frame("$CDDSE,1,1,A,3380400790,00,46504437,01,16501437*17").expect("valid DSE");
        let d = Dse::parse(&f.fields).expect("parse DSE");
        assert_eq!(d.mmsi, Some("3380400790".to_string()));
        assert_eq!(d.datasets.len(), 2);
        assert_eq!(d.datasets[0].code, Some("00".to_string()));
        assert_eq!(d.datasets[0].data, Some("46504437".to_string()));
        assert_eq!(d.datasets[1].code, Some("01".to_string()));
        assert_eq!(d.datasets[1].data, Some("16501437".to_string()));
    }
}
