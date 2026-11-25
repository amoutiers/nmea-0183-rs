use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HTC: Heading/Track Control Command.
///
/// Wire: `override,rudder_angle,rudder_direction,steering_mode,turn_mode,rudder_limit,off_heading_limit,radius_of_turn,rate_of_turn,heading_to_steer,off_track_limit,track,heading_reference[,sentence_status]`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Htc {
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
    /// Sentence status: `C` for a command, `R` for a status report.
    /// Legacy HTC frames may omit this field.
    pub sentence_status: Option<char>,
}

impl Htc {
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
            sentence_status: r.char(),
        }
    }
}

impl NmeaEncodable for Htc {
    const SENTENCE_TYPE: &str = "HTC";

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
        if self.sentence_status.is_some() {
            w.char(self.sentence_status);
        }
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn htc_empty() {
        assert_eq!(Htc::parse(&[]), Htc::default());
    }

    #[test]
    fn htc_encode_roundtrip() {
        let original = Htc {
            override_status: Some('V'),
            commanded_rudder_angle: Some(12.3),
            commanded_rudder_direction: Some('R'),
            selected_steering_mode: Some('H'),
            turn_mode: Some('T'),
            commanded_rudder_limit: Some(45.0),
            commanded_off_heading_limit: Some(25.0),
            commanded_radius_of_turn: Some(0.5),
            commanded_rate_of_turn: Some(15.0),
            commanded_heading_to_steer: Some(200.0),
            commanded_off_track_limit: Some(1.0),
            commanded_track: Some(250.0),
            heading_reference: Some('T'),
            sentence_status: Some('C'),
        };
        let encoded = original.to_sentence("HE").expect("encode HTC");
        let (body, _) = encoded.split_once('*').expect("checksum");
        assert_eq!(body, "$HEHTC,V,12.3,R,H,T,45,25,0.5,15,200,1,250,T,C");
        let frame = parse_frame(&encoded).expect("decode HTC");
        assert_eq!(Htc::parse(&frame.fields), original);
    }

    #[test]
    fn htc_values_sim183() {
        // Fixture published in the Ninglu SIM183 NMEA0183 Tester Manual, page 12:
        // https://www.manualslib.com/manual/2441097/Ninglu-Nmea0183.html
        const FIXTURE: &str = "$--HTC,A,123.4,R,M,N,,,,,,,,T*1D";

        let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
        assert_eq!(
            body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
            u8::from_str_radix(checksum, 16).expect("hex checksum")
        );

        let frame = parse_frame(FIXTURE).expect("valid SIM183 HTC frame");
        let htc = Htc::parse(&frame.fields);

        assert_eq!(htc.override_status, Some('A'));
        assert!((htc.commanded_rudder_angle.expect("value present") - 123.4).abs() < 1e-5);
        assert_eq!(htc.commanded_rudder_direction, Some('R'));
        assert_eq!(htc.selected_steering_mode, Some('M'));
        assert_eq!(htc.turn_mode, Some('N'));
        assert_eq!(htc.commanded_rudder_limit, None);
        assert_eq!(htc.commanded_off_heading_limit, None);
        assert_eq!(htc.commanded_radius_of_turn, None);
        assert_eq!(htc.commanded_rate_of_turn, None);
        assert_eq!(htc.commanded_heading_to_steer, None);
        assert_eq!(htc.commanded_off_track_limit, None);
        assert_eq!(htc.commanded_track, None);
        assert_eq!(htc.heading_reference, Some('T'));
        assert_eq!(htc.sentence_status, None);
    }
}
