use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GFA — GNSS fix accuracy and integrity.
///
/// Wire: `time,hpl,vpl,std_x,std_y,theta,std_h,sal,integrity_status`
#[derive(Debug, Clone, PartialEq)]
pub struct Gfa {
    /// UTC time of the associated GGA or GNS fix (HHMMSS.SS).
    pub time: Option<String>,
    /// Horizontal protection level in meters.
    pub horizontal_protection_level: Option<f32>,
    /// Vertical protection level in meters.
    pub vertical_protection_level: Option<f32>,
    /// Standard deviation of the semi-major axis of the error ellipse in meters.
    pub semi_major_std_dev: Option<f32>,
    /// Standard deviation of the semi-minor axis of the error ellipse in meters.
    pub semi_minor_std_dev: Option<f32>,
    /// Semi-major-axis orientation in degrees from true north.
    pub semi_major_orientation: Option<f32>,
    /// Standard deviation of altitude in meters.
    pub altitude_std_dev: Option<f32>,
    /// Selected accuracy level in meters.
    pub selected_accuracy_level: Option<f32>,
    /// Variable-length integrity status, such as `SCC`.
    pub integrity_status: Option<String>,
}

impl Gfa {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            time: r.string(),
            horizontal_protection_level: r.f32(),
            vertical_protection_level: r.f32(),
            semi_major_std_dev: r.f32(),
            semi_minor_std_dev: r.f32(),
            semi_major_orientation: r.f32(),
            altitude_std_dev: r.f32(),
            selected_accuracy_level: r.f32(),
            integrity_status: r.string(),
        })
    }
}

impl NmeaEncodable for Gfa {
    const SENTENCE_TYPE: &str = "GFA";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.f32(self.horizontal_protection_level);
        w.f32(self.vertical_protection_level);
        w.f32(self.semi_major_std_dev);
        w.f32(self.semi_minor_std_dev);
        w.f32(self.semi_major_orientation);
        w.f32(self.altitude_std_dev);
        w.f32(self.selected_accuracy_level);
        w.string(self.integrity_status.as_deref());
        w.finish()
    }
}
