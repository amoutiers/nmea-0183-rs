use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// XTR: Cross-track Error, Dead Reckoning.
///
/// Wire: `cross_track_error,steer_direction,unit`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Xtr {
    /// Cross-track error magnitude in the specified distance unit.
    pub cross_track_error: Option<f32>,
    /// Direction to steer ('L' = left, 'R' = right).
    pub steer_direction: Option<char>,
    /// Distance unit ('N' = nautical miles).
    pub unit: Option<char>,
}

impl Xtr {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            cross_track_error: r.f32(),
            steer_direction: r.char(),
            unit: r.char(),
        }
    }
}

impl NmeaEncodable for Xtr {
    const SENTENCE_TYPE: &str = "XTR";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.cross_track_error);
        w.char(self.steer_direction);
        w.char(self.unit);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn xtr_empty() {
        assert_eq!(Xtr::parse(&["", "", ""]), Xtr::default());
    }

    #[test]
    fn xtr_encode_roundtrip() {
        let original = Xtr {
            cross_track_error: Some(0.125),
            steer_direction: Some('L'),
            unit: Some('N'),
        };
        let sentence = original.to_sentence("GP").expect("encode XTR");
        assert_eq!(sentence.split('*').next(), Some("$GPXTR,0.125,L,N"));
        let frame = parse_frame(&sentence).expect("re-parse XTR");
        assert_eq!(Xtr::parse(&frame.fields), original);
    }

    #[test]
    fn xtr_malformed_fields() {
        assert_eq!(Xtr::parse(&["invalid", "LEFT", "NM"]), Xtr::default());
        assert_eq!(Xtr::parse(&["NaN", "", ""]), Xtr::default());
    }

    #[test]
    fn xtr_missing_fields() {
        assert_eq!(Xtr::parse(&[]), Xtr::default());
        assert_eq!(Xtr::parse(&["0.125"]), Xtr {
            cross_track_error: Some(0.125),
            ..Default::default()
        });
    }

    #[test]
    fn xtr_values_marnav() {
        let frame = parse_frame("$GPXTR,,,*65").expect("valid marnav XTR");
        let value = Xtr::parse(&frame.fields);
        assert_eq!(value.cross_track_error, None);
        assert_eq!(value.steer_direction, None);
        assert_eq!(value.unit, None);
    }
}
