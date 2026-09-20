use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// BWC — Bearing & Distance to Waypoint — Great Circle.
///
/// Wire: `time,lat,ns,lon,ew,bear_true,T,bear_mag,M,dist,N,wpt,mode`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Bwc {
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

impl Bwc {
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

impl NmeaEncodable for Bwc {
    const SENTENCE_TYPE: &str = "BWC";

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
    fn bwc_empty() {
        let f = Bwc {
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
        let b = Bwc::parse(&frame.fields).expect("parse");
        assert!(b.time.is_none());
        assert!(b.lat.is_none());
        assert!(b.wpt.is_none());
    }

    #[test]
    fn bwc_encode_roundtrip() {
        let original = Bwc {
            time: Some("225444".to_string()),
            lat: Some(4917.24),
            ns: Some('N'),
            lon: Some(12309.57),
            ew: Some('W'),
            bear_true: Some(51.9),
            bear_mag: Some(31.6),
            dist: Some(1.3),
            wpt: Some("004".to_string()),
            mode: Some('A'),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Bwc::parse(&frame.fields).expect("re-parse BWC");
        assert_eq!(original, parsed);
    }

    #[test]
    fn bwc_no_position_signalk() {
        let frame = parse_frame("$IIBWC,200321,,,,,119.5,T,129.5,M,22.10,N,1*1E")
            .expect("valid SignalK BWC frame");
        let bwc = Bwc::parse(&frame.fields).expect("parse BWC");
        assert_eq!(bwc.time, Some("200321".to_string()));
        assert!(bwc.lat.is_none());
        assert!(bwc.ns.is_none());
        assert!(bwc.lon.is_none());
        assert!(bwc.ew.is_none());
        assert!((bwc.bear_true.expect("bear_true") - 119.5).abs() < 0.1);
        assert!((bwc.bear_mag.expect("bear_mag") - 129.5).abs() < 0.1);
        assert!((bwc.dist.expect("dist") - 22.10).abs() < 0.01);
        assert_eq!(bwc.wpt, Some("1".to_string()));
    }

    #[test]
    fn bwc_pynmeagps() {
        let frame =
            parse_frame("$GPBWC,220516,5130.02,N,00046.34,W,213.8,T,218.0,M,0004.6,N,EGLM*21")
                .expect("valid pynmeagps BWC frame");
        let bwc = Bwc::parse(&frame.fields).expect("parse BWC");
        assert_eq!(bwc.time, Some("220516".to_string()));
        assert!((bwc.lat.expect("lat") - 5130.02).abs() < 0.01);
        assert_eq!(bwc.ns, Some('N'));
        assert!((bwc.bear_true.expect("bear_true") - 213.8).abs() < 0.1);
        assert!((bwc.bear_mag.expect("bear_mag") - 218.0).abs() < 0.1);
        assert!((bwc.dist.expect("dist") - 4.6).abs() < 0.1);
        assert_eq!(bwc.wpt, Some("EGLM".to_string()));
    }

    #[test]
    fn bwc_lon_full_precision_and_padding() {
        // 9-sig-digit longitude that f32 cannot hold exactly, with a leading-zero degree.
        let frame = parse_frame("$GPBWC,220516,5130.02,N,00046.34678,W,213.8,T,218.0,M,0004.6,N,EGLM*18")
            .expect("valid BWC");
        let bwc = Bwc::parse(&frame.fields).expect("parse BWC");
        let lon = bwc.lon.expect("lon");
        assert!((lon - 46.34678).abs() < 1e-9, "f64 precision lost: {lon}");
        let sentence = bwc.to_sentence("GP").expect("encode");
        assert!(sentence.contains(",00046.34678,"), "leading zero lost: {sentence}");
    }
}
