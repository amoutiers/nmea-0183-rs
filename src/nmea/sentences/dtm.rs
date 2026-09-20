use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// DTM — Datum Reference.
///
/// Wire: `datum,sub_datum,lat_offset,ns,lon_offset,ew,alt_offset,ref_datum`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dtm {
    /// Local datum code (e.g., "W84" for WGS-84).
    pub datum: Option<String>,
    /// Local datum subdivision code.
    pub sub_datum: Option<String>,
    /// Latitude offset in minutes.
    pub lat_offset: Option<f32>,
    /// North/South indicator for latitude offset ('N' or 'S').
    pub ns: Option<char>,
    /// Longitude offset in minutes.
    pub lon_offset: Option<f32>,
    /// East/West indicator for longitude offset ('E' or 'W').
    pub ew: Option<char>,
    /// Altitude offset in meters.
    pub alt_offset: Option<f32>,
    /// Reference datum code (e.g., "W84" for WGS-84).
    pub ref_datum: Option<String>,
}

impl Dtm {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            datum: r.string(),
            sub_datum: r.string(),
            lat_offset: r.f32(),
            ns: r.char(),
            lon_offset: r.f32(),
            ew: r.char(),
            alt_offset: r.f32(),
            ref_datum: r.string(),
        }
    }
}

impl NmeaEncodable for Dtm {
    const SENTENCE_TYPE: &str = "DTM";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.datum.as_deref());
        w.string(self.sub_datum.as_deref());
        w.f32(self.lat_offset);
        w.char(self.ns);
        w.f32(self.lon_offset);
        w.char(self.ew);
        w.f32(self.alt_offset);
        w.string(self.ref_datum.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn dtm_empty() {
        let f = Dtm {
            datum: None,
            sub_datum: None,
            lat_offset: None,
            ns: None,
            lon_offset: None,
            ew: None,
            alt_offset: None,
            ref_datum: None,
        }
        .to_sentence("GP").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let d = Dtm::parse(&frame.fields);
        assert!(d.datum.is_none());
        assert!(d.ref_datum.is_none());
    }

    #[test]
    fn dtm_encode_roundtrip() {
        let original = Dtm {
            datum: Some("W84".to_string()),
            sub_datum: None,
            lat_offset: Some(0.0),
            ns: Some('N'),
            lon_offset: Some(0.0),
            ew: Some('E'),
            alt_offset: Some(0.0),
            ref_datum: Some("W84".to_string()),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Dtm::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn dtm_pynmeagps() {
        let frame =
            parse_frame("$GPDTM,W84,,0.0,N,0.0,E,0.0,W84*6F").expect("valid pynmeagps DTM frame");
        let dtm = Dtm::parse(&frame.fields);
        assert_eq!(dtm.datum, Some("W84".to_string()));
        assert!(dtm.sub_datum.is_none());
        assert!((dtm.lat_offset.expect("lat_offset") - 0.0).abs() < 0.001);
        assert_eq!(dtm.ns, Some('N'));
        assert!((dtm.lon_offset.expect("lon_offset") - 0.0).abs() < 0.001);
        assert_eq!(dtm.ew, Some('E'));
        assert!((dtm.alt_offset.expect("alt_offset") - 0.0).abs() < 0.001);
        assert_eq!(dtm.ref_datum, Some("W84".to_string()));
    }

    #[test]
    fn dtm_subdivision_gonmea() {
        let frame = parse_frame("$GPDTM,W84,X,00.1200,S,12.0000,W,100,W84*27")
            .expect("valid go-nmea DTM frame");
        let dtm = Dtm::parse(&frame.fields);
        assert_eq!(dtm.datum, Some("W84".to_string()));
        assert_eq!(dtm.sub_datum, Some("X".to_string()));
        assert!((dtm.lat_offset.expect("lat_offset") - 0.12).abs() < 0.001);
        assert_eq!(dtm.ns, Some('S'));
        assert!((dtm.lon_offset.expect("lon_offset") - 12.0).abs() < 0.001);
        assert_eq!(dtm.ew, Some('W'));
        assert!((dtm.alt_offset.expect("alt_offset") - 100.0).abs() < 0.1);
        assert_eq!(dtm.ref_datum, Some("W84".to_string()));
    }
}
