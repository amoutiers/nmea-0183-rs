use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// BOD — Bearing Origin to Destination.
///
/// Wire: `bear_true,bear_true_type,bear_mag,bear_mag_type,wpt_dest,wpt_origin`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Bod {
    /// Bearing true in degrees.
    pub bear_true: Option<f32>,
    /// Bearing true type indicator ('T' = true).
    pub bear_true_type: Option<char>,
    /// Bearing magnetic in degrees.
    pub bear_mag: Option<f32>,
    /// Bearing magnetic type indicator ('M' = magnetic).
    pub bear_mag_type: Option<char>,
    /// Destination waypoint identifier.
    pub wpt_dest: Option<String>,
    /// Origin waypoint identifier.
    pub wpt_origin: Option<String>,
}

impl Bod {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            bear_true: r.f32(),
            bear_true_type: r.char(),
            bear_mag: r.f32(),
            bear_mag_type: r.char(),
            wpt_dest: r.string(),
            wpt_origin: r.string(),
        })
    }
}

impl NmeaEncodable for Bod {
    const SENTENCE_TYPE: &str = "BOD";

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
    fn bod_destination_only_gonmea() {
        let frame =
            parse_frame("$GPBOD,099.3,T,105.6,M,POINTB*64").expect("valid go-nmea BOD frame");
        let bod = Bod::parse(&frame.fields).expect("parse BOD");
        assert!((bod.bear_true.expect("bear_true") - 99.3).abs() < 0.1);
        assert_eq!(bod.bear_true_type, Some('T'));
        assert!((bod.bear_mag.expect("bear_mag") - 105.6).abs() < 0.1);
        assert_eq!(bod.bear_mag_type, Some('M'));
        assert_eq!(bod.wpt_dest, Some("POINTB".to_string()));
        assert!(bod.wpt_origin.is_none());
    }

    #[test]
    fn bod_empty() {
        let f = Bod {
            bear_true: None,
            bear_true_type: None,
            bear_mag: None,
            bear_mag_type: None,
            wpt_dest: None,
            wpt_origin: None,
        }
        .to_sentence("GP").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let b = Bod::parse(&frame.fields).expect("parse");
        assert!(b.bear_true.is_none());
        assert!(b.wpt_dest.is_none());
        assert!(b.wpt_origin.is_none());
    }

    #[test]
    fn bod_encode_roundtrip() {
        let original = Bod {
            bear_true: Some(45.0),
            bear_true_type: Some('T'),
            bear_mag: Some(23.0),
            bear_mag_type: Some('M'),
            wpt_dest: Some("DEST".to_string()),
            wpt_origin: Some("START".to_string()),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Bod::parse(&frame.fields).expect("re-parse BOD");
        assert_eq!(original, parsed);
    }

    #[test]
    fn bod_pynmeagps() {
        let frame =
            parse_frame("$GPBOD,097.0,T,103.2,M,POINTB,POINTA*4A").expect("valid pynmeagps BOD frame");
        let bod = Bod::parse(&frame.fields).expect("parse BOD");
        assert!((bod.bear_true.expect("bear_true") - 97.0).abs() < 0.1);
        assert_eq!(bod.bear_true_type, Some('T'));
        assert!((bod.bear_mag.expect("bear_mag") - 103.2).abs() < 0.1);
        assert_eq!(bod.bear_mag_type, Some('M'));
        assert_eq!(bod.wpt_dest, Some("POINTB".to_string()));
        assert_eq!(bod.wpt_origin, Some("POINTA".to_string()));
    }
}
