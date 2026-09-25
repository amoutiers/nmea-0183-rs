use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// DSR — DSC transponder response.
///
/// Wire: `total,sentence,mmsi,(symbol,information)...,expansion`
#[derive(Debug, Clone, PartialEq)]
pub struct Dsr {
    pub total_sentences: Option<u8>,
    pub sentence_number: Option<u8>,
    /// Preserved as text because the wire format uses a fixed ten-character MMSI field.
    pub vessel_mmsi: Option<String>,
    /// Repeated DSC symbol-code/additional-information pairs, kept as text to retain leading zeroes.
    pub data_sets: Vec<(Option<String>, Option<String>)>,
    pub expansion_indicator: Option<char>,
}

impl Dsr {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let total_sentences = r.u8();
        let sentence_number = r.u8();
        let vessel_mmsi = r.string();

        let (data_fields, expansion_indicator) = match fields.get(3..) {
            Some(trailing) => match trailing.split_last() {
                Some((indicator, data_fields)) => (
                    data_fields,
                    indicator.chars().next().filter(|_| !indicator.is_empty()),
                ),
                None => (&[][..], None),
            },
            None => (&[][..], None),
        };
        let data_sets = data_fields
            .chunks(2)
            .map(|set| {
                (
                    set.first()
                        .filter(|value| !value.is_empty())
                        .map(|value| (*value).to_string()),
                    set.get(1)
                        .filter(|value| !value.is_empty())
                        .map(|value| (*value).to_string()),
                )
            })
            .collect();

        Some(Self {
            total_sentences,
            sentence_number,
            vessel_mmsi,
            data_sets,
            expansion_indicator,
        })
    }
}

impl NmeaEncodable for Dsr {
    const SENTENCE_TYPE: &str = "DSR";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.total_sentences);
        w.u8(self.sentence_number);
        w.string(self.vessel_mmsi.as_deref());
        for (symbol, information) in &self.data_sets {
            w.string(symbol.as_deref());
            w.string(information.as_deref());
        }
        w.char(self.expansion_indicator);
        w.finish()
    }
}
