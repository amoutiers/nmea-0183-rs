use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// DSI — DSC transponder initialize.
///
/// Wire: `total,sentence,mmsi,course,vessel_type,geographic_area,(command,data)...,expansion`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dsi {
    pub total_sentences: Option<u8>,
    pub sentence_number: Option<u8>,
    /// Preserved as text because the wire format uses a fixed ten-character MMSI field.
    pub vessel_mmsi: Option<String>,
    /// Preserved as text to retain the specified three-character wire width.
    pub vessel_course: Option<String>,
    /// Preserved as text to retain the specified two-character wire width.
    pub vessel_type: Option<String>,
    /// Geographic area in 0.01-minute encoding, preserved in its wire form.
    pub geographic_area: Option<String>,
    /// Repeated command-code/additional-data pairs.
    pub command_sets: Vec<(Option<String>, Option<String>)>,
    pub expansion_indicator: Option<char>,
}

impl Dsi {
    /// Parse fields from a decoded NMEA frame; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let total_sentences = r.u8();
        let sentence_number = r.u8();
        let vessel_mmsi = r.string();
        let vessel_course = r.string();
        let vessel_type = r.string();
        let geographic_area = r.string();

        let (command_fields, expansion_indicator) = match fields.get(6..) {
            Some(trailing) => match trailing.split_last() {
                Some((indicator, command_fields)) => (
                    command_fields,
                    FieldReader::new(&[*indicator]).char(),
                ),
                None => (&[][..], None),
            },
            None => (&[][..], None),
        };
        let command_sets = command_fields
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
            vessel_course,
            vessel_type,
            geographic_area,
            command_sets,
            expansion_indicator,
        }
    }
}

impl NmeaEncodable for Dsi {
    const SENTENCE_TYPE: &str = "DSI";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.total_sentences);
        w.u8(self.sentence_number);
        w.string(self.vessel_mmsi.as_deref());
        w.string(self.vessel_course.as_deref());
        w.string(self.vessel_type.as_deref());
        w.string(self.geographic_area.as_deref());
        for (command, data) in &self.command_sets {
            w.string(command.as_deref());
            w.string(data.as_deref());
        }
        w.char(self.expansion_indicator);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    const FIXTURE: &str = "$--DSI,1,1,3601234560,,,,00,12345678,E*3D";

    fn checksum(body: &str) -> u8 {
        body.bytes().fold(0, |value, byte| value ^ byte)
    }

    #[test]
    fn dsi_empty() {
        assert_eq!(
            Dsi::parse(&["", "", "", "", "", "", ""]),
            Dsi {
                total_sentences: None,
                sentence_number: None,
                vessel_mmsi: None,
                vessel_course: None,
                vessel_type: None,
                geographic_area: None,
                command_sets: vec![],
                expansion_indicator: None,
            }
        );
    }

    #[test]
    fn dsi_encode_roundtrip() {
        let original = Dsi {
            total_sentences: Some(1),
            sentence_number: Some(1),
            vessel_mmsi: Some("3601234560".to_string()),
            vessel_course: None,
            vessel_type: None,
            geographic_area: None,
            command_sets: vec![(Some("00".to_string()), Some("12345678".to_string()))],
            expansion_indicator: Some('E'),
        };
        let sentence = original.to_sentence("GP").expect("encode DSI");
        let frame = parse_frame(sentence.trim()).expect("re-parse DSI");
        assert_eq!(original, Dsi::parse(&frame.fields));
    }

    #[test]
    fn dsi_expansion_indicator_requires_one_character() {
        assert_eq!(Dsi::parse(&["", "", "", "", "", "", "EX"]).expansion_indicator, None);
    }

    #[test]
    fn dsi_values_sim183() {
        // Published in the Ninglu SIM183 NMEA0183 Tester Manual, page 12.
        // https://www.manualslib.com/manual/2441097/Ninglu-Nmea0183.html
        let (body, reported_checksum) = FIXTURE[1..].split_once('*').expect("checksum separator");
        assert_eq!(
            checksum(body),
            u8::from_str_radix(reported_checksum, 16).expect("hex checksum")
        );

        let frame = parse_frame(FIXTURE).expect("valid SIM183 DSI frame");
        let dsi = Dsi::parse(&frame.fields);
        assert_eq!(dsi.total_sentences, Some(1));
        assert_eq!(dsi.sentence_number, Some(1));
        assert_eq!(dsi.vessel_mmsi.as_deref(), Some("3601234560"));
        assert_eq!(dsi.vessel_course, None);
        assert_eq!(dsi.vessel_type, None);
        assert_eq!(dsi.geographic_area, None);
        assert_eq!(
            dsi.command_sets,
            [(Some("00".into()), Some("12345678".into()))]
        );
        assert_eq!(dsi.expansion_indicator, Some('E'));
    }
}
