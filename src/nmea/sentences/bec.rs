use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// BEC — Bearing and Distance to Waypoint (Dead Reckoning).
///
/// Wire: `time,lat,NS,lon,EW,bear_true,T,bear_mag,M,dist,N,wpt`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Bec {
    /// UTC time of fix.
    pub time: Option<String>,
    /// Latitude in NMEA ddmm.mmm format.
    pub lat: Option<f64>,
    /// North/South indicator.
    pub ns: Option<char>,
    /// Longitude in NMEA dddmm.mmm format.
    pub lon: Option<f64>,
    /// East/West indicator.
    pub ew: Option<char>,
    /// True bearing to waypoint in degrees.
    pub bear_true: Option<f32>,
    /// Magnetic bearing to waypoint in degrees.
    pub bear_mag: Option<f32>,
    /// Distance to waypoint in nautical miles.
    pub dist: Option<f32>,
    /// Destination waypoint identifier.
    pub wpt: Option<String>,
}

impl Bec {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let time = r.string();
        let lat = r.f64();
        let ns = r.char();
        let lon = r.f64();
        let ew = r.char();
        let bear_true = r.f32();
        r.skip(); // T
        let bear_mag = r.f32();
        r.skip(); // M
        let dist = r.f32();
        r.skip(); // N
        let wpt = r.string();
        Self {
            time,
            lat,
            ns,
            lon,
            ew,
            bear_true,
            bear_mag,
            dist,
            wpt,
        }
    }
}

impl NmeaEncodable for Bec {
    const SENTENCE_TYPE: &str = "BEC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.lat(self.lat);
        w.char(self.ns);
        w.lon(self.lon);
        w.char(self.ew);
        w.f32(self.bear_true);
        w.fixed('T');
        w.f32(self.bear_mag);
        w.fixed('M');
        w.f32(self.dist);
        w.fixed('N');
        w.string(self.wpt.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn bec_empty() {
        let s = Bec {
            time: None,
            lat: None,
            ns: None,
            lon: None,
            ew: None,
            bear_true: None,
            bear_mag: None,
            dist: None,
            wpt: None,
        }
        .to_sentence("GP").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let b = Bec::parse(&f.fields);
        assert!(b.time.is_none());
        assert!(b.dist.is_none());
    }

    #[test]
    fn bec_encode_roundtrip() {
        let original = Bec {
            time: Some("220516".to_string()),
            lat: Some(5130.02),
            ns: Some('N'),
            lon: Some(46.34),
            ew: Some('W'),
            bear_true: Some(213.8),
            bear_mag: Some(218.0),
            dist: Some(4.6),
            wpt: Some("EGLM".to_string()),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Bec::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn bec_gpbec_gonmea() {
        let frame =
            parse_frame("$GPBEC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*33")
                .expect("valid");
        let b = Bec::parse(&frame.fields);
        assert_eq!(b.time.as_deref(), Some("220516"));
        assert!((b.lat.expect("lat") - 5130.02).abs() < 0.01);
        assert_eq!(b.ns, Some('N'));
        assert_eq!(b.wpt.as_deref(), Some("EGLM"));
    }
}
