use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GST — GNSS Pseudo-Range Error Statistics.
///
/// Wire: `time,range_rms,std_major,std_minor,orient,std_lat,std_lon,std_alt`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gst {
    /// UTC time of observation (hhmmss.ss format).
    pub time: Option<String>,
    /// RMS value of standard deviation of range inputs in meters.
    pub range_rms: Option<f32>,
    /// Standard deviation of semi-major axis in meters.
    pub std_major: Option<f32>,
    /// Standard deviation of semi-minor axis in meters.
    pub std_minor: Option<f32>,
    /// Orientation of semi-major axis in degrees from true north.
    pub orient: Option<f32>,
    /// Standard deviation of latitude error in meters.
    pub std_lat: Option<f32>,
    /// Standard deviation of longitude error in meters.
    pub std_lon: Option<f32>,
    /// Standard deviation of altitude error in meters.
    pub std_alt: Option<f32>,
}

impl Gst {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            time: r.string(),
            range_rms: r.f32(),
            std_major: r.f32(),
            std_minor: r.f32(),
            orient: r.f32(),
            std_lat: r.f32(),
            std_lon: r.f32(),
            std_alt: r.f32(),
        })
    }
}

impl NmeaEncodable for Gst {
    const SENTENCE_TYPE: &str = "GST";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.f32(self.range_rms);
        w.f32(self.std_major);
        w.f32(self.std_minor);
        w.f32(self.orient);
        w.f32(self.std_lat);
        w.f32(self.std_lon);
        w.f32(self.std_alt);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn gst_empty() {
        let f = parse_frame("$GPGST,,,,,,,,*57").expect("valid");
        let g = Gst::parse(&f.fields).expect("parse");
        assert!(g.time.is_none());
        assert!(g.range_rms.is_none());
        assert!(g.std_major.is_none());
        assert!(g.std_minor.is_none());
        assert!(g.orient.is_none());
        assert!(g.std_lat.is_none());
        assert!(g.std_lon.is_none());
        assert!(g.std_alt.is_none());
    }

    #[test]
    fn gst_full_pynmeagps() {
        // pynmeagps fixture with all fields populated, GN talker
        let frame =
            parse_frame("$GNGST,103607.00,38,60,38,89,15,24,31*63").expect("valid pynmeagps GST");
        let gst = Gst::parse(&frame.fields).expect("parse GST");
        assert_eq!(gst.time, Some("103607.00".to_string()));
        assert!((gst.range_rms.expect("rms") - 38.0).abs() < 0.1);
        assert!((gst.std_major.expect("std_major") - 60.0).abs() < 0.1);
        assert!((gst.std_minor.expect("std_minor") - 38.0).abs() < 0.1);
        assert!((gst.orient.expect("orient") - 89.0).abs() < 0.1);
        assert!((gst.std_lat.expect("std_lat") - 15.0).abs() < 0.1);
        assert!((gst.std_lon.expect("std_lon") - 24.0).abs() < 0.1);
        assert!((gst.std_alt.expect("std_alt") - 31.0).abs() < 0.1);
    }

    #[test]
    fn gst_partial_gpsd() {
        let frame = parse_frame("$GPGST,131519.00,11,,,,0.70,0.49,1.1*53").expect("valid");
        let gst = Gst::parse(&frame.fields).expect("parse GST");
        assert_eq!(gst.time, Some("131519.00".to_string()));
        assert!((gst.range_rms.expect("rms") - 11.0).abs() < 0.1);
        assert!(gst.std_major.is_none());
        assert!(gst.std_minor.is_none());
        assert!(gst.orient.is_none());
        assert!((gst.std_lat.expect("std_lat") - 0.70).abs() < 0.01);
        assert!((gst.std_lon.expect("std_lon") - 0.49).abs() < 0.01);
        assert!((gst.std_alt.expect("std_alt") - 1.1).abs() < 0.1);
    }
    #[test]
    fn gst_encode_roundtrip() {
        let gst = Gst {
            time: Some("131519.00".to_string()),
            range_rms: Some(11.0),
            std_major: Some(5.2),
            std_minor: Some(3.1),
            orient: Some(45.0),
            std_lat: Some(0.7),
            std_lon: Some(0.49),
            std_alt: Some(1.1),
        };
        let sentence = gst.to_sentence("GP").expect("encode");
        assert!(sentence.starts_with("$GPGST,"));
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let gst2 = Gst::parse(&frame.fields).expect("re-parse GST");
        assert_eq!(gst, gst2);
    }
}
