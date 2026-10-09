use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// TRD: Thruster response data.
///
/// Wire: `thruster_number,rpm_response,rpm_mode,pitch_response,pitch_mode,azimuth_response`
#[derive(Debug, Clone, PartialEq, Default)]
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
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            thruster_number: r.string(),
            rpm_response: r.f32(),
            rpm_mode: r.char(),
            pitch_response: r.f32(),
            pitch_mode: r.char(),
            azimuth_response: r.f32(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    fn assert_close(actual: Option<f32>, expected: f32) {
        assert!((actual.expect("value present") - expected).abs() < 0.001);
    }

    #[test]
    fn trd_empty() {
        let frame = parse_frame("$RCTRD,,,,,,*53").expect("valid empty TRD frame");
        assert_eq!(
            Trd::parse(&frame.fields),
            Trd {
                thruster_number: None,
                rpm_response: None,
                rpm_mode: None,
                pitch_response: None,
                pitch_mode: None,
                azimuth_response: None,
            }
        );
    }

    #[test]
    fn trd_encode_roundtrip() {
        let original = Trd {
            thruster_number: Some("2".into()),
            rpm_response: Some(400.5),
            rpm_mode: Some('R'),
            pitch_response: None,
            pitch_mode: Some('V'),
            azimuth_response: Some(90.8),
        };
        let sentence = original.to_sentence("RC").expect("encode TRD");
        let frame = parse_frame(sentence.trim()).expect("re-parse TRD");
        assert_eq!(Trd::parse(&frame.fields), original);
    }

    #[test]
    fn trd_values_navis() {
        let frame = parse_frame("$RCTRD,2,400.5,R,,V,90.8*55")
            .expect("valid Navis DP4000 TRD frame");
        let trd = Trd::parse(&frame.fields);
        assert_eq!(trd.thruster_number.as_deref(), Some("2"));
        assert_close(trd.rpm_response, 400.5);
        assert_eq!(trd.rpm_mode, Some('R'));
        assert!(trd.pitch_response.is_none());
        assert_eq!(trd.pitch_mode, Some('V'));
        assert_close(trd.azimuth_response, 90.8);
    }
}
