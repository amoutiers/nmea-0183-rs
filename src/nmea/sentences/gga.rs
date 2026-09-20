use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GGA — Global Positioning System Fix Data.
///
/// Wire: `time,lat,NS,lon,EW,quality,numSats,hdop,alt,altUnit,geoidSep,geoidUnit,dgpsAge,dgpsStation`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gga {
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
    /// Fix quality: 0=invalid, 1=GPS, 2=DGPS, 4=RTK, 5=float RTK.
    pub quality: Option<u8>,
    /// Number of satellites in use.
    pub num_sats: Option<u8>,
    /// Horizontal dilution of precision.
    pub hdop: Option<f32>,
    /// Altitude above mean sea level in meters.
    pub altitude: Option<f32>,
    /// Altitude unit (always 'M' for meters).
    pub alt_unit: Option<char>,
    /// Geoidal separation in meters.
    pub geoid_sep: Option<f32>,
    /// Geoidal separation unit (always 'M').
    pub geoid_unit: Option<char>,
    /// Age of DGPS data in seconds.
    pub dgps_age: Option<f32>,
    /// DGPS reference station ID.
    pub dgps_station: Option<String>,
}

impl Gga {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            time: r.string(),
            lat: r.f64(),
            ns: r.char(),
            lon: r.f64(),
            ew: r.char(),
            quality: r.u8(),
            num_sats: r.u8(),
            hdop: r.f32(),
            altitude: r.f32(),
            alt_unit: r.char(),
            geoid_sep: r.f32(),
            geoid_unit: r.char(),
            dgps_age: r.f32(),
            dgps_station: r.string(),
        }
    }
}

impl NmeaEncodable for Gga {
    const SENTENCE_TYPE: &str = "GGA";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.lat(self.lat);
        w.char(self.ns);
        w.lon(self.lon);
        w.char(self.ew);
        w.u8(self.quality);
        w.u8(self.num_sats);
        w.f32(self.hdop);
        w.f32(self.altitude);
        w.char(self.alt_unit);
        w.f32(self.geoid_sep);
        w.char(self.geoid_unit);
        w.f32(self.dgps_age);
        w.string(self.dgps_station.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn gga_empty() {
        // SignalK fixture: all fields empty
        let frame = parse_frame("$GPGGA,,,,,,,,,,,,,,*56").expect("valid empty GGA frame");
        let gga = Gga::parse(&frame.fields);
        assert!(gga.time.is_none());
        assert!(gga.lat.is_none());
        assert!(gga.quality.is_none());
        assert!(gga.altitude.is_none());
    }

    #[test]
    fn gga_full_signalk() {
        let frame = parse_frame("$GPGGA,172814.0,3723.46587704,N,12202.26957864,W,2,6,1.2,18.893,M,-25.669,M,2.0,0031*4F")
            .expect("valid GGA frame");
        let gga = Gga::parse(&frame.fields);
        assert_eq!(gga.time, Some("172814.0".to_string()));
        assert!((gga.lat.expect("lat") - 3723.46587704).abs() < 0.0001);
        assert_eq!(gga.ns, Some('N'));
        assert_eq!(gga.quality, Some(2));
        assert_eq!(gga.num_sats, Some(6));
        assert!((gga.hdop.expect("hdop") - 1.2).abs() < 0.01);
        assert!((gga.altitude.expect("alt") - 18.893).abs() < 0.001);
    }

    #[test]
    fn gga_multi_constellation_pynmeagps() {
        // pynmeagps fixture: GN talker
        let frame =
            parse_frame("$GNGGA,103607.00,5327.03942,N,00214.42462,W,1,06,5.88,56.0,M,48.5,M,,*64")
                .expect("valid GN GGA frame");
        let gga = Gga::parse(&frame.fields);
        assert_eq!(gga.time, Some("103607.00".to_string()));
        assert_eq!(gga.quality, Some(1));
        assert_eq!(gga.num_sats, Some(6));
        assert!((gga.hdop.expect("hdop") - 5.88).abs() < 0.01);
        assert!((gga.altitude.expect("alt") - 56.0).abs() < 0.1);
        assert!((gga.geoid_sep.expect("geoid") - 48.5).abs() < 0.1);
    }

    #[test]
    fn gga_encode_roundtrip() {
        let gga = Gga {
            time: Some("120000.00".to_string()),
            lat: Some(4807.038),
            ns: Some('N'),
            lon: Some(1131.0),
            ew: Some('E'),
            quality: Some(1),
            num_sats: Some(8),
            hdop: Some(0.9),
            altitude: Some(545.4),
            alt_unit: Some('M'),
            geoid_sep: Some(46.9),
            geoid_unit: Some('M'),
            dgps_age: None,
            dgps_station: None,
        };
        let sentence = gga.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse GGA");
        let gga2 = Gga::parse(&frame.fields);
        assert_eq!(gga, gga2);
    }
}
