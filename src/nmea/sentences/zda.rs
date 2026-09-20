use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ZDA — Time and Date.
///
/// Wire: `time,day,month,year,ltzh,ltzm`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Zda {
    /// UTC time (hhmmss.ss format).
    pub time: Option<String>,
    /// Day of month (01-31).
    pub day: Option<u8>,
    /// Month (01-12).
    pub month: Option<u8>,
    /// Four-digit year.
    pub year: Option<u32>,
    /// Local zone hours offset from UTC (-13 to +13).
    pub local_hour_offset: Option<i8>,
    /// Local zone minutes offset (00-59).
    pub local_min_offset: Option<u8>,
}

impl Zda {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            time: r.string(),
            day: r.u8(),
            month: r.u8(),
            year: r.u32(),
            local_hour_offset: r.i8(),
            local_min_offset: r.u8(),
        })
    }
}

impl NmeaEncodable for Zda {
    const SENTENCE_TYPE: &str = "ZDA";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.u8(self.day);
        w.u8(self.month);
        w.u32(self.year);
        w.i8(self.local_hour_offset);
        w.u8(self.local_min_offset);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn zda_empty() {
        let f = parse_frame("$GPZDA,,,,,,*48").expect("valid");
        let z = Zda::parse(&f.fields).expect("parse");
        assert!(z.time.is_none());
        assert!(z.day.is_none());
        assert!(z.month.is_none());
        assert!(z.year.is_none());
        assert!(z.local_hour_offset.is_none());
        assert!(z.local_min_offset.is_none());
    }

    #[test]
    fn zda_encode_roundtrip() {
        let zda = Zda {
            time: Some("160012.71".to_string()),
            day: Some(11),
            month: Some(3),
            year: Some(2004),
            local_hour_offset: Some(-1),
            local_min_offset: Some(0),
        };
        let sentence = zda.to_sentence("GP").expect("encode");
        assert!(sentence.starts_with("$GPZDA,"));
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let zda2 = Zda::parse(&frame.fields).expect("re-parse ZDA");
        assert_eq!(zda.time, zda2.time);
        assert_eq!(zda.day, zda2.day);
        assert_eq!(zda.month, zda2.month);
        assert_eq!(zda.year, zda2.year);
    }

    #[test]
    fn zda_full_gonmea() {
        let f =
            parse_frame("$GPZDA,172809.456,12,07,1996,00,00*57").expect("valid ZDA from go-nmea");
        let z = Zda::parse(&f.fields).expect("parse ZDA");
        assert_eq!(z.time, Some("172809.456".to_string()));
        assert_eq!(z.day, Some(12));
        assert_eq!(z.month, Some(7));
        assert_eq!(z.year, Some(1996));
        assert_eq!(z.local_hour_offset, Some(0));
        assert_eq!(z.local_min_offset, Some(0));
    }

    #[test]
    fn zda_pynmeagps() {
        let f =
            parse_frame("$GNZDA,103607.00,06,03,2021,00,00*7F").expect("valid ZDA from pynmeagps");
        let z = Zda::parse(&f.fields).expect("parse ZDA");
        assert_eq!(z.time, Some("103607.00".to_string()));
        assert_eq!(z.day, Some(6));
        assert_eq!(z.month, Some(3));
        assert_eq!(z.year, Some(2021));
    }
}
