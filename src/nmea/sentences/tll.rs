use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// TLL — Target Latitude and Longitude.
///
/// Wire: `target_num,lat,NS,lon,EW,name,time,status,ref_target`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Tll {
    /// Target number (00–99).
    pub target_num: Option<u8>,
    /// Latitude in NMEA ddmm.mmm format.
    pub lat: Option<f64>,
    /// North/South indicator.
    pub ns: Option<char>,
    /// Longitude in NMEA dddmm.mmm format.
    pub lon: Option<f64>,
    /// East/West indicator.
    pub ew: Option<char>,
    /// Target name.
    pub name: Option<String>,
    /// UTC time of data.
    pub time: Option<String>,
    /// Target status ('L' = lost, 'Q' = acquiring, 'T' = tracking).
    pub status: Option<char>,
    /// Reference target ('R' = reference, empty = not reference).
    pub ref_target: Option<char>,
}

impl Tll {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let target_num = r.u8();
        let lat = r.f64();
        let ns = r.char();
        let lon = r.f64();
        let ew = r.char();
        let name = r.string();
        let time = r.string();
        let status = r.char();
        let ref_target = r.char();
        Self {
            target_num,
            lat,
            ns,
            lon,
            ew,
            name,
            time,
            status,
            ref_target,
        }
    }
}

impl NmeaEncodable for Tll {
    const SENTENCE_TYPE: &str = "TLL";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.target_num);
        w.lat(self.lat);
        w.char(self.ns);
        w.lon(self.lon);
        w.char(self.ew);
        w.string(self.name.as_deref());
        w.string(self.time.as_deref());
        w.char(self.status);
        w.char(self.ref_target);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn tll_empty() {
        let s = Tll {
            target_num: None,
            lat: None,
            ns: None,
            lon: None,
            ew: None,
            name: None,
            time: None,
            status: None,
            ref_target: None,
        }
        .to_sentence("RA").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let t = Tll::parse(&f.fields);
        assert!(t.target_num.is_none());
        assert!(t.lat.is_none());
    }

    #[test]
    fn tll_encode_roundtrip() {
        let original = Tll {
            target_num: Some(1),
            lat: Some(3647.422),
            ns: Some('N'),
            lon: Some(1432.592),
            ew: Some('E'),
            name: Some("TGT01".to_string()),
            time: Some("120000".to_string()),
            status: Some('T'),
            ref_target: None,
        };
        let sentence = original.to_sentence("RA").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Tll::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn tll_full_gonmea() {
        let frame =
            parse_frame("$RATLL,1,3646.54266,N,00235.37778,W,test,020915,L,R*78").expect("valid");
        let t = Tll::parse(&frame.fields);
        assert_eq!(t.target_num, Some(1));
        assert!((t.lat.expect("lat") - 3646.54266).abs() < 0.00001);
        assert_eq!(t.ns, Some('N'));
        assert!((t.lon.expect("lon") - 235.37778).abs() < 0.00001);
        assert_eq!(t.ew, Some('W'));
        assert_eq!(t.name.as_deref(), Some("test"));
        assert_eq!(t.time.as_deref(), Some("020915"));
        assert_eq!(t.status, Some('L'));
        assert_eq!(t.ref_target, Some('R'));
    }

    #[test]
    fn tll_ratll_gonmea() {
        let frame = parse_frame("$RATLL,,3647.422,N,01432.592,E,,,,*58").expect("valid");
        let t = Tll::parse(&frame.fields);
        assert!(t.target_num.is_none());
        assert!((t.lat.expect("lat") - 3647.422).abs() < 0.001);
        assert_eq!(t.ns, Some('N'));
        assert!((t.lon.expect("lon") - 1432.592).abs() < 0.001);
        assert_eq!(t.ew, Some('E'));
        assert!(t.name.is_none());
        assert!(t.status.is_none());
    }
}
