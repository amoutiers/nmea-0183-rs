use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PRC: Propulsion Remote Control Status.
///
/// Wire: `lever demand,lever status,RPM demand,RPM mode,pitch demand,pitch mode,
/// operating location,engine or shaft`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Prc {
    /// Lever demand position, from full astern to full ahead (percent).
    pub lever_demand: Option<f32>,
    /// Lever demand validity: 'A' = valid, 'V' = invalid.
    pub lever_status: Option<char>,
    /// RPM demand in percent or revolutions per minute, according to `rpm_mode`.
    pub rpm_demand: Option<f32>,
    /// RPM demand mode: 'P' = percent, 'R' = revolutions per minute, 'V' = invalid.
    pub rpm_mode: Option<char>,
    /// Propeller pitch demand in percent, according to `pitch_mode`.
    pub pitch_demand: Option<f32>,
    /// Pitch demand mode: 'P' = percent, 'D' = degrees, 'V' = invalid.
    pub pitch_mode: Option<char>,
    /// Control location: bridge, wing, engine room, or local.
    pub operating_location: Option<char>,
    /// Engine or propeller shaft identifier, retained as text for compatible parsing.
    pub engine_or_shaft: Option<String>,
}

impl Prc {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            lever_demand: r.f32(),
            lever_status: r.char(),
            rpm_demand: r.f32(),
            rpm_mode: r.char(),
            pitch_demand: r.f32(),
            pitch_mode: r.char(),
            operating_location: r.char(),
            engine_or_shaft: r.string(),
        }
    }
}

impl NmeaEncodable for Prc {
    const SENTENCE_TYPE: &str = "PRC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.lever_demand);
        w.char(self.lever_status);
        w.f32(self.rpm_demand);
        w.char(self.rpm_mode);
        w.f32(self.pitch_demand);
        w.char(self.pitch_mode);
        w.char(self.operating_location);
        w.string(self.engine_or_shaft.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn prc_empty() {
        assert_eq!(Prc::parse(&[]), Prc::default());
    }

    #[test]
    fn prc_encode_roundtrip() {
        let original = Prc {
            lever_demand: Some(-30.0),
            lever_status: Some('A'),
            rpm_demand: Some(-120.0),
            rpm_mode: Some('R'),
            pitch_demand: Some(-4.5),
            pitch_mode: Some('D'),
            operating_location: Some('B'),
            engine_or_shaft: Some("1".into()),
        };
        let encoded = original.to_sentence("RC").expect("encode PRC");
        let (body, _) = encoded.split_once('*').expect("checksum");
        assert_eq!(body, "$RCPRC,-30,A,-120,R,-4.5,D,B,1");
        let frame = parse_frame(&encoded).expect("decode PRC");
        assert_eq!(Prc::parse(&frame.fields), original);
    }

    #[test]
    fn prc_values_jrc_vdr_datanalyzer() {
        // Fixture published in the JRC VDR DATAnalyzer User Manual, Log screen:
        // https://jrcbrasil.com/vda/manuals/vdr_datanalyzer_manual.pdf
        const FIXTURE: &str = "$RCPRC,,V,0.0,R,,V,,0*1C";

        let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
        assert_eq!(
            body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
            u8::from_str_radix(checksum, 16).expect("hex checksum")
        );

        let frame = parse_frame(FIXTURE).expect("valid JRC VDR DATAnalyzer PRC frame");
        let prc = Prc::parse(&frame.fields);

        assert!(prc.lever_demand.is_none());
        assert_eq!(prc.lever_status, Some('V'));
        assert!((prc.rpm_demand.expect("value present") - 0.0).abs() < 1e-5);
        assert_eq!(prc.rpm_mode, Some('R'));
        assert!(prc.pitch_demand.is_none());
        assert_eq!(prc.pitch_mode, Some('V'));
        assert!(prc.operating_location.is_none());
        assert_eq!(prc.engine_or_shaft.as_deref(), Some("0"));
    }
}
