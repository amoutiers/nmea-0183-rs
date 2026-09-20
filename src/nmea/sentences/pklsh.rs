use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PKLSH — Kenwood Short Position Report (Long format).
///
/// Proprietary Kenwood sentence.
/// Wire: `$PKLSH,lat,ns,lon,ew,time,validity,fleet,unit_id`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PKLSH"`.
#[derive(Debug, Clone, PartialEq)]
pub struct Pklsh {
    /// Latitude in DDDMM.MMM format.
    pub lat: Option<f64>,
    /// North/South indicator.
    pub ns: Option<char>,
    /// Longitude in DDDMM.MMM format.
    pub lon: Option<f64>,
    /// East/West indicator.
    pub ew: Option<char>,
    /// UTC time (hhmmss).
    pub time: Option<String>,
    /// Validity (A=valid, V=invalid).
    pub validity: Option<char>,
    /// Fleet identifier.
    pub fleet: Option<String>,
    /// Unit identifier.
    pub unit_id: Option<String>,
}

impl Pklsh {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let lat = r.f64();
        let ns = r.char();
        let lon = r.f64();
        let ew = r.char();
        let time = r.string();
        let validity = r.char();
        let fleet = r.string();
        let unit_id = r.string();
        Some(Self {
            lat,
            ns,
            lon,
            ew,
            time,
            validity,
            fleet,
            unit_id,
        })
    }
}

impl NmeaEncodable for Pklsh {
    const SENTENCE_TYPE: &str = "PKLSH";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f64(self.lat);
        w.char(self.ns);
        w.f64(self.lon);
        w.char(self.ew);
        w.string(self.time.as_deref());
        w.char(self.validity);
        w.string(self.fleet.as_deref());
        w.string(self.unit_id.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn pklsh_empty() {
        let s = Pklsh {
            lat: None,
            ns: None,
            lon: None,
            ew: None,
            time: None,
            validity: None,
            fleet: None,
            unit_id: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Pklsh::parse(&f.fields).expect("parse");
        assert!(p.lat.is_none());
        assert!(p.unit_id.is_none());
    }

    #[test]
    fn pklsh_encode_roundtrip() {
        let original = Pklsh {
            lat: Some(3926.7952),
            ns: Some('N'),
            lon: Some(12000.5947),
            ew: Some('W'),
            time: Some("022732".to_string()),
            validity: Some('A'),
            fleet: Some("100".to_string()),
            unit_id: Some("2000".to_string()),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Pklsh::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn pklsh_kenwood_gonmea() {
        let f =
            parse_frame("$PKLSH,3926.7952,N,12000.5947,W,022732,A,100,2000*1A").expect("valid PKLSH");
        let p = Pklsh::parse(&f.fields).expect("parse PKLSH");
        assert!((p.lat.expect("lat") - 3926.7952).abs() < 0.0001);
        assert_eq!(p.ns, Some('N'));
        assert!((p.lon.expect("lon") - 12000.5947).abs() < 0.0001);
        assert_eq!(p.ew, Some('W'));
        assert_eq!(p.time, Some("022732".to_string()));
        assert_eq!(p.validity, Some('A'));
        assert_eq!(p.fleet, Some("100".to_string()));
        assert_eq!(p.unit_id, Some("2000".to_string()));
    }
}
