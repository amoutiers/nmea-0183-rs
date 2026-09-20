use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// MWV — Wind Speed and Angle.
///
/// Wire: `angle,reference(R/T),speed,speedUnit,status(A/V)`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mwv {
    /// Wind angle in degrees (0-360).
    pub wind_angle: Option<f32>,
    /// Reference type ('R' = relative, 'T' = theoretical/true per NMEA).
    pub reference: Option<char>,
    /// Wind speed (unit in `speed_units` field).
    pub wind_speed: Option<f32>,
    /// Speed unit indicator ('N' = knots, 'M' = m/s, 'K' = km/h).
    pub speed_units: Option<char>,
    /// Data validity ('A' = valid, 'V' = invalid).
    pub status: Option<char>,
}

impl Mwv {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            wind_angle: r.f32(),
            reference: r.char(),
            wind_speed: r.f32(),
            speed_units: r.char(),
            status: r.char(),
        })
    }
}

impl NmeaEncodable for Mwv {
    const SENTENCE_TYPE: &str = "MWV";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.wind_angle);
        w.char(self.reference);
        w.f32(self.wind_speed);
        w.char(self.speed_units);
        w.char(self.status);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn mwv_empty() {
        let frame = parse_frame("$IIMWV,,,,*4C").expect("valid");
        let mwv = Mwv::parse(&frame.fields).expect("parse MWV");
        assert!(mwv.wind_angle.is_none());
        assert!(mwv.reference.is_none());
    }

    #[test]
    fn mwv_relative_signalk() {
        let frame = parse_frame("$IIMWV,336,R,13.41,N,A*22").expect("valid");
        let mwv = Mwv::parse(&frame.fields).expect("parse MWV");
        assert!((mwv.wind_angle.expect("angle") - 336.0).abs() < 0.1);
        assert_eq!(mwv.reference, Some('R'));
        assert!((mwv.wind_speed.expect("speed") - 13.41).abs() < 0.01);
        assert_eq!(mwv.speed_units, Some('N'));
        assert_eq!(mwv.status, Some('A'));
    }

    #[test]
    fn mwv_encode_roundtrip() {
        let original = Mwv {
            wind_angle: Some(336.0),
            reference: Some('R'),
            wind_speed: Some(13.41),
            speed_units: Some('N'),
            status: Some('A'),
        };
        let sentence = original.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse MWV sentence");
        let parsed = Mwv::parse(&frame.fields).expect("parse MWV from re-encoded frame");

        assert_eq!(original, parsed);
    }

    #[test]
    fn mwv_true_signalk() {
        let frame = parse_frame("$IIMWV,074,T,05.85,N,A*2E").expect("valid");
        let mwv = Mwv::parse(&frame.fields).expect("parse MWV");
        assert_eq!(mwv.reference, Some('T'));
    }
}
