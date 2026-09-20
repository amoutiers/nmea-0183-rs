use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GLL — Geographic Position (Latitude/Longitude).
///
/// Wire: `lat,NS,lon,EW,time,status,mode`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gll {
    /// Latitude in NMEA format (DDMM.MMMM).
    pub lat: Option<f64>,
    /// Latitude hemisphere: 'N' or 'S'.
    pub ns: Option<char>,
    /// Longitude in NMEA format (DDDMM.MMMM).
    pub lon: Option<f64>,
    /// Longitude hemisphere: 'E' or 'W'.
    pub ew: Option<char>,
    /// UTC time of fix (HHMMSS format).
    pub time: Option<String>,
    /// Status: 'A' = valid, 'V' = invalid.
    pub status: Option<char>,
    /// Mode indicator (NMEA 2.3+): 'A'=autonomous, 'D'=differential.
    pub mode: Option<char>,
}

impl Gll {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            lat: r.f64(),
            ns: r.char(),
            lon: r.f64(),
            ew: r.char(),
            time: r.string(),
            status: r.char(),
            mode: r.char(),
        })
    }
}

impl NmeaEncodable for Gll {
    const SENTENCE_TYPE: &str = "GLL";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.lat(self.lat);
        w.char(self.ns);
        w.lon(self.lon);
        w.char(self.ew);
        w.string(self.time.as_deref());
        w.char(self.status);
        w.char(self.mode);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn gll_empty() {
        // SignalK fixture: all fields empty
        let frame = parse_frame("$GPGLL,,,,,,,*7C").expect("valid empty GLL frame");
        let gll = Gll::parse(&frame.fields).expect("parse empty GLL");
        assert!(gll.lat.is_none());
        assert!(gll.ns.is_none());
        assert!(gll.lon.is_none());
        assert!(gll.status.is_none());
    }

    #[test]
    fn gll_full_signalk() {
        let frame =
            parse_frame("$GPGLL,5958.613,N,02325.928,E,121022,A,D*40").expect("valid GLL frame");
        let gll = Gll::parse(&frame.fields).expect("parse GLL");
        assert!((gll.lat.expect("lat") - 5958.613).abs() < 0.001);
        assert_eq!(gll.ns, Some('N'));
        assert!((gll.lon.expect("lon") - 2325.928).abs() < 0.001);
        assert_eq!(gll.ew, Some('E'));
        assert_eq!(gll.status, Some('A'));
        assert_eq!(gll.mode, Some('D'));
    }

    #[test]
    fn gll_multi_constellation_pynmeagps() {
        // pynmeagps fixture: GN talker
        let frame = parse_frame("$GNGLL,5327.03942,N,00214.42462,W,103607.00,A,A*68")
            .expect("valid GN GLL frame");
        let gll = Gll::parse(&frame.fields).expect("parse GN GLL");
        assert!((gll.lat.expect("lat") - 5327.03942).abs() < 0.00001);
        assert_eq!(gll.ns, Some('N'));
        assert!((gll.lon.expect("lon") - 214.42462).abs() < 0.00001);
        assert_eq!(gll.ew, Some('W'));
        assert_eq!(gll.time, Some("103607.00".to_string()));
        assert_eq!(gll.status, Some('A'));
        assert_eq!(gll.mode, Some('A'));
    }

    #[test]
    fn gll_encode_pads_longitude_degrees() {
        let frame = parse_frame("$GPGLL,5958.613,N,02325.928,E,120000,A*29")
            .expect("valid GLL");
        let gll = Gll::parse(&frame.fields).expect("parse GLL");
        let sentence = gll.to_sentence("GP").expect("encode");
        // Longitude 023°25.928' must keep its 3-digit (5-char integer) degree field.
        assert!(sentence.contains(",02325.928,"), "leading zero lost: {sentence}");
    }

    #[test]
    fn gll_encode_roundtrip() {
        let gll = Gll {
            lat: Some(4807.038),
            ns: Some('N'),
            lon: Some(1131.0),
            ew: Some('E'),
            time: Some("120000".to_string()),
            status: Some('A'),
            mode: Some('A'),
        };
        let sentence = gll.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse GLL");
        let gll2 = Gll::parse(&frame.fields).expect("parse roundtrip GLL");
        assert_eq!(gll, gll2);
    }
}
