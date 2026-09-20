use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// WPL — Waypoint Location.
///
/// Wire: `lat,NS,lon,EW,ident`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Wpl {
    /// Latitude in NMEA ddmm.mmm format.
    pub lat: Option<f64>,
    /// North/South indicator.
    pub ns: Option<char>,
    /// Longitude in NMEA dddmm.mmm format.
    pub lon: Option<f64>,
    /// East/West indicator.
    pub ew: Option<char>,
    /// Waypoint identifier.
    pub ident: Option<String>,
}

impl Wpl {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let lat = r.f64();
        let ns = r.char();
        let lon = r.f64();
        let ew = r.char();
        let ident = r.string();
        Self {
            lat,
            ns,
            lon,
            ew,
            ident,
        }
    }
}

impl NmeaEncodable for Wpl {
    const SENTENCE_TYPE: &str = "WPL";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.lat(self.lat);
        w.char(self.ns);
        w.lon(self.lon);
        w.char(self.ew);
        w.string(self.ident.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn wpl_empty() {
        let s = Wpl {
            lat: None,
            ns: None,
            lon: None,
            ew: None,
            ident: None,
        }
        .to_sentence("II").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let w = Wpl::parse(&f.fields);
        assert!(w.lat.is_none());
        assert!(w.ident.is_none());
    }

    #[test]
    fn wpl_encode_roundtrip() {
        let original = Wpl {
            lat: Some(5503.453),
            ns: Some('N'),
            lon: Some(1037.2742),
            ew: Some('E'),
            ident: Some("411".to_string()),
        };
        let sentence = original.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Wpl::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn wpl_iiwpl_gonmea() {
        let frame = parse_frame("$IIWPL,5503.4530,N,01037.2742,E,411*6F").expect("valid");
        let w = Wpl::parse(&frame.fields);
        assert!((w.lat.expect("lat") - 5503.453).abs() < 0.001);
        assert_eq!(w.ns, Some('N'));
        assert!((w.lon.expect("lon") - 1037.2742).abs() < 0.001);
        assert_eq!(w.ew, Some('E'));
        assert_eq!(w.ident.as_deref(), Some("411"));
    }

    #[test]
    fn wpl_southern_hemisphere_gonmea() {
        let frame = parse_frame("$IIWPL,3356.4650,S,15124.5567,E,411*73").expect("valid");
        let w = Wpl::parse(&frame.fields);
        assert!((w.lat.expect("lat") - 3356.465).abs() < 0.001);
        assert_eq!(w.ns, Some('S'));
        assert!((w.lon.expect("lon") - 15124.5567).abs() < 0.001);
        assert_eq!(w.ew, Some('E'));
        assert_eq!(w.ident.as_deref(), Some("411"));
    }
}
