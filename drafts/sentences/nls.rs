use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// Status reported for one navigation light.
#[derive(Debug, Clone, PartialEq)]
pub struct NavigationLightStatus {
    /// Numeric identifier assigned by the navigation lights controller.
    pub light_id: Option<u16>,
    /// Current light status.
    pub status: Option<u8>,
    /// Estimated remaining working time in units of 100 hours.
    pub remaining_working_hours: Option<u8>,
}

/// NLS — Navigation light status.
///
/// Wire: `total,sentence,sequence,count,(light,status,remaining_hours)...`
#[derive(Debug, Clone, PartialEq)]
pub struct Nls {
    pub total_sentences: Option<u8>,
    pub sentence_number: Option<u8>,
    pub sequential_message_id: Option<u8>,
    /// Number of light/status groups carried by this sentence.
    pub light_count: Option<u8>,
    /// Up to eight navigation-light status groups.
    pub lights: Vec<NavigationLightStatus>,
}

impl Nls {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let total_sentences = r.u8();
        let sentence_number = r.u8();
        let sequential_message_id = r.u8();
        let light_count = r.u8();
        let lights = (0..light_count.unwrap_or(0).min(8))
            .map(|_| NavigationLightStatus {
                light_id: r.u16(),
                status: r.u8(),
                remaining_working_hours: r.u8(),
            })
            .collect();

        Some(Self {
            total_sentences,
            sentence_number,
            sequential_message_id,
            light_count,
            lights,
        })
    }
}

impl NmeaEncodable for Nls {
    const SENTENCE_TYPE: &str = "NLS";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.total_sentences);
        w.u8(self.sentence_number);
        w.u8(self.sequential_message_id);
        w.u8(self.light_count);
        for light in self.lights.iter().take(8) {
            w.u16(light.light_id);
            w.u8(light.status);
            w.u8(light.remaining_working_hours);
        }
        w.finish()
    }
}
