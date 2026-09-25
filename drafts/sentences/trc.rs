use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// TRC — Thruster Control Data.
///
/// Wire: `thruster_number,rpm_demand,rpm_mode,pitch_demand,pitch_mode,azimuth_demand,operating_location,sentence_status`
#[derive(Debug, Clone, PartialEq)]
pub struct Trc {
    /// Thruster identifier, retained as text to preserve significant width.
    pub thruster_number: Option<String>,
    /// Requested RPM or percent, according to `rpm_mode`.
    pub rpm_demand: Option<f32>,
    /// RPM demand mode: `R` (RPM), `P` (percent), or `V` (invalid).
    pub rpm_mode: Option<char>,
    /// Requested pitch in percent.
    pub pitch_demand: Option<f32>,
    /// Pitch demand mode: `P` (percent) or `V` (invalid).
    pub pitch_mode: Option<char>,
    /// Requested thrust direction, in degrees.
    pub azimuth_demand: Option<f32>,
    /// Command source location, normally `B` for bridge.
    pub operating_location: Option<char>,
    /// `C` when the thruster is in use, `R` when it is not used.
    pub sentence_status: Option<char>,
}

impl Trc {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            thruster_number: r.string(),
            rpm_demand: r.f32(),
            rpm_mode: r.char(),
            pitch_demand: r.f32(),
            pitch_mode: r.char(),
            azimuth_demand: r.f32(),
            operating_location: r.char(),
            sentence_status: r.char(),
        })
    }
}

impl NmeaEncodable for Trc {
    const SENTENCE_TYPE: &str = "TRC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.thruster_number.as_deref());
        w.f32(self.rpm_demand);
        w.char(self.rpm_mode);
        w.f32(self.pitch_demand);
        w.char(self.pitch_mode);
        w.f32(self.azimuth_demand);
        w.char(self.operating_location);
        w.char(self.sentence_status);
        w.finish()
    }
}
