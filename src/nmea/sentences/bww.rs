use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// BWW — Bearing, Waypoint to Waypoint.
///
/// Wire: `bear_true,T,bear_mag,M,wpt_dest,wpt_origin`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Bww {
    /// Bearing true in degrees.
    pub bear_true: Option<f32>,
    /// True bearing type indicator ('T').
    pub bear_true_type: Option<char>,
    /// Bearing magnetic in degrees.
    pub bear_mag: Option<f32>,
    /// Magnetic bearing type indicator ('M').
    pub bear_mag_type: Option<char>,
    /// Destination waypoint identifier.
    pub wpt_dest: Option<String>,
    /// Origin waypoint identifier.
    pub wpt_origin: Option<String>,
}

impl Bww {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let bear_true = r.f32();
        let bear_true_type = r.char();
        let bear_mag = r.f32();
        let bear_mag_type = r.char();
        let wpt_dest = r.string();
        let wpt_origin = r.string();
        Some(Self {
            bear_true,
            bear_true_type,
            bear_mag,
            bear_mag_type,
            wpt_dest,
            wpt_origin,
        })
    }
}

impl NmeaEncodable for Bww {
    const SENTENCE_TYPE: &str = "BWW";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.bear_true);
        w.char(self.bear_true_type);
        w.f32(self.bear_mag);
        w.char(self.bear_mag_type);
        w.string(self.wpt_dest.as_deref());
        w.string(self.wpt_origin.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn bww_empty() {
        let s = Bww {
            bear_true: None,
            bear_true_type: None,
            bear_mag: None,
            bear_mag_type: None,
            wpt_dest: None,
            wpt_origin: None,
        }
        .to_sentence("GP").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let b = Bww::parse(&f.fields).expect("parse");
        assert!(b.bear_true.is_none());
        assert!(b.wpt_dest.is_none());
    }

    #[test]
    fn bww_encode_roundtrip() {
        let original = Bww {
            bear_true: Some(97.0),
            bear_true_type: Some('T'),
            bear_mag: Some(103.2),
            bear_mag_type: Some('M'),
            wpt_dest: Some("POINTB".to_string()),
            wpt_origin: Some("POINTA".to_string()),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Bww::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn bww_gpbww_gonmea() {
        let frame = parse_frame("$GPBWW,097.0,T,103.2,M,POINTB,POINTA*41").expect("valid");
        let b = Bww::parse(&frame.fields).expect("parse");
        assert!((b.bear_true.expect("bear_true") - 97.0).abs() < 0.1);
        assert_eq!(b.bear_true_type, Some('T'));
        assert!((b.bear_mag.expect("bear_mag") - 103.2).abs() < 0.1);
        assert_eq!(b.bear_mag_type, Some('M'));
        assert_eq!(b.wpt_dest.as_deref(), Some("POINTB"));
        assert_eq!(b.wpt_origin.as_deref(), Some("POINTA"));
    }
}
