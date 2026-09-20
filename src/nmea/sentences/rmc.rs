use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// RMC — Recommended Minimum Navigation Information.
///
/// Wire: `time,status,lat,NS,lon,EW,sog,cog,date,magvar,magvarEW,mode,navstatus`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rmc {
    /// UTC time of fix (HHMMSS.SSS format).
    pub time: Option<String>,
    /// Status: 'A' = active/valid, 'V' = void/invalid.
    pub status: Option<char>,
    /// Latitude in NMEA format (DDMM.MMMM).
    pub lat: Option<f64>,
    /// Latitude hemisphere: 'N' or 'S'.
    pub ns: Option<char>,
    /// Longitude in NMEA format (DDDMM.MMMM).
    pub lon: Option<f64>,
    /// Longitude hemisphere: 'E' or 'W'.
    pub ew: Option<char>,
    /// Speed over ground in knots.
    pub sog: Option<f32>,
    /// Course over ground in degrees true.
    pub cog: Option<f32>,
    /// Date (DDMMYY format).
    pub date: Option<String>,
    /// Magnetic variation in degrees.
    pub mag_var: Option<f32>,
    /// Magnetic variation direction: 'E' or 'W'.
    pub mag_var_ew: Option<char>,
    /// Positioning mode indicator (NMEA 2.3+): 'A'=autonomous, 'D'=differential, etc.
    pub pos_mode: Option<char>,
    /// Navigational status (NMEA 4.1+): 'S'=safe, 'C'=caution, 'U'=unsafe, 'V'=not valid.
    pub nav_status: Option<char>,
}

impl Rmc {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            time: r.string(),
            status: r.char(),
            lat: r.f64(),
            ns: r.char(),
            lon: r.f64(),
            ew: r.char(),
            sog: r.f32(),
            cog: r.f32(),
            date: r.string(),
            mag_var: r.f32(),
            mag_var_ew: r.char(),
            pos_mode: r.char(),
            nav_status: r.char(),
        })
    }
}

impl NmeaEncodable for Rmc {
    const SENTENCE_TYPE: &str = "RMC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.char(self.status);
        w.lat(self.lat);
        w.char(self.ns);
        w.lon(self.lon);
        w.char(self.ew);
        w.f32(self.sog);
        w.f32(self.cog);
        w.string(self.date.as_deref());
        w.f32(self.mag_var);
        w.char(self.mag_var_ew);
        w.char(self.pos_mode);
        w.char(self.nav_status);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn rmc_empty() {
        let f = parse_frame("$IIRMC,,,,,,,,,,,,,*70").expect("valid");
        let r = Rmc::parse(&f.fields).expect("parse");
        assert!(r.time.is_none());
        assert!(r.lat.is_none());
        assert!(r.lon.is_none());
        assert!(r.sog.is_none());
        assert!(r.cog.is_none());
    }

    #[test]
    fn rmc_full_signalk() {
        let frame =
            parse_frame("$GPRMC,085412.000,A,5222.3198,N,00454.5784,E,0.58,251.34,030414,,,A*65")
                .expect("valid RMC frame");
        let rmc = Rmc::parse(&frame.fields).expect("parse RMC");
        assert_eq!(rmc.time, Some("085412.000".to_string()));
        assert_eq!(rmc.status, Some('A'));
        assert!((rmc.lat.expect("lat") - 5222.3198).abs() < 0.001);
        assert_eq!(rmc.ns, Some('N'));
        assert!((rmc.lon.expect("lon") - 454.5784).abs() < 0.001);
        assert_eq!(rmc.ew, Some('E'));
        assert!((rmc.sog.expect("sog") - 0.58).abs() < 0.01);
        assert!((rmc.cog.expect("cog") - 251.34).abs() < 0.01);
        assert_eq!(rmc.date, Some("030414".to_string()));
        assert_eq!(rmc.pos_mode, Some('A'));
    }

    #[test]
    fn rmc_missing_speed_course_signalk() {
        // SignalK fixture: missing SOG/COG, has magnetic variation
        let frame = parse_frame("$GPRMC,085412.000,A,5222.3198,N,00454.5784,E,,,030414,12,E*42")
            .expect("valid RMC missing speed");
        let rmc = Rmc::parse(&frame.fields).expect("parse RMC");
        assert_eq!(rmc.status, Some('A'));
        assert!(rmc.sog.is_none());
        assert!(rmc.cog.is_none());
        assert!((rmc.mag_var.expect("mag_var") - 12.0).abs() < 0.01);
        assert_eq!(rmc.mag_var_ew, Some('E'));
    }

    #[test]
    fn rmc_multi_constellation_pynmeagps() {
        // pynmeagps fixture: GN talker (multi-constellation), has pos_mode V
        let frame =
            parse_frame("$GNRMC,103607.00,A,5327.03942,N,10214.42462,W,0.046,,060321,,,A,V*0E")
                .expect("valid GN RMC frame");
        let rmc = Rmc::parse(&frame.fields).expect("parse GN RMC");
        assert_eq!(rmc.time, Some("103607.00".to_string()));
        assert_eq!(rmc.status, Some('A'));
        assert!((rmc.lat.expect("lat") - 5327.03942).abs() < 0.00001);
        assert_eq!(rmc.ns, Some('N'));
        assert!((rmc.sog.expect("sog") - 0.046).abs() < 0.001);
        assert!(rmc.cog.is_none());
        assert_eq!(rmc.date, Some("060321".to_string()));
    }

    #[test]
    fn rmc_nav_status_4_1() {
        // NMEA 4.1 RMC field 12 (nav status). The 'V' must be parsed and re-encoded.
        let frame =
            parse_frame("$GNRMC,103607.00,A,5327.03942,N,10214.42462,W,0.046,,060321,,,A,V*0E")
                .expect("valid GN RMC frame");
        let rmc = Rmc::parse(&frame.fields).expect("parse RMC");
        assert_eq!(rmc.nav_status, Some('V'));

        let sentence = rmc.to_sentence("GN").expect("encode");
        let frame2 = parse_frame(sentence.trim()).expect("re-parse");
        let rmc2 = Rmc::parse(&frame2.fields).expect("re-parse");
        assert_eq!(rmc2.nav_status, Some('V'));
    }

    #[test]
    fn rmc_encode_roundtrip() {
        let rmc = Rmc {
            time: Some("120000.00".to_string()),
            status: Some('A'),
            lat: Some(4807.038),
            ns: Some('N'),
            lon: Some(1131.0),
            ew: Some('E'),
            sog: Some(5.5),
            cog: Some(54.7),
            date: Some("230394".to_string()),
            mag_var: Some(3.1),
            mag_var_ew: Some('E'),
            pos_mode: Some('A'),
            nav_status: None,
        };
        let sentence = rmc.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse RMC");
        let rmc2 = Rmc::parse(&frame.fields).expect("parse roundtrip RMC");
        assert_eq!(rmc, rmc2);
    }
}
