use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PRC — Propulsion Remote Control Status.
///
/// Wire: `lever demand,lever status,RPM demand,RPM mode,pitch demand,pitch mode,
/// operating location,engine or shaft`
#[derive(Debug, Clone, PartialEq)]
pub struct Prc {
    /// Lever demand position, from full astern to full ahead (percent).
    pub lever_demand: Option<f32>,
    /// Lever demand validity: 'A' = valid, 'V' = invalid.
    pub lever_status: Option<char>,
    /// RPM demand in percent or revolutions per minute, according to `rpm_mode`.
    pub rpm_demand: Option<f32>,
    /// RPM demand mode: 'P' = percent, 'R' = revolutions per minute, 'V' = invalid.
    pub rpm_mode: Option<char>,
    /// Propeller pitch demand in percent, according to `pitch_mode`.
    pub pitch_demand: Option<f32>,
    /// Pitch demand mode: 'P' = percent, 'V' = invalid.
    pub pitch_mode: Option<char>,
    /// Control location: bridge, wing, engine room, or local.
    pub operating_location: Option<char>,
    /// Engine or propeller shaft identifier, retained as text because sources disagree on its width.
    pub engine_or_shaft: Option<String>,
}

impl Prc {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            lever_demand: r.f32(),
            lever_status: r.char(),
            rpm_demand: r.f32(),
            rpm_mode: r.char(),
            pitch_demand: r.f32(),
            pitch_mode: r.char(),
            operating_location: r.char(),
            engine_or_shaft: r.string(),
        })
    }
}

impl NmeaEncodable for Prc {
    const SENTENCE_TYPE: &str = "PRC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.lever_demand);
        w.char(self.lever_status);
        w.f32(self.rpm_demand);
        w.char(self.rpm_mode);
        w.f32(self.pitch_demand);
        w.char(self.pitch_mode);
        w.char(self.operating_location);
        w.string(self.engine_or_shaft.as_deref());
        w.finish()
    }
}
