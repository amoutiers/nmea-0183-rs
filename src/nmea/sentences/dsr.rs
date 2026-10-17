use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// DSR — DSC transponder response.
///
/// Wire: `total,sentence,mmsi,(symbol,information)...,expansion`
#[derive(Debug, Clone, PartialEq, Default)]
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
    /// Parse fields from a decoded NMEA frame; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let total_sentences = r.u8();
        let sentence_number = r.u8();
        let vessel_mmsi = r.string();

        let (data_fields, expansion_indicator) = match fields.get(3..) {
            Some(trailing) => match trailing.split_last() {
                Some((indicator, data_fields)) => (
                    data_fields,
                    FieldReader::new(&[*indicator]).char(),
                ),
                None => (&[][..], None),
            },
            None => (&[][..], None),
        };
        let data_sets = data_fields
            .chunks(2)
            .map(|set| {
                let mut r = FieldReader::new(set);
                (r.string(), r.string())
            })
            .collect();

        Self {
            total_sentences,
            sentence_number,
            vessel_mmsi,
            data_sets,
            expansion_indicator,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    const FIXTURE: &str = "$--DSR,1,1,3601234560,00,12345678,E*0A";

    fn checksum(body: &str) -> u8 {
        body.bytes().fold(0, |value, byte| value ^ byte)
    }

    #[test]
    fn dsr_empty() {
        assert_eq!(
            Dsr::parse(&["", "", "", ""]),
            Dsr {
                total_sentences: None,
                sentence_number: None,
                vessel_mmsi: None,
                data_sets: vec![],
                expansion_indicator: None,
            }
        );
    }

    #[test]
    fn dsr_encode_roundtrip() {
        let original = Dsr {
            total_sentences: Some(1),
            sentence_number: Some(1),
            vessel_mmsi: Some("3601234560".to_string()),
            data_sets: vec![(Some("00".to_string()), Some("12345678".to_string()))],
            expansion_indicator: Some('E'),
        };
        let sentence = original.to_sentence("CD").expect("encode DSR");
        let frame = parse_frame(sentence.trim()).expect("re-parse DSR");
        assert_eq!(original, Dsr::parse(&frame.fields));
    }

    #[test]
    fn dsr_expansion_indicator_requires_one_character() {
        assert_eq!(Dsr::parse(&["", "", "", "EX"]).expansion_indicator, None);
    }

    #[test]
    fn dsr_values_sim183() {
        // Published in the Ninglu SIM183 NMEA0183 Tester Manual, page 12.
        // https://www.manualslib.com/manual/2441097/Ninglu-Nmea0183.html
        let (body, reported_checksum) = FIXTURE[1..].split_once('*').expect("checksum separator");
        assert_eq!(
            checksum(body),
            u8::from_str_radix(reported_checksum, 16).expect("hex checksum")
        );

        let frame = parse_frame(FIXTURE).expect("valid SIM183 DSR frame");
        let dsr = Dsr::parse(&frame.fields);
        assert_eq!(dsr.total_sentences, Some(1));
        assert_eq!(dsr.sentence_number, Some(1));
        assert_eq!(dsr.vessel_mmsi.as_deref(), Some("3601234560"));
        assert_eq!(
            dsr.data_sets,
            [(Some("00".into()), Some("12345678".into()))]
        );
        assert_eq!(dsr.expansion_indicator, Some('E'));
    }
}
