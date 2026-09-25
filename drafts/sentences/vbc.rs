use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VBC — Water-Referenced and Ground-Referenced Docking Speed Data.
///
/// Wire: `longitudinal water,bow transverse water,CCRP transverse water,
/// stern transverse water,water status,longitudinal ground,bow transverse ground,
/// CCRP transverse ground,stern transverse ground,ground status`
#[derive(Debug, Clone, PartialEq)]
pub struct Vbc {
    /// Longitudinal water speed at the consistent common reference point (CCRP), in knots.
    pub longitudinal_water_speed: Option<f32>,
    /// Bow transverse water speed in knots; negative is port.
    pub bow_transverse_water_speed: Option<f32>,
    /// CCRP transverse water speed in knots; negative is port.
    pub ccrp_transverse_water_speed: Option<f32>,
    /// Stern transverse water speed in knots; negative is port.
    pub stern_transverse_water_speed: Option<f32>,
    /// Water speed status: 'A' = valid, 'V' = invalid.
    pub water_speed_status: Option<char>,
    /// Longitudinal ground speed at the CCRP, in knots.
    pub longitudinal_ground_speed: Option<f32>,
    /// Bow transverse ground speed in knots; negative is port.
    pub bow_transverse_ground_speed: Option<f32>,
    /// CCRP transverse ground speed in knots; negative is port.
    pub ccrp_transverse_ground_speed: Option<f32>,
    /// Stern transverse ground speed in knots; negative is port.
    pub stern_transverse_ground_speed: Option<f32>,
    /// Ground speed status: 'A' = valid, 'V' = invalid.
    pub ground_speed_status: Option<char>,
}

impl Vbc {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            longitudinal_water_speed: r.f32(),
            bow_transverse_water_speed: r.f32(),
            ccrp_transverse_water_speed: r.f32(),
            stern_transverse_water_speed: r.f32(),
            water_speed_status: r.char(),
            longitudinal_ground_speed: r.f32(),
            bow_transverse_ground_speed: r.f32(),
            ccrp_transverse_ground_speed: r.f32(),
            stern_transverse_ground_speed: r.f32(),
            ground_speed_status: r.char(),
        })
    }
}

impl NmeaEncodable for Vbc {
    const SENTENCE_TYPE: &str = "VBC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.longitudinal_water_speed);
        w.f32(self.bow_transverse_water_speed);
        w.f32(self.ccrp_transverse_water_speed);
        w.f32(self.stern_transverse_water_speed);
        w.char(self.water_speed_status);
        w.f32(self.longitudinal_ground_speed);
        w.f32(self.bow_transverse_ground_speed);
        w.f32(self.ccrp_transverse_ground_speed);
        w.f32(self.stern_transverse_ground_speed);
        w.char(self.ground_speed_status);
        w.finish()
    }
}
