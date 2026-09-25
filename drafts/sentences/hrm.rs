use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HRM — Heel angle, roll period and roll amplitude measurement device.
///
/// Wire: `heel_angle,roll_period,roll_amplitude_port,roll_amplitude_starboard,status,roll_peak_hold_port,roll_peak_hold_starboard,peak_hold_reset_time,peak_hold_reset_day,peak_hold_reset_month,peak_hold_reset_year,heel_angle_alert_threshold,sentence_status`
#[derive(Debug, Clone, PartialEq)]
pub struct Hrm {
    /// Actual heel angle in degrees. Positive is starboard, negative is port.
    pub heel_angle: Option<f32>,
    /// Time between successive maximum heel angles, in seconds.
    pub roll_period: Option<f32>,
    /// Maximum heel angle to port during the latest motion, in degrees.
    pub roll_amplitude_port: Option<f32>,
    /// Maximum heel angle to starboard during the latest motion, in degrees.
    pub roll_amplitude_starboard: Option<f32>,
    /// Data status: `A` valid or `V` invalid.
    pub status: Option<char>,
    /// Maximum port heel angle since the peak hold was reset, in degrees.
    pub roll_peak_hold_port: Option<f32>,
    /// Maximum starboard heel angle since the peak hold was reset, in degrees.
    pub roll_peak_hold_starboard: Option<f32>,
    /// UTC time at which the peak hold values were reset (`hhmmss`).
    pub peak_hold_reset_time: Option<String>,
    /// UTC day on which the peak hold values were reset.
    pub peak_hold_reset_day: Option<u8>,
    /// UTC month on which the peak hold values were reset.
    pub peak_hold_reset_month: Option<u8>,
    /// UTC year in which the peak hold values were reset.
    pub peak_hold_reset_year: Option<u16>,
    /// Heel angle threshold configured for alerts, in degrees.
    pub heel_angle_alert_threshold: Option<f32>,
    /// Sentence status: `R` report or `C` configuration command.
    pub sentence_status: Option<char>,
}

impl Hrm {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            heel_angle: r.f32(),
            roll_period: r.f32(),
            roll_amplitude_port: r.f32(),
            roll_amplitude_starboard: r.f32(),
            status: r.char(),
            roll_peak_hold_port: r.f32(),
            roll_peak_hold_starboard: r.f32(),
            peak_hold_reset_time: r.string(),
            peak_hold_reset_day: r.u8(),
            peak_hold_reset_month: r.u8(),
            peak_hold_reset_year: r.u16(),
            heel_angle_alert_threshold: r.f32(),
            sentence_status: r.char(),
        })
    }
}

impl NmeaEncodable for Hrm {
    const SENTENCE_TYPE: &str = "HRM";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.heel_angle);
        w.f32(self.roll_period);
        w.f32(self.roll_amplitude_port);
        w.f32(self.roll_amplitude_starboard);
        w.char(self.status);
        w.f32(self.roll_peak_hold_port);
        w.f32(self.roll_peak_hold_starboard);
        w.string(self.peak_hold_reset_time.as_deref());
        w.u8(self.peak_hold_reset_day);
        w.u8(self.peak_hold_reset_month);
        w.u16(self.peak_hold_reset_year);
        w.f32(self.heel_angle_alert_threshold);
        w.char(self.sentence_status);
        w.finish()
    }
}
