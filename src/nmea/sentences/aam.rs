use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// AAM — Waypoint Arrival Alarm.
///
/// Wire: `arrce,perp,crad,cunit,wpt`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Aam {
    /// Arrival circle entered ('A' = entered, 'V' = not entered).
    pub arrce: Option<char>,
    /// Perpendicular passed ('A' = passed, 'V' = not passed).
    pub perp: Option<char>,
    /// Arrival circle radius.
    pub crad: Option<f32>,
    /// Arrival circle radius unit ('N' = nautical miles).
    pub cunit: Option<char>,
    /// Waypoint identifier.
    pub wpt: Option<String>,
}

impl Aam {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            arrce: r.char(),
            perp: r.char(),
            crad: r.f32(),
            cunit: r.char(),
            wpt: r.string(),
        }
    }
}

impl NmeaEncodable for Aam {
    const SENTENCE_TYPE: &str = "AAM";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.arrce);
        w.char(self.perp);
        w.f32(self.crad);
        w.char(self.cunit);
        w.string(self.wpt.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn aam_empty() {
        let f = Aam {
            arrce: None,
            perp: None,
            crad: None,
            cunit: None,
            wpt: None,
        }
        .to_sentence("GP").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let a = Aam::parse(&frame.fields);
        assert!(a.arrce.is_none());
        assert!(a.perp.is_none());
        assert!(a.crad.is_none());
        assert!(a.cunit.is_none());
        assert!(a.wpt.is_none());
    }

    #[test]
    fn aam_encode_roundtrip() {
        let original = Aam {
            arrce: Some('A'),
            perp: Some('A'),
            crad: Some(0.1),
            cunit: Some('N'),
            wpt: Some("DEST".to_string()),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Aam::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn aam_pynmeagps() {
        let frame =
            parse_frame("$GPAAM,A,A,0.10,N,WPTNME*32").expect("valid pynmeagps AAM frame");
        let aam = Aam::parse(&frame.fields);
        assert_eq!(aam.arrce, Some('A'));
        assert_eq!(aam.perp, Some('A'));
        assert!((aam.crad.expect("crad") - 0.10).abs() < 0.01);
        assert_eq!(aam.cunit, Some('N'));
        assert_eq!(aam.wpt, Some("WPTNME".to_string()));
    }
}
