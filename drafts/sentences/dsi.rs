use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// DSI — DSC transponder initialize.
///
/// Wire: `total,sentence,mmsi,course,vessel_type,geographic_area,(command,data)...,expansion`
#[derive(Debug, Clone, PartialEq)]
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
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
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
                    indicator.chars().next().filter(|_| !indicator.is_empty()),
                ),
                None => (&[][..], None),
            },
            None => (&[][..], None),
        };
        let command_sets = command_fields
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
            vessel_course,
            vessel_type,
            geographic_area,
            command_sets,
            expansion_indicator,
        })
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
