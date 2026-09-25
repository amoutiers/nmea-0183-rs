use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Sfi {
    pub total_sentences: Option<u8>,
    pub sentence_number: Option<u8>,
    pub frequencies: Vec<(Option<u32>, Option<String>)>,
}

impl Sfi {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let total_sentences = r.u8();
        let sentence_number = r.u8();
        let frequencies = fields
            .iter()
            .skip(2)
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|pair| {
                (
                    pair.first().and_then(|value| value.parse().ok()),
                    pair.get(1)
                        .filter(|value| !value.is_empty())
                        .map(|value| (*value).to_string()),
                )
            })
            .collect();
        Some(Self {
            total_sentences,
            sentence_number,
            frequencies,
        })
    }
}

impl NmeaEncodable for Sfi {
    const SENTENCE_TYPE: &'static str = "SFI";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.total_sentences);
        w.u8(self.sentence_number);
        for (frequency, mode) in &self.frequencies {
            w.u32(*frequency);
            w.string(mode.as_deref());
        }
        w.finish()
    }
}
