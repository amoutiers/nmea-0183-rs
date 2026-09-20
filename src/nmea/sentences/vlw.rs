use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VLW — Distance Traveled through Water.
///
/// Wire: `twd,twdUnit,wd,wdUnit[,tgd,tgdUnit,gd,gdUnit]`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vlw {
    /// Total cumulative water distance in nautical miles.
    pub total_water_dist: Option<f32>,
    /// Total water distance unit ('N' = nautical miles).
    pub total_water_dist_unit: Option<char>,
    /// Water distance since reset in nautical miles.
    pub water_dist: Option<f32>,
    /// Water distance unit ('N' = nautical miles).
    pub water_dist_unit: Option<char>,
    /// Total cumulative ground distance in nautical miles (NMEA 3.0+).
    pub total_ground_dist: Option<f32>,
    /// Total ground distance unit ('N' = nautical miles).
    pub total_ground_dist_unit: Option<char>,
    /// Ground distance since reset in nautical miles (NMEA 3.0+).
    pub ground_dist: Option<f32>,
    /// Ground distance unit ('N' = nautical miles).
    pub ground_dist_unit: Option<char>,
}

impl Vlw {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            total_water_dist: r.f32(),
            total_water_dist_unit: r.char(),
            water_dist: r.f32(),
            water_dist_unit: r.char(),
            total_ground_dist: r.f32(),
            total_ground_dist_unit: r.char(),
            ground_dist: r.f32(),
            ground_dist_unit: r.char(),
        })
    }
}

impl NmeaEncodable for Vlw {
    const SENTENCE_TYPE: &str = "VLW";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.total_water_dist);
        w.char(self.total_water_dist_unit);
        w.f32(self.water_dist);
        w.char(self.water_dist_unit);
        w.f32(self.total_ground_dist);
        w.char(self.total_ground_dist_unit);
        w.f32(self.ground_dist);
        w.char(self.ground_dist_unit);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn vlw_empty() {
        let f = Vlw {
            total_water_dist: None,
            total_water_dist_unit: None,
            water_dist: None,
            water_dist_unit: None,
            total_ground_dist: None,
            total_ground_dist_unit: None,
            ground_dist: None,
            ground_dist_unit: None,
        }
        .to_sentence("II").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let v = Vlw::parse(&frame.fields).expect("parse");
        assert!(v.total_water_dist.is_none());
        assert!(v.water_dist.is_none());
        assert!(v.total_ground_dist.is_none());
        assert!(v.ground_dist.is_none());
    }

    #[test]
    fn vlw_encode_roundtrip() {
        let original = Vlw {
            total_water_dist: Some(10.1),
            total_water_dist_unit: Some('N'),
            water_dist: Some(3.2),
            water_dist_unit: Some('N'),
            total_ground_dist: None,
            total_ground_dist_unit: None,
            ground_dist: None,
            ground_dist_unit: None,
        };
        let sentence = original.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Vlw::parse(&frame.fields).expect("re-parse VLW");
        assert_eq!(original, parsed);
    }

    #[test]
    fn vlw_pynmeagps() {
        let frame =
            parse_frame("$GNVLW,,N,,N,0.000,N,0.000,N*44").expect("valid pynmeagps VLW frame");
        let vlw = Vlw::parse(&frame.fields).expect("parse VLW");
        assert!(vlw.total_water_dist.is_none());
        assert_eq!(vlw.total_water_dist_unit, Some('N'));
        assert!(vlw.water_dist.is_none());
        assert_eq!(vlw.water_dist_unit, Some('N'));
        assert!((vlw.total_ground_dist.expect("tgd") - 0.0).abs() < 0.001);
        assert_eq!(vlw.total_ground_dist_unit, Some('N'));
        assert!((vlw.ground_dist.expect("gd") - 0.0).abs() < 0.001);
        assert_eq!(vlw.ground_dist_unit, Some('N'));
    }

    #[test]
    fn vlw_with_ground_gonmea() {
        let frame =
            parse_frame("$IIVLW,10.1,N,3.2,N,1,N,0.1,N*62").expect("valid go-nmea VLW frame");
        let vlw = Vlw::parse(&frame.fields).expect("parse VLW");
        assert!((vlw.total_water_dist.expect("twd") - 10.1).abs() < 0.01);
        assert_eq!(vlw.total_water_dist_unit, Some('N'));
        assert!((vlw.water_dist.expect("wd") - 3.2).abs() < 0.01);
        assert_eq!(vlw.water_dist_unit, Some('N'));
        assert!((vlw.total_ground_dist.expect("tgd") - 1.0).abs() < 0.01);
        assert_eq!(vlw.total_ground_dist_unit, Some('N'));
        assert!((vlw.ground_dist.expect("gd") - 0.1).abs() < 0.01);
        assert_eq!(vlw.ground_dist_unit, Some('N'));
    }
}
