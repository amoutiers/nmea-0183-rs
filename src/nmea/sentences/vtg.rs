use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VTG — Track Made Good and Ground Speed.
///
/// Wire: `courseT,T,courseM,M,speedKts,N,speedKmh,K,mode`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vtg {
    /// Course over ground true in degrees.
    pub course_true: Option<f32>,
    /// Course over ground magnetic in degrees.
    pub course_mag: Option<f32>,
    /// Speed over ground in knots.
    pub speed_kts: Option<f32>,
    /// Speed over ground in km/h.
    pub speed_kmh: Option<f32>,
    /// Mode indicator ('A' = autonomous, 'D' = differential, etc.).
    pub mode: Option<char>,
}

impl Vtg {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let course_true = r.f32();
        r.skip();
        let course_mag = r.f32();
        r.skip();
        let speed_kts = r.f32();
        r.skip();
        let speed_kmh = r.f32();
        r.skip();
        let mode = r.char();
        Self {
            course_true,
            course_mag,
            speed_kts,
            speed_kmh,
            mode,
        }
    }
}

impl NmeaEncodable for Vtg {
    const SENTENCE_TYPE: &str = "VTG";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.course_true);
        w.fixed('T');
        w.f32(self.course_mag);
        w.fixed('M');
        w.f32(self.speed_kts);
        w.fixed('N');
        w.f32(self.speed_kmh);
        w.fixed('K');
        w.char(self.mode);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn vtg_empty() {
        let f = parse_frame("$IIVTG,,,,,,,,,*69").expect("valid");
        let v = Vtg::parse(&f.fields);
        assert!(v.course_true.is_none());
        assert!(v.course_mag.is_none());
        assert!(v.speed_kts.is_none());
        assert!(v.speed_kmh.is_none());
        assert!(v.mode.is_none());
    }

    #[test]
    fn vtg_full_signalk() {
        let f = parse_frame("$GPVTG,0.0,T,359.3,M,0.0,N,0.0,K,A*2F").expect("valid VTG frame");
        let v = Vtg::parse(&f.fields);
        assert!((v.course_mag.expect("course_mag present") - 359.3).abs() < 0.01);
        assert_eq!(v.mode, Some('A'));
    }

    #[test]
    fn vtg_missing_course_signalk() {
        let f = parse_frame("$GPVTG,,T,,M,0.102,N,0.190,K,A*28").expect("valid VTG frame");
        let v = Vtg::parse(&f.fields);
        assert!(v.course_true.is_none());
        assert!((v.speed_kts.expect("speed_kts present") - 0.102).abs() < 0.001);
    }

    #[test]
    fn vtg_encode_roundtrip() {
        let v = Vtg {
            course_true: Some(0.0),
            course_mag: Some(359.3),
            speed_kts: Some(5.0),
            speed_kmh: Some(9.26),
            mode: Some('A'),
        };
        let s = v.to_sentence("GP").expect("encode");
        let f = parse_frame(s.trim()).expect("re-parse VTG frame");
        let v2 = Vtg::parse(&f.fields);
        assert_eq!(v, v2);
    }
}
