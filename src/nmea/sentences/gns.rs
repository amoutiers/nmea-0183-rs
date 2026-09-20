use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GNS — GNSS Fix Data (multi-constellation).
///
/// Wire: `time,lat,NS,lon,EW,mode,numSats,hdop,alt,geoidSep,dgpsAge,dgpsStation,navStatus`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gns {
    /// UTC time of fix (HHMMSS.SS format).
    pub time: Option<String>,
    /// Latitude in NMEA format (DDMM.MMMM).
    pub lat: Option<f64>,
    /// Latitude hemisphere: 'N' or 'S'.
    pub ns: Option<char>,
    /// Longitude in NMEA format (DDDMM.MMMM).
    pub lon: Option<f64>,
    /// Longitude hemisphere: 'E' or 'W'.
    pub ew: Option<char>,
    /// Mode indicator (multi-char: one per constellation, e.g. "ANN").
    pub mode: Option<String>,
    /// Number of satellites in use.
    pub num_sats: Option<u8>,
    /// Horizontal dilution of precision.
    pub hdop: Option<f32>,
    /// Altitude above mean sea level in meters.
    pub altitude: Option<f32>,
    /// Geoidal separation in meters.
    pub geoid_sep: Option<f32>,
    /// Age of DGPS data in seconds.
    pub dgps_age: Option<f32>,
    /// DGPS reference station ID.
    pub dgps_station: Option<String>,
    /// Navigation status indicator.
    pub nav_status: Option<char>,
}

impl Gns {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            time: r.string(),
            lat: r.f64(),
            ns: r.char(),
            lon: r.f64(),
            ew: r.char(),
            mode: r.string(),
            num_sats: r.u8(),
            hdop: r.f32(),
            altitude: r.f32(),
            geoid_sep: r.f32(),
            dgps_age: r.f32(),
            dgps_station: r.string(),
            nav_status: r.char(),
        })
    }
}

impl NmeaEncodable for Gns {
    const SENTENCE_TYPE: &str = "GNS";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.lat(self.lat);
        w.char(self.ns);
        w.lon(self.lon);
        w.char(self.ew);
        w.string(self.mode.as_deref());
        w.u8(self.num_sats);
        w.f32(self.hdop);
        w.f32(self.altitude);
        w.f32(self.geoid_sep);
        w.f32(self.dgps_age);
        w.string(self.dgps_station.as_deref());
        w.char(self.nav_status);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn gns_empty() {
        // SignalK fixture: all fields empty except nav_status
        let frame = parse_frame("$GPGNS,,,,,,,,,,,,,S*32").expect("valid empty GNS frame");
        let gns = Gns::parse(&frame.fields).expect("parse empty GNS");
        assert!(gns.time.is_none());
        assert!(gns.lat.is_none());
        assert!(gns.mode.is_none());
        assert!(gns.num_sats.is_none());
        assert_eq!(gns.nav_status, Some('S'));
    }

    #[test]
    fn gns_full_signalk() {
        let frame =
            parse_frame("$GPGNS,111648.00,0235.0379,S,04422.1450,W,ANN,12,0.8,8.5,-22.3,,,S*5D")
                .expect("valid GNS frame");
        let gns = Gns::parse(&frame.fields).expect("parse GNS");
        assert_eq!(gns.time, Some("111648.00".to_string()));
        assert!((gns.lat.expect("lat") - 235.0379).abs() < 0.001);
        assert_eq!(gns.ns, Some('S'));
        assert_eq!(gns.mode, Some("ANN".to_string()));
        assert_eq!(gns.num_sats, Some(12));
        assert!((gns.hdop.expect("hdop") - 0.8).abs() < 0.01);
        assert_eq!(gns.nav_status, Some('S'));
    }

    #[test]
    fn gns_multi_constellation_pynmeagps() {
        // pynmeagps fixture: GN talker, mode "AANN" (4-constellation)
        let frame = parse_frame(
            "$GNGNS,103607.00,5327.03942,N,00214.42462,W,AANN,06,5.88,56.0,48.5,,,V*34",
        )
        .expect("valid GN GNS frame");
        let gns = Gns::parse(&frame.fields).expect("parse GN GNS");
        assert_eq!(gns.mode, Some("AANN".to_string()));
        assert_eq!(gns.num_sats, Some(6));
        assert!((gns.hdop.expect("hdop") - 5.88).abs() < 0.01);
        assert!((gns.altitude.expect("alt") - 56.0).abs() < 0.1);
        assert_eq!(gns.nav_status, Some('V'));
    }

    #[test]
    fn gns_encode_roundtrip() {
        let gns = Gns {
            time: Some("120000.00".to_string()),
            lat: Some(4807.038),
            ns: Some('N'),
            lon: Some(1131.0),
            ew: Some('E'),
            mode: Some("AAN".to_string()),
            num_sats: Some(10),
            hdop: Some(0.9),
            altitude: Some(100.5),
            geoid_sep: Some(-23.0),
            dgps_age: None,
            dgps_station: None,
            nav_status: Some('S'),
        };
        let sentence = gns.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse GNS");
        let gns2 = Gns::parse(&frame.fields).expect("parse roundtrip GNS");
        assert_eq!(gns, gns2);
    }
}
