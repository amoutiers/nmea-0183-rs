use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HTD — Heading/Track Control Data.
///
/// Wire: `override,rudder_angle,rudder_direction,steering_mode,turn_mode,rudder_limit,off_heading_limit,radius_of_turn,rate_of_turn,heading_to_steer,off_track_limit,track,heading_reference,rudder_status,off_heading_status,off_track_status,vessel_heading`
#[derive(Debug, Clone, PartialEq)]
pub struct Htd {
    /// Override status: `A` when active, `V` when inactive.
    pub override_status: Option<char>,
    /// Commanded rudder angle, in degrees.
    pub commanded_rudder_angle: Option<f32>,
    /// Commanded rudder direction: `L` (port) or `R` (starboard).
    pub commanded_rudder_direction: Option<char>,
    /// Selected steering mode.
    pub selected_steering_mode: Option<char>,
    /// Turn mode: `R` (radius), `T` (rate), or `N` (uncontrolled).
    pub turn_mode: Option<char>,
    /// Commanded rudder limit, in degrees.
    pub commanded_rudder_limit: Option<f32>,
    /// Commanded off-heading limit, in degrees.
    pub commanded_off_heading_limit: Option<f32>,
    /// Commanded radius of turn for heading changes, in nautical miles.
    pub commanded_radius_of_turn: Option<f32>,
    /// Commanded rate of turn for heading changes, in degrees per minute.
    pub commanded_rate_of_turn: Option<f32>,
    /// Commanded heading to steer, in degrees.
    pub commanded_heading_to_steer: Option<f32>,
    /// Commanded off-track limit, in nautical miles.
    pub commanded_off_track_limit: Option<f32>,
    /// Commanded track, in degrees.
    pub commanded_track: Option<f32>,
    /// Heading reference in use: `T` (true) or `M` (magnetic).
    pub heading_reference: Option<char>,
    /// Rudder status: `A` within limits, `V` at or beyond its limit.
    pub rudder_status: Option<char>,
    /// Off-heading status: `A` within limits, `V` at or beyond its limit.
    pub off_heading_status: Option<char>,
    /// Off-track status: `A` within limits, `V` at or beyond its limit.
    pub off_track_status: Option<char>,
    /// Vessel heading, in degrees.
    pub vessel_heading: Option<f32>,
}

impl Htd {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            override_status: r.char(),
            commanded_rudder_angle: r.f32(),
            commanded_rudder_direction: r.char(),
            selected_steering_mode: r.char(),
            turn_mode: r.char(),
            commanded_rudder_limit: r.f32(),
            commanded_off_heading_limit: r.f32(),
            commanded_radius_of_turn: r.f32(),
            commanded_rate_of_turn: r.f32(),
            commanded_heading_to_steer: r.f32(),
            commanded_off_track_limit: r.f32(),
            commanded_track: r.f32(),
            heading_reference: r.char(),
            rudder_status: r.char(),
            off_heading_status: r.char(),
            off_track_status: r.char(),
            vessel_heading: r.f32(),
        })
    }
}

impl NmeaEncodable for Htd {
    const SENTENCE_TYPE: &str = "HTD";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.override_status);
        w.f32(self.commanded_rudder_angle);
        w.char(self.commanded_rudder_direction);
        w.char(self.selected_steering_mode);
        w.char(self.turn_mode);
        w.f32(self.commanded_rudder_limit);
        w.f32(self.commanded_off_heading_limit);
        w.f32(self.commanded_radius_of_turn);
        w.f32(self.commanded_rate_of_turn);
        w.f32(self.commanded_heading_to_steer);
        w.f32(self.commanded_off_track_limit);
        w.f32(self.commanded_track);
        w.char(self.heading_reference);
        w.char(self.rudder_status);
        w.char(self.off_heading_status);
        w.char(self.off_track_status);
        w.f32(self.vessel_heading);
        w.finish()
    }
}
