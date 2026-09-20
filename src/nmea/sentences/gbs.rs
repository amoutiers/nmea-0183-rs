use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GBS — GNSS Satellite Fault Detection.
///
/// Wire: `time,err_lat,err_lon,err_alt,svid,prob,bias,stddev`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gbs {
    /// UTC time of observation (hhmmss.ss format).
    pub time: Option<String>,
    /// Expected error in latitude in meters.
    pub err_lat: Option<f32>,
    /// Expected error in longitude in meters.
    pub err_lon: Option<f32>,
    /// Expected error in altitude in meters.
    pub err_alt: Option<f32>,
    /// Satellite ID of most likely failed satellite.
    pub svid: Option<u32>,
    /// Probability of missed detection for most likely failed satellite.
    pub prob: Option<f32>,
    /// Estimate of bias in meters on most likely failed satellite.
    pub bias: Option<f32>,
    /// Standard deviation of bias estimate in meters.
    pub stddev: Option<f32>,
}

impl Gbs {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            time: r.string(),
            err_lat: r.f32(),
            err_lon: r.f32(),
            err_alt: r.f32(),
            svid: r.u32(),
            prob: r.f32(),
            bias: r.f32(),
            stddev: r.f32(),
        })
    }
}

impl NmeaEncodable for Gbs {
    const SENTENCE_TYPE: &str = "GBS";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.f32(self.err_lat);
        w.f32(self.err_lon);
        w.f32(self.err_alt);
        w.u32(self.svid);
        w.f32(self.prob);
        w.f32(self.bias);
        w.f32(self.stddev);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn gbs_empty() {
        let f = parse_frame("$GPGBS,,,,,,,,*41").expect("valid");
        let g = Gbs::parse(&f.fields).expect("parse");
        assert!(g.time.is_none());
        assert!(g.err_lat.is_none());
        assert!(g.err_lon.is_none());
        assert!(g.err_alt.is_none());
        assert!(g.svid.is_none());
        assert!(g.prob.is_none());
        assert!(g.bias.is_none());
        assert!(g.stddev.is_none());
    }

    #[test]
    fn gbs_full_pynmeagps() {
        // pynmeagps fixture with svid, bias, stddev populated
        let frame = parse_frame("$GPGBS,235458.00,1.4,1.3,3.1,03,,-21.4,3.8,1,0*5A")
            .expect("valid pynmeagps full GBS");
        let gbs = Gbs::parse(&frame.fields).expect("parse GBS");
        assert_eq!(gbs.time, Some("235458.00".to_string()));
        assert!((gbs.err_lat.expect("err_lat") - 1.4).abs() < 0.1);
        assert!((gbs.err_lon.expect("err_lon") - 1.3).abs() < 0.1);
        assert!((gbs.err_alt.expect("err_alt") - 3.1).abs() < 0.1);
        assert_eq!(gbs.svid, Some(3));
        assert!(gbs.prob.is_none());
        assert!((gbs.bias.expect("bias") - (-21.4)).abs() < 0.1);
        assert!((gbs.stddev.expect("stddev") - 3.8).abs() < 0.1);
    }

    #[test]
    fn gbs_multi_constellation_pynmeagps() {
        let frame = parse_frame("$GNGBS,103607.00,15.1,24.2,31.0,,,,,,*6F")
            .expect("valid pynmeagps GNGBS frame");
        let gbs = Gbs::parse(&frame.fields).expect("parse GBS");
        assert_eq!(gbs.time, Some("103607.00".to_string()));
        assert!((gbs.err_lat.expect("err_lat") - 15.1).abs() < 0.1);
        assert!((gbs.err_lon.expect("err_lon") - 24.2).abs() < 0.1);
    }

    #[test]
    fn gbs_partial_gpsd() {
        let frame = parse_frame("$GPGBS,194907.00,3.0,1.9,4.2,,,,*4E").expect("valid");
        let gbs = Gbs::parse(&frame.fields).expect("parse GBS");
        assert_eq!(gbs.time, Some("194907.00".to_string()));
        assert!((gbs.err_lat.expect("err_lat") - 3.0).abs() < 0.1);
        assert!((gbs.err_lon.expect("err_lon") - 1.9).abs() < 0.1);
        assert!((gbs.err_alt.expect("err_alt") - 4.2).abs() < 0.1);
        assert!(gbs.svid.is_none());
    }

    #[test]
    fn gbs_pynmeagps() {
        let frame =
            parse_frame("$GPGBS,235503.00,1.6,1.4,3.2,,,,,,*40").expect("valid pynmeagps GBS");
        let gbs = Gbs::parse(&frame.fields).expect("parse GBS");
        assert_eq!(gbs.time, Some("235503.00".to_string()));
        assert!((gbs.err_lat.expect("err_lat") - 1.6).abs() < 0.1);
        assert!((gbs.err_alt.expect("err_alt") - 3.2).abs() < 0.1);
    }
    #[test]
    fn gbs_encode_roundtrip() {
        let gbs = Gbs {
            time: Some("194907.00".to_string()),
            err_lat: Some(3.0),
            err_lon: Some(1.9),
            err_alt: Some(4.2),
            svid: Some(12),
            prob: Some(0.5),
            bias: Some(1.1),
            stddev: Some(0.8),
        };
        let sentence = gbs.to_sentence("GP").expect("encode");
        assert!(sentence.starts_with("$GPGBS,"));
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let gbs2 = Gbs::parse(&frame.fields).expect("re-parse GBS");
        assert_eq!(gbs, gbs2);
    }
}
