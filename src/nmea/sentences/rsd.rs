use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// RSD — RADAR System Data.
///
/// Wire: `origin1_range,origin1_bearing,vrm1,bearing_line1,origin2_range,origin2_bearing,vrm2,bearing_line2,cursor_range,cursor_bearing,range_scale,range_unit,display_rotation`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rsd {
    /// Origin 1 range.
    pub origin1_range: Option<f32>,
    /// Origin 1 bearing.
    pub origin1_bearing: Option<f32>,
    /// Variable Range Marker 1.
    pub vrm1: Option<f32>,
    /// Bearing Line 1.
    pub bearing_line1: Option<f32>,
    /// Origin 2 range.
    pub origin2_range: Option<f32>,
    /// Origin 2 bearing.
    pub origin2_bearing: Option<f32>,
    /// Variable Range Marker 2.
    pub vrm2: Option<f32>,
    /// Bearing Line 2.
    pub bearing_line2: Option<f32>,
    /// Cursor range from own ship.
    pub cursor_range: Option<f32>,
    /// Cursor bearing.
    pub cursor_bearing: Option<f32>,
    /// Range scale.
    pub range_scale: Option<f32>,
    /// Range unit (N=nautical miles, K=km, S=statute miles).
    pub range_unit: Option<char>,
    /// Display rotation (H=head up, N=north up, C=course up).
    pub display_rotation: Option<char>,
}

impl Rsd {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let origin1_range = r.f32();
        let origin1_bearing = r.f32();
        let vrm1 = r.f32();
        let bearing_line1 = r.f32();
        let origin2_range = r.f32();
        let origin2_bearing = r.f32();
        let vrm2 = r.f32();
        let bearing_line2 = r.f32();
        let cursor_range = r.f32();
        let cursor_bearing = r.f32();
        let range_scale = r.f32();
        let range_unit = r.char();
        let display_rotation = r.char();
        Some(Self {
            origin1_range,
            origin1_bearing,
            vrm1,
            bearing_line1,
            origin2_range,
            origin2_bearing,
            vrm2,
            bearing_line2,
            cursor_range,
            cursor_bearing,
            range_scale,
            range_unit,
            display_rotation,
        })
    }
}

impl NmeaEncodable for Rsd {
    const SENTENCE_TYPE: &str = "RSD";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.origin1_range);
        w.f32(self.origin1_bearing);
        w.f32(self.vrm1);
        w.f32(self.bearing_line1);
        w.f32(self.origin2_range);
        w.f32(self.origin2_bearing);
        w.f32(self.vrm2);
        w.f32(self.bearing_line2);
        w.f32(self.cursor_range);
        w.f32(self.cursor_bearing);
        w.f32(self.range_scale);
        w.char(self.range_unit);
        w.char(self.display_rotation);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn rsd_empty() {
        let s = Rsd {
            origin1_range: None,
            origin1_bearing: None,
            vrm1: None,
            bearing_line1: None,
            origin2_range: None,
            origin2_bearing: None,
            vrm2: None,
            bearing_line2: None,
            cursor_range: None,
            cursor_bearing: None,
            range_scale: None,
            range_unit: None,
            display_rotation: None,
        }
        .to_sentence("RA").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let r = Rsd::parse(&f.fields).expect("parse");
        assert!(r.origin1_range.is_none());
        assert!(r.display_rotation.is_none());
    }

    #[test]
    fn rsd_encode_roundtrip() {
        let original = Rsd {
            origin1_range: Some(0.0),
            origin1_bearing: None,
            vrm1: Some(2.5),
            bearing_line1: Some(5.0),
            origin2_range: Some(0.0),
            origin2_bearing: None,
            vrm2: Some(4.5),
            bearing_line2: Some(355.0),
            cursor_range: None,
            cursor_bearing: None,
            range_scale: Some(3.0),
            range_unit: Some('N'),
            display_rotation: Some('H'),
        };
        let sentence = original.to_sentence("RA").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Rsd::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn rsd_cursor_only_gonmea() {
        let f = parse_frame("$RARSD,,,,,,,,,0.808,326.9,0.750,N,N*58").expect("valid RSD");
        let r = Rsd::parse(&f.fields).expect("parse RSD");
        assert!(r.origin1_range.is_none());
        assert!(r.vrm1.is_none());
        assert!(r.origin2_range.is_none());
        assert!(r.vrm2.is_none());
        assert!((r.cursor_range.expect("cr") - 0.808).abs() < 0.001);
        assert!((r.cursor_bearing.expect("cb") - 326.9).abs() < 0.1);
        assert!((r.range_scale.expect("rs") - 0.75).abs() < 0.001);
        assert_eq!(r.range_unit, Some('N'));
        assert_eq!(r.display_rotation, Some('N'));
    }

    #[test]
    fn rsd_rarsd_gonmea() {
        let f = parse_frame("$RARSD,0.00,,2.50,005.0,0.00,,4.50,355.0,,,3.0,N,H*51")
            .expect("valid RSD");
        let r = Rsd::parse(&f.fields).expect("parse RSD");
        assert!((r.origin1_range.expect("o1r") - 0.0).abs() < 0.01);
        assert!(r.origin1_bearing.is_none());
        assert!((r.vrm1.expect("vrm1") - 2.5).abs() < 0.01);
        assert!((r.bearing_line1.expect("bl1") - 5.0).abs() < 0.01);
        assert!((r.vrm2.expect("vrm2") - 4.5).abs() < 0.01);
        assert!((r.bearing_line2.expect("bl2") - 355.0).abs() < 0.01);
        assert!(r.cursor_range.is_none());
        assert!(r.cursor_bearing.is_none());
        assert!((r.range_scale.expect("rs") - 3.0).abs() < 0.01);
        assert_eq!(r.range_unit, Some('N'));
        assert_eq!(r.display_rotation, Some('H'));
    }
}
