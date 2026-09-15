use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PKLDS — Kenwood GPS Data (Long format).
///
/// Proprietary Kenwood sentence.
/// Wire: `$PKLDS,time,validity,lat,ns,lon,ew,speed,course,date,variation,var_ew,fleet,unit_id,status,extension`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PKLDS"`.
#[derive(Debug, Clone, PartialEq)]
pub struct Pklds {
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
    /// Magnetic variation in degrees.
    pub variation: Option<f32>,
    /// Magnetic variation direction with prefix.
    pub var_ew: Option<String>,
    /// Fleet identifier.
    pub fleet: Option<String>,
    /// Unit identifier.
    pub unit_id: Option<String>,
    /// Status.
    pub status: Option<String>,
    /// Extension data.
    pub extension: Option<String>,
}

impl Pklds {
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
        let variation = r.f32();
        let var_ew = r.string();
        let fleet = r.string();
        let unit_id = r.string();
        let status = r.string();
        let extension = r.string();
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
            variation,
            var_ew,
            fleet,
            unit_id,
            status,
            extension,
        })
    }
}

impl NmeaEncodable for Pklds {
    const SENTENCE_TYPE: &str = "PKLDS";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.char(self.validity);
        w.f64(self.lat);
        w.char(self.ns);
        w.f64(self.lon);
        w.char(self.ew);
        w.f32(self.speed);
        w.f32(self.course);
        w.string(self.date.as_deref());
        w.f32(self.variation);
        w.string(self.var_ew.as_deref());
        w.string(self.fleet.as_deref());
        w.string(self.unit_id.as_deref());
        w.string(self.status.as_deref());
        w.string(self.extension.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn pklds_empty() {
        let s = Pklds {
            time: None,
            validity: None,
            lat: None,
            ns: None,
            lon: None,
            ew: None,
            speed: None,
            course: None,
            date: None,
            variation: None,
            var_ew: None,
            fleet: None,
            unit_id: None,
            status: None,
            extension: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Pklds::parse(&f.fields).expect("parse");
        assert!(p.time.is_none());
        assert!(p.validity.is_none());
        assert!(p.extension.is_none());
    }

    #[test]
    fn pklds_encode_roundtrip() {
        let original = Pklds {
            time: Some("220516".to_string()),
            validity: Some('A'),
            lat: Some(5133.82),
            ns: Some('N'),
            lon: Some(42.24),
            ew: Some('W'),
            speed: Some(173.8),
            course: Some(231.8),
            date: Some("130694".to_string()),
            variation: Some(4.2),
            var_ew: Some("W00".to_string()),
            fleet: Some("100".to_string()),
            unit_id: Some("2000".to_string()),
            status: Some("15".to_string()),
            extension: Some("00".to_string()),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Pklds::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn pklds_east_variation_gonmea() {
        let f = parse_frame(
            "$PKLDS,220516,A,5133.82,N,00042.24,W,173.8,231.8,130694,004.2,E00,100,2000,15,00,*72",
        )
        .expect("valid PKLDS east variation");
        let p = Pklds::parse(&f.fields).expect("parse PKLDS");
        assert_eq!(p.variation, Some(4.2));
        assert_eq!(p.var_ew.as_deref(), Some("E00"));
    }

    #[test]
    fn pklds_kenwood_gonmea() {
        let f = parse_frame(
            "$PKLDS,220516,A,5133.82,N,00042.24,W,173.8,231.8,130694,004.2,W00,100,2000,15,00,*60",
        )
        .expect("valid PKLDS");
        let p = Pklds::parse(&f.fields).expect("parse PKLDS");
        assert_eq!(p.time, Some("220516".to_string()));
        assert_eq!(p.validity, Some('A'));
        assert!((p.lat.expect("lat") - 5133.82).abs() < 0.001);
        assert_eq!(p.ns, Some('N'));
        assert_eq!(p.fleet, Some("100".to_string()));
        assert_eq!(p.unit_id, Some("2000".to_string()));
        assert_eq!(p.status, Some("15".to_string()));
        assert_eq!(p.extension, Some("00".to_string()));
    }
}
