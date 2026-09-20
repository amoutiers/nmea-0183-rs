use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// DBK — Depth Below Keel.
///
/// Wire: `depthF,f,depthM,M,depthFathoms,F`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dbk {
    /// Depth below keel in feet.
    pub depth_feet: Option<f32>,
    /// Depth below keel in meters.
    pub depth_meters: Option<f32>,
    /// Depth below keel in fathoms.
    pub depth_fathoms: Option<f32>,
}

impl Dbk {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let depth_feet = r.f32();
        r.skip(); // f
        let depth_meters = r.f32();
        r.skip(); // M
        let depth_fathoms = r.f32();
        Self {
            depth_feet,
            depth_meters,
            depth_fathoms,
        }
    }
}

impl NmeaEncodable for Dbk {
    const SENTENCE_TYPE: &str = "DBK";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.depth_feet);
        w.fixed('f');
        w.f32(self.depth_meters);
        w.fixed('M');
        w.f32(self.depth_fathoms);
        w.fixed('F');
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn dbk_empty() {
        let f = parse_frame("$IIDBK,,f,,M,,F*20").expect("valid");
        let d = Dbk::parse(&f.fields);
        assert!(d.depth_feet.is_none());
        assert!(d.depth_meters.is_none());
        assert!(d.depth_fathoms.is_none());
    }

    #[test]
    fn dbk_encode_roundtrip() {
        let dbk = Dbk {
            depth_feet: Some(35.53),
            depth_meters: Some(10.83),
            depth_fathoms: Some(5.85),
        };
        let sentence = dbk.to_sentence("II").expect("encode");
        assert!(sentence.starts_with("$IIDBK,"));
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let dbk2 = Dbk::parse(&frame.fields);
        assert_eq!(dbk, dbk2);
    }

    #[test]
    fn dbk_full_gonmea() {
        let f = parse_frame("$SDDBK,12.3,f,3.7,M,2.0,F*2F").expect("valid DBK from go-nmea");
        let d = Dbk::parse(&f.fields);
        assert!((d.depth_feet.expect("depth_feet present") - 12.3).abs() < 0.01);
        assert!((d.depth_meters.expect("depth_meters present") - 3.7).abs() < 0.01);
        assert!((d.depth_fathoms.expect("depth_fathoms present") - 2.0).abs() < 0.01);
    }

    #[test]
    fn dbk_signalk() {
        let f =
            parse_frame("$IIDBK,035.53,f,010.83,M,005.85,F*3C").expect("valid DBK from SignalK");
        let d = Dbk::parse(&f.fields);
        assert!((d.depth_meters.expect("depth_meters present") - 10.83).abs() < 0.01);
    }
}
