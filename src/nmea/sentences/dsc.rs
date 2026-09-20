use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// DSC — Digital Selective Calling.
///
/// Wire: `format_specifier,address,category,cmd1,cmd2,position,time_or_tel,mmsi,distress_cause,ack,expansion`
/// Field layout follows the NMEA 0183 definitions catalogued by
/// [pynmeagps](https://github.com/semuconsulting/pynmeagps/blob/master/src/pynmeagps/nmeatypes_get.py).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dsc {
    /// Format specifier (e.g. "12" for distress).
    pub format_specifier: Option<String>,
    /// DSC address.
    pub address: Option<String>,
    /// Category code.
    pub category: Option<String>,
    /// First command/nature of distress.
    pub cmd1: Option<String>,
    /// Second command.
    pub cmd2: Option<String>,
    /// Position (lat/lon encoded).
    pub position: Option<String>,
    /// Time or telephone number.
    pub time_or_tel: Option<String>,
    /// MMSI of transmitter.
    pub mmsi: Option<String>,
    /// Distress cause.
    pub distress_cause: Option<String>,
    /// Acknowledgement type.
    pub ack: Option<char>,
    /// Expansion indicator.
    pub expansion: Option<char>,
}

impl Dsc {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let format_specifier = r.string();
        let address = r.string();
        let category = r.string();
        let cmd1 = r.string();
        let cmd2 = r.string();
        let position = r.string();
        let time_or_tel = r.string();
        let mmsi = r.string();
        let distress_cause = r.string();
        let ack = r.string().and_then(|value| FieldReader::new(&[value.trim()]).char());
        let expansion = r.string().and_then(|value| FieldReader::new(&[value.trim()]).char());
        Self {
            format_specifier,
            address,
            category,
            cmd1,
            cmd2,
            position,
            time_or_tel,
            mmsi,
            distress_cause,
            ack,
            expansion,
        }
    }
}

impl NmeaEncodable for Dsc {
    const SENTENCE_TYPE: &str = "DSC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.format_specifier.as_deref());
        w.string(self.address.as_deref());
        w.string(self.category.as_deref());
        w.string(self.cmd1.as_deref());
        w.string(self.cmd2.as_deref());
        w.string(self.position.as_deref());
        w.string(self.time_or_tel.as_deref());
        w.string(self.mmsi.as_deref());
        w.string(self.distress_cause.as_deref());
        w.char(self.ack);
        w.char(self.expansion);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn dsc_empty() {
        let s = Dsc {
            format_specifier: None,
            address: None,
            category: None,
            cmd1: None,
            cmd2: None,
            position: None,
            time_or_tel: None,
            mmsi: None,
            distress_cause: None,
            ack: None,
            expansion: None,
        }
        .to_sentence("CD")
        .expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let d = Dsc::parse(&f.fields);
        assert!(d.format_specifier.is_none());
        assert!(d.address.is_none());
        assert!(d.expansion.is_none());
    }

    #[test]
    fn dsc_encode_roundtrip() {
        let original = Dsc {
            format_specifier: Some("12".to_string()),
            address: Some("3380400790".to_string()),
            category: Some("12".to_string()),
            cmd1: Some("06".to_string()),
            cmd2: Some("00".to_string()),
            position: Some("1423108312".to_string()),
            time_or_tel: Some("2019".to_string()),
            mmsi: None,
            distress_cause: None,
            ack: Some('S'),
            expansion: Some('E'),
        };
        let sentence = original.to_sentence("CD").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Dsc::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn dsc_cddsc_signalk() {
        let f = parse_frame("$CDDSC,12,3380400790,12,06,00,1423108312,2019,,,S,E*6A")
            .expect("valid DSC");
        let d = Dsc::parse(&f.fields);
        assert_eq!(d.format_specifier, Some("12".to_string()));
        assert_eq!(d.address, Some("3380400790".to_string()));
        assert_eq!(d.category, Some("12".to_string()));
        assert_eq!(d.cmd1, Some("06".to_string()));
        assert_eq!(d.cmd2, Some("00".to_string()));
        assert_eq!(d.position, Some("1423108312".to_string()));
        assert_eq!(d.time_or_tel, Some("2019".to_string()));
        assert_eq!(d.mmsi, None);
        assert_eq!(d.distress_cause, None);
        assert_eq!(d.ack, Some('S'));
        assert_eq!(d.expansion, Some('E'));
    }

    #[test]
    fn dsc_padded_indicators_gonmea() {
        let f = parse_frame(
            "$CDDSC,12,3380400790,12,06,00,1423108312,2019, ,  , S, E  *4a",
        )
        .expect("valid DSC");
        let d = Dsc::parse(&f.fields);
        assert_eq!(d.ack, Some('S'));
        assert_eq!(d.expansion, Some('E'));
    }
}
