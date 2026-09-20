use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// BWR — Bearing & Distance to Waypoint — Rhumb Line.
///
/// Wire: `time,lat,ns,lon,ew,bear_true,T,bear_mag,M,dist,N,wpt,mode`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Bwr {
    /// UTC time of observation (hhmmss.ss format).
    pub time: Option<String>,
    /// Waypoint latitude in NMEA format (ddmm.mm).
    pub lat: Option<f64>,
    /// North/South indicator ('N' or 'S').
    pub ns: Option<char>,
    /// Waypoint longitude in NMEA format (dddmm.mm).
    pub lon: Option<f64>,
    /// East/West indicator ('E' or 'W').
    pub ew: Option<char>,
    /// Bearing true to waypoint in degrees.
    pub bear_true: Option<f32>,
    /// Bearing magnetic to waypoint in degrees.
    pub bear_mag: Option<f32>,
    /// Distance to waypoint in nautical miles.
    pub dist: Option<f32>,
    /// Waypoint identifier.
    pub wpt: Option<String>,
    /// Mode indicator ('A' = autonomous, 'D' = differential, etc.).
    pub mode: Option<char>,
}

impl Bwr {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
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
        let mode = r.char();
        Some(Self {
            time,
            lat,
            ns,
            lon,
            ew,
            bear_true,
            bear_mag,
            dist,
            wpt,
            mode,
        })
    }
}

impl NmeaEncodable for Bwr {
    const SENTENCE_TYPE: &str = "BWR";

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
        w.char(self.mode);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn bwr_empty() {
        let f = Bwr {
            time: None,
            lat: None,
            ns: None,
            lon: None,
            ew: None,
            bear_true: None,
            bear_mag: None,
            dist: None,
            wpt: None,
            mode: None,
        }
        .to_sentence("GP").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let b = Bwr::parse(&frame.fields).expect("parse");
        assert!(b.time.is_none());
        assert!(b.lat.is_none());
        assert!(b.wpt.is_none());
    }

    #[test]
    fn bwr_encode_roundtrip() {
        let original = Bwr {
            time: Some("225444".to_string()),
            lat: Some(4917.24),
            ns: Some('N'),
            lon: Some(12309.57),
            ew: Some('W'),
            bear_true: Some(51.9),
            bear_mag: Some(31.6),
            dist: Some(1.3),
            wpt: Some("004".to_string()),
            mode: None,
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Bwr::parse(&frame.fields).expect("re-parse BWR");
        assert_eq!(original, parsed);
    }

    #[test]
    fn bwr_minimal_gonmea() {
        let frame =
            parse_frame("$GPBWR,081837,,,,,,T,,M,,N,*02").expect("valid go-nmea BWR frame");
        let bwr = Bwr::parse(&frame.fields).expect("parse BWR");
        assert_eq!(bwr.time, Some("081837".to_string()));
        assert!(bwr.lat.is_none());
        assert!(bwr.ns.is_none());
        assert!(bwr.lon.is_none());
        assert!(bwr.ew.is_none());
        assert!(bwr.bear_true.is_none());
        assert!(bwr.bear_mag.is_none());
        assert!(bwr.dist.is_none());
        assert!(bwr.wpt.is_none());
        assert!(bwr.mode.is_none());
    }

    #[test]
    fn bwr_full_gonmea() {
        let frame =
            parse_frame("$GPBWR,225444,4917.24,N,12309.57,W,051.9,T,031.6,M,001.3,N,004*38")
                .expect("valid BWR frame");
        let bwr = Bwr::parse(&frame.fields).expect("parse BWR");
        assert_eq!(bwr.time, Some("225444".to_string()));
        assert!((bwr.lat.expect("lat") - 4917.24).abs() < 0.01);
        assert_eq!(bwr.ns, Some('N'));
        assert!((bwr.lon.expect("lon") - 12309.57).abs() < 0.01);
        assert_eq!(bwr.ew, Some('W'));
        assert!((bwr.bear_true.expect("bear_true") - 51.9).abs() < 0.1);
        assert!((bwr.bear_mag.expect("bear_mag") - 31.6).abs() < 0.1);
        assert!((bwr.dist.expect("dist") - 1.3).abs() < 0.1);
        assert_eq!(bwr.wpt, Some("004".to_string()));
        assert!(bwr.mode.is_none());
    }
}
