use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// TRC: Thruster Control Data.
///
/// Wire: `thruster_number,rpm_demand,rpm_mode,pitch_demand,pitch_mode,azimuth_demand,operating_location,sentence_status`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Trc {
    /// Thruster identifier, retained as text to preserve significant width.
    pub thruster_number: Option<String>,
    /// Requested RPM or percent, according to `rpm_mode`.
    pub rpm_demand: Option<f32>,
    /// RPM demand mode: `R` (RPM), `P` (percent), or `V` (invalid).
    pub rpm_mode: Option<char>,
    /// Requested pitch in percent or degrees, according to the pitch mode.
    pub pitch_demand: Option<f32>,
    /// Pitch demand mode: `P` (percent), `D` (degrees), or `V` (invalid).
    pub pitch_mode: Option<char>,
    /// Requested thrust direction, in degrees.
    pub azimuth_demand: Option<f32>,
    /// Command source location, such as `B` for bridge.
    pub operating_location: Option<char>,
    /// Sentence status: `C` for a command, `R` for a status report.
    pub sentence_status: Option<char>,
}

impl Trc {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            thruster_number: r.string(),
            rpm_demand: r.f32(),
            rpm_mode: r.char(),
            pitch_demand: r.f32(),
            pitch_mode: r.char(),
            azimuth_demand: r.f32(),
            operating_location: r.char(),
            sentence_status: r.char(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn trc_empty() {
        assert_eq!(Trc::parse(&[]), Trc::default());
    }

    #[test]
    fn trc_encode_roundtrip() {
        let original = Trc {
            thruster_number: Some("2".into()),
            rpm_demand: Some(75.0),
            rpm_mode: Some('P'),
            pitch_demand: Some(-2.5),
            pitch_mode: Some('D'),
            azimuth_demand: Some(90.0),
            operating_location: Some('B'),
            sentence_status: Some('R'),
        };
        let encoded = original.to_sentence("RC").expect("encode TRC");
        let (body, _) = encoded.split_once('*').expect("checksum");
        assert_eq!(body, "$RCTRC,2,75,P,-2.5,D,90,B,R");
        let frame = parse_frame(&encoded).expect("decode TRC");
        assert_eq!(Trc::parse(&frame.fields), original);
    }

    #[test]
    fn trc_values_navis_dp4000() {
        // Fixture published in the NAVIS NavDP 4000 Operation Manual, TRC Message:
        // https://alphatronmarine.com/files/secured/docuware_documents/144-DP%2BNavis%2BDP4000%2BOperation%2BManual%2B%2B1-5-2018.pdf
        const FIXTURE: &str = "$RCTRC,2,402.6,R,,V,88.3,B,C*50";

        let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
        assert_eq!(
            body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
            u8::from_str_radix(checksum, 16).expect("hex checksum")
        );

        let frame = parse_frame(FIXTURE).expect("valid Navis DP4000 TRC frame");
        let trc = Trc::parse(&frame.fields);

        assert_eq!(trc.thruster_number.as_deref(), Some("2"));
        assert!((trc.rpm_demand.expect("value present") - 402.6).abs() < 1e-5);
        assert_eq!(trc.rpm_mode, Some('R'));
        assert!(trc.pitch_demand.is_none());
        assert_eq!(trc.pitch_mode, Some('V'));
        assert!((trc.azimuth_demand.expect("value present") - 88.3).abs() < 1e-5);
        assert_eq!(trc.operating_location, Some('B'));
        assert_eq!(trc.sentence_status, Some('C'));
    }
}
