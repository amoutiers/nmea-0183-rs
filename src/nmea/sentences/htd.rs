use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HTD: Heading/Track Control Data.
///
/// Wire: `override,rudder_angle,rudder_direction,steering_mode,turn_mode,rudder_limit,off_heading_limit,radius_of_turn,rate_of_turn,heading_to_steer,off_track_limit,track,heading_reference,rudder_status,off_heading_status,off_track_status,vessel_heading`
#[derive(Debug, Clone, PartialEq, Default)]
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
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
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
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn htd_empty() {
        assert_eq!(Htd::parse(&[]), Htd::default());
    }

    #[test]
    fn htd_encode_roundtrip() {
        let original = Htd {
            override_status: Some('V'),
            commanded_rudder_angle: Some(0.0),
            commanded_rudder_direction: Some('L'),
            selected_steering_mode: Some('T'),
            turn_mode: Some('R'),
            commanded_rudder_limit: Some(40.0),
            commanded_off_heading_limit: Some(20.0),
            commanded_radius_of_turn: Some(2.5),
            commanded_rate_of_turn: Some(10.0),
            commanded_heading_to_steer: Some(123.0),
            commanded_off_track_limit: Some(0.4),
            commanded_track: Some(135.0),
            heading_reference: Some('M'),
            rudder_status: Some('A'),
            off_heading_status: Some('V'),
            off_track_status: Some('A'),
            vessel_heading: Some(130.0),
        };
        let encoded = original.to_sentence("HE").expect("encode HTD");
        let (body, _) = encoded.split_once('*').expect("checksum");
        assert_eq!(body, "$HEHTD,V,0,L,T,R,40,20,2.5,10,123,0.4,135,M,A,V,A,130");
        let frame = parse_frame(&encoded).expect("decode HTD");
        assert_eq!(Htd::parse(&frame.fields), original);
    }

    #[test]
    fn htd_values_deif_xdi() {
        // Fixture published in the DEIF XDi Series Designer's Handbook, page 169:
        // https://www.manualslib.com/manual/2903353/Deif-Xdi-Series.html?page=169
        const FIXTURE: &str =
            "$HEHTD,A,0.0,L,M,T,45.0,25.0,0.5,3275.0,200.0,1.0,250.0,T,A,A,A,270.0*67";

        let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
        assert_eq!(
            body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
            u8::from_str_radix(checksum, 16).expect("hex checksum")
        );

        let frame = parse_frame(FIXTURE).expect("valid DEIF XDi HTD frame");
        let htd = Htd::parse(&frame.fields);

        assert_eq!(htd.override_status, Some('A'));
        assert!((htd.commanded_rudder_angle.expect("value present") - 0.0).abs() < 1e-5);
        assert_eq!(htd.commanded_rudder_direction, Some('L'));
        assert_eq!(htd.selected_steering_mode, Some('M'));
        assert_eq!(htd.turn_mode, Some('T'));
        assert!((htd.commanded_rudder_limit.expect("value present") - 45.0).abs() < 1e-5);
        assert!((htd.commanded_off_heading_limit.expect("value present") - 25.0).abs() < 1e-5);
        assert!((htd.commanded_radius_of_turn.expect("value present") - 0.5).abs() < 1e-5);
        assert!((htd.commanded_rate_of_turn.expect("value present") - 3275.0).abs() < 1e-5);
        assert!((htd.commanded_heading_to_steer.expect("value present") - 200.0).abs() < 1e-5);
        assert!((htd.commanded_off_track_limit.expect("value present") - 1.0).abs() < 1e-5);
        assert!((htd.commanded_track.expect("value present") - 250.0).abs() < 1e-5);
        assert_eq!(htd.heading_reference, Some('T'));
        assert_eq!(htd.rudder_status, Some('A'));
        assert_eq!(htd.off_heading_status, Some('A'));
        assert_eq!(htd.off_track_status, Some('A'));
        assert!((htd.vessel_heading.expect("value present") - 270.0).abs() < 1e-5);
    }
}
