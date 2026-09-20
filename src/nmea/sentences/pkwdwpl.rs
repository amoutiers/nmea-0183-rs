use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PKWDWPL — Kenwood Waypoint Location.
///
/// Proprietary Kenwood sentence.
/// Wire: `$PKWDWPL,time,validity,lat,ns,lon,ew,speed,course,date,altitude,wpt_name,table_symbol`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PKWDWPL"`.
/// Encode with `to_sentence("")`.
#[derive(Debug, Clone, PartialEq)]
pub struct Pkwdwpl {
    /// UTC time (hhmmss).
    pub time: Option<String>,
    /// Validity (A=valid, V=invalid).
    pub validity: Option<char>,
    /// Latitude in DDDMM.MMM format.
    pub lat: Option<f64>,
    /// North/South indicator.
    pub ns: Option<char>,
    /// Longitude in DDDMM.MMM format.
    pub lon: Option<f64>,
    /// East/West indicator.
    pub ew: Option<char>,
    /// Speed over ground in knots.
    pub speed: Option<f32>,
    /// Course over ground in degrees.
    pub course: Option<f32>,
    /// UTC date (ddmmyy).
    pub date: Option<String>,
    /// Altitude in metres.
    pub altitude: Option<f32>,
    /// Waypoint name.
    pub wpt_name: Option<String>,
    /// Table/symbol identifier.
    pub table_symbol: Option<String>,
}

impl Pkwdwpl {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let time = r.string();
        let validity = r.char();
        let lat = r.f64();
        let ns = r.char();
        let lon = r.f64();
        let ew = r.char();
        let speed = r.f32();
        let course = r.f32();
        let date = r.string();
        let altitude = r.f32();
        let wpt_name = r.string();
        let table_symbol = r.string();
        Some(Self {
            time,
            validity,
            lat,
            ns,
            lon,
            ew,
            speed,
            course,
            date,
            altitude,
            wpt_name,
            table_symbol,
        })
    }
}

impl NmeaEncodable for Pkwdwpl {
    const SENTENCE_TYPE: &str = "PKWDWPL";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.char(self.validity);
        w.lat(self.lat);
        w.char(self.ns);
        w.lon(self.lon);
        w.char(self.ew);
        w.f32(self.speed);
        w.f32(self.course);
        w.string(self.date.as_deref());
        w.f32(self.altitude);
        w.string(self.wpt_name.as_deref());
        w.string(self.table_symbol.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn pkwdwpl_empty() {
        let s = Pkwdwpl {
            time: None,
            validity: None,
            lat: None,
            ns: None,
            lon: None,
            ew: None,
            speed: None,
            course: None,
            date: None,
            altitude: None,
            wpt_name: None,
            table_symbol: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Pkwdwpl::parse(&f.fields).expect("parse");
        assert!(p.time.is_none());
        assert!(p.wpt_name.is_none());
        assert!(p.table_symbol.is_none());
    }

    #[test]
    fn pkwdwpl_encode_roundtrip() {
        let original = Pkwdwpl {
            time: Some("150803".to_string()),
            validity: Some('A'),
            lat: Some(4237.14),
            ns: Some('N'),
            lon: Some(7120.83),
            ew: Some('W'),
            speed: Some(173.8),
            course: Some(231.8),
            date: Some("190316".to_string()),
            altitude: Some(1120.0),
            wpt_name: Some("test".to_string()),
            table_symbol: Some("/'".to_string()),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Pkwdwpl::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn pkwdwpl_values() {
        let f = parse_frame("$PKWDWPL,150803,A,4237.14,N,07120.83,W,173.8,231.8,190316,1120,test,/'*39")
            .expect("valid PKWDWPL");
        let p = Pkwdwpl::parse(&f.fields).expect("parse PKWDWPL");
        assert_eq!(p.time, Some("150803".to_string()));
        assert_eq!(p.validity, Some('A'));
        assert!((p.lat.expect("lat") - 4237.14).abs() < 0.001);
        assert_eq!(p.ns, Some('N'));
        assert!((p.lon.expect("lon") - 7120.83).abs() < 0.001);
        assert_eq!(p.ew, Some('W'));
        assert!((p.speed.expect("speed") - 173.8).abs() < 0.01);
        assert!((p.course.expect("course") - 231.8).abs() < 0.01);
        assert_eq!(p.date, Some("190316".to_string()));
        assert!((p.altitude.expect("alt") - 1120.0).abs() < 0.01);
        assert_eq!(p.wpt_name, Some("test".to_string()));
        assert_eq!(p.table_symbol, Some("/'".to_string()));
    }
}
