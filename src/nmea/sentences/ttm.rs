use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// TTM — Tracked Target Message.
///
/// Wire: `target_num,dist,bearing,bearing_type,speed,course,course_type,dist_cpa,time_cpa,speed_units,name,status,ref_target,time,acq_type`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ttm {
    /// Target number (0–99).
    pub target_num: Option<u8>,
    /// Distance to target in nautical miles.
    pub dist: Option<f32>,
    /// Bearing to target in degrees.
    pub bearing: Option<f32>,
    /// Bearing type (T=true, R=relative).
    pub bearing_type: Option<char>,
    /// Target speed in knots.
    pub speed: Option<f32>,
    /// Target course in degrees.
    pub course: Option<f32>,
    /// Course type (T=true, R=relative).
    pub course_type: Option<char>,
    /// Distance at closest point of approach (CPA).
    pub dist_cpa: Option<f32>,
    /// Time to CPA in minutes.
    pub time_cpa: Option<f32>,
    /// Speed/distance units (N=knots, S=statute miles, K=km).
    pub speed_units: Option<char>,
    /// Target name.
    pub name: Option<String>,
    /// Target status (L=lost, Q=acquiring, T=tracking).
    pub status: Option<char>,
    /// Reference target indicator.
    pub ref_target: Option<char>,
    /// UTC time of data.
    pub time: Option<String>,
    /// Acquisition type (A=automatic, M=manual, R=reported).
    pub acq_type: Option<char>,
}

impl Ttm {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let target_num = r.u8();
        let dist = r.f32();
        let bearing = r.f32();
        let bearing_type = r.char();
        let speed = r.f32();
        let course = r.f32();
        let course_type = r.char();
        let dist_cpa = r.f32();
        let time_cpa = r.f32();
        let speed_units = r.char();
        let name = r.string();
        let status = r.char();
        let ref_target = r.char();
        let time = r.string();
        let acq_type = r.char();
        Some(Self {
            target_num,
            dist,
            bearing,
            bearing_type,
            speed,
            course,
            course_type,
            dist_cpa,
            time_cpa,
            speed_units,
            name,
            status,
            ref_target,
            time,
            acq_type,
        })
    }
}

impl NmeaEncodable for Ttm {
    const SENTENCE_TYPE: &str = "TTM";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.target_num);
        w.f32(self.dist);
        w.f32(self.bearing);
        w.char(self.bearing_type);
        w.f32(self.speed);
        w.f32(self.course);
        w.char(self.course_type);
        w.f32(self.dist_cpa);
        w.f32(self.time_cpa);
        w.char(self.speed_units);
        w.string(self.name.as_deref());
        w.char(self.status);
        w.char(self.ref_target);
        w.string(self.time.as_deref());
        w.char(self.acq_type);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn ttm_empty() {
        let s = Ttm {
            target_num: None,
            dist: None,
            bearing: None,
            bearing_type: None,
            speed: None,
            course: None,
            course_type: None,
            dist_cpa: None,
            time_cpa: None,
            speed_units: None,
            name: None,
            status: None,
            ref_target: None,
            time: None,
            acq_type: None,
        }
        .to_sentence("RA").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let t = Ttm::parse(&f.fields).expect("parse");
        assert!(t.target_num.is_none());
        assert!(t.dist.is_none());
        assert!(t.acq_type.is_none());
    }

    #[test]
    fn ttm_encode_roundtrip() {
        let original = Ttm {
            target_num: Some(2),
            dist: Some(1.43),
            bearing: Some(170.5),
            bearing_type: Some('T'),
            speed: Some(0.16),
            course: Some(264.4),
            course_type: Some('T'),
            dist_cpa: Some(1.42),
            time_cpa: Some(36.9),
            speed_units: Some('N'),
            name: None,
            status: Some('T'),
            ref_target: None,
            time: None,
            acq_type: Some('M'),
        };
        let sentence = original.to_sentence("RA").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Ttm::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn ttm_rattm_gonmea() {
        let f = parse_frame("$RATTM,02,1.43,170.5,T,0.16,264.4,T,1.42,36.9,N,,T,,,M*2A")
            .expect("valid TTM");
        let t = Ttm::parse(&f.fields).expect("parse TTM");
        assert_eq!(t.target_num, Some(2));
        assert!((t.dist.expect("dist") - 1.43).abs() < 0.001);
        assert!((t.bearing.expect("bearing") - 170.5).abs() < 0.01);
        assert_eq!(t.bearing_type, Some('T'));
        assert!((t.speed.expect("speed") - 0.16).abs() < 0.001);
        assert!((t.course.expect("course") - 264.4).abs() < 0.01);
        assert_eq!(t.course_type, Some('T'));
        assert!((t.dist_cpa.expect("dcpa") - 1.42).abs() < 0.001);
        assert!((t.time_cpa.expect("tcpa") - 36.9).abs() < 0.01);
        assert_eq!(t.speed_units, Some('N'));
        assert!(t.name.is_none());
        assert_eq!(t.status, Some('T'));
        assert!(t.ref_target.is_none());
        assert!(t.time.is_none());
        assert_eq!(t.acq_type, Some('M'));
    }
}
