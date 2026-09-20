use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// OSD — Own Ship Data.
///
/// Wire: `heading,heading_status,vessel_course,course_ref,vessel_speed,speed_ref,vessel_set,vessel_drift,speed_units`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Osd {
    /// Heading in degrees.
    pub heading: Option<f32>,
    /// Heading status ('A' = valid).
    pub heading_status: Option<char>,
    /// Vessel course in degrees referenced to North.
    pub vessel_course: Option<f32>,
    /// Course reference ('T' = true, 'M' = magnetic, 'W' = water, 'R' = relative).
    pub course_ref: Option<char>,
    /// Vessel speed.
    pub vessel_speed: Option<f32>,
    /// Speed reference ('B' = bottom, 'W' = water, 'R' = relative).
    pub speed_ref: Option<char>,
    /// Vessel set (current direction) in degrees true.
    pub vessel_set: Option<f32>,
    /// Vessel drift (current speed).
    pub vessel_drift: Option<f32>,
    /// Speed units ('N' = knots, 'S' = statute miles/hour, 'K' = km/h).
    pub speed_units: Option<char>,
}

impl Osd {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let heading = r.f32();
        let heading_status = r.char();
        let vessel_course = r.f32();
        let course_ref = r.char();
        let vessel_speed = r.f32();
        let speed_ref = r.char();
        let vessel_set = r.f32();
        let vessel_drift = r.f32();
        let speed_units = r.char();
        Some(Self {
            heading,
            heading_status,
            vessel_course,
            course_ref,
            vessel_speed,
            speed_ref,
            vessel_set,
            vessel_drift,
            speed_units,
        })
    }
}

impl NmeaEncodable for Osd {
    const SENTENCE_TYPE: &str = "OSD";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.heading);
        w.char(self.heading_status);
        w.f32(self.vessel_course);
        w.char(self.course_ref);
        w.f32(self.vessel_speed);
        w.char(self.speed_ref);
        w.f32(self.vessel_set);
        w.f32(self.vessel_drift);
        w.char(self.speed_units);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn osd_empty() {
        let s = Osd {
            heading: None,
            heading_status: None,
            vessel_course: None,
            course_ref: None,
            vessel_speed: None,
            speed_ref: None,
            vessel_set: None,
            vessel_drift: None,
            speed_units: None,
        }
        .to_sentence("RA").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let o = Osd::parse(&f.fields).expect("parse");
        assert!(o.heading.is_none());
        assert!(o.speed_units.is_none());
    }

    #[test]
    fn osd_encode_roundtrip() {
        let original = Osd {
            heading: Some(179.0),
            heading_status: Some('A'),
            vessel_course: Some(179.0),
            course_ref: Some('M'),
            vessel_speed: Some(0.0),
            speed_ref: Some('M'),
            vessel_set: None,
            vessel_drift: None,
            speed_units: Some('N'),
        };
        let sentence = original.to_sentence("RA").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Osd::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn osd_raosd_gonmea() {
        let frame = parse_frame("$RAOSD,179.0,A,179.0,M,00.0,M,,,N*76").expect("valid");
        let o = Osd::parse(&frame.fields).expect("parse");
        assert!((o.heading.expect("heading") - 179.0).abs() < 0.1);
        assert_eq!(o.heading_status, Some('A'));
        assert_eq!(o.course_ref, Some('M'));
        assert!(o.vessel_set.is_none());
        assert_eq!(o.speed_units, Some('N'));
    }
}
