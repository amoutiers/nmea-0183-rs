use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// TRD — Thruster Response Data.
///
/// Wire: `thruster_number,rpm_response,rpm_mode,pitch_response,pitch_mode,azimuth_response`
#[derive(Debug, Clone, PartialEq)]
pub struct Trd {
    /// Thruster identifier, retained as text to preserve significant width.
    pub thruster_number: Option<String>,
    /// Actual RPM or percent, according to `rpm_mode`.
    pub rpm_response: Option<f32>,
    /// RPM response mode: `R` (RPM), `P` (percent), or `V` (invalid).
    pub rpm_mode: Option<char>,
    /// Actual pitch in percent.
    pub pitch_response: Option<f32>,
    /// Pitch response mode: `P` (percent) or `V` (invalid).
    pub pitch_mode: Option<char>,
    /// Actual thrust direction, in degrees.
    pub azimuth_response: Option<f32>,
}

impl Trd {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            thruster_number: r.string(),
            rpm_response: r.f32(),
            rpm_mode: r.char(),
            pitch_response: r.f32(),
            pitch_mode: r.char(),
            azimuth_response: r.f32(),
        })
    }
}

impl NmeaEncodable for Trd {
    const SENTENCE_TYPE: &str = "TRD";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.thruster_number.as_deref());
        w.f32(self.rpm_response);
        w.char(self.rpm_mode);
        w.f32(self.pitch_response);
        w.char(self.pitch_mode);
        w.f32(self.azimuth_response);
        w.finish()
    }
}
