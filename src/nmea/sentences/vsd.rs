use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VSD — AIS voyage static data.
///
/// Conventional NMEA 0183 wire form: `$--VSD,...*hh`.
#[derive(Debug, Clone, PartialEq)]
pub struct Vsd {
    /// Type of ship and cargo (AIS type code).
    pub type_of_ship: Option<u8>,
    /// Maximum present static draught in metres.
    pub draught: Option<f32>,
    /// Number of persons on board (0-8191 in the protocol).
    pub persons: Option<u16>,
    /// Destination (up to 20 characters).
    pub destination: Option<String>,
    /// Estimated time of arrival (UTC, HHMM or HHMMSS depending on sender).
    pub arrival_time: Option<String>,
    /// Estimated arrival day (UTC).
    pub arrival_day: Option<u8>,
    /// Estimated arrival month (UTC).
    pub arrival_month: Option<u8>,
    /// Navigational status code.
    pub nav_status: Option<u8>,
    /// Regional reserved application identifier.
    pub regional: Option<u8>,
}

impl Vsd {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            type_of_ship: r.u8(),
            draught: r.f32(),
            persons: r.u16(),
            destination: r.string(),
            arrival_time: r.string(),
            arrival_day: r.u8(),
            arrival_month: r.u8(),
            nav_status: r.u8(),
            regional: r.u8(),
        })
    }
}

impl NmeaEncodable for Vsd {
    const SENTENCE_TYPE: &str = "VSD";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.type_of_ship);
        w.f32(self.draught);
        w.u16(self.persons);
        w.string(self.destination.as_deref());
        w.string(self.arrival_time.as_deref());
        w.u8(self.arrival_day);
        w.u8(self.arrival_month);
        w.u8(self.nav_status);
        w.u8(self.regional);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn vsd_empty() {
        let sentence = Vsd {
            type_of_ship: None,
            draught: None,
            persons: None,
            destination: None,
            arrival_time: None,
            arrival_day: None,
            arrival_month: None,
            nav_status: None,
            regional: None,
        }
        .to_sentence("RA")
        .expect("encode");
        let frame = parse_frame(sentence.trim()).expect("valid");
        let vsd = Vsd::parse(&frame.fields).expect("parse");
        assert!(vsd.type_of_ship.is_none());
        assert!(vsd.destination.is_none());
    }

    #[test]
    fn vsd_encode_rejects_non_ascii_destination() {
        let vsd = Vsd {
            type_of_ship: Some(0),
            draught: Some(4.5),
            persons: Some(6),
            destination: Some("Port de Saint-Maloé".to_string()),
            arrival_time: Some("220516".to_string()),
            arrival_day: Some(1),
            arrival_month: Some(2),
            nav_status: Some(8),
            regional: None,
        };
        assert_eq!(
            vsd.encode(),
            Err(crate::EncodeError::InvalidFieldCharacter('é'))
        );
    }

    #[test]
    fn vsd_encode_roundtrip() {
        let original = Vsd {
            type_of_ship: Some(0),
            draught: Some(4.5),
            persons: Some(6),
            destination: Some("PORT".to_string()),
            arrival_time: Some("220516".to_string()),
            arrival_day: Some(1),
            arrival_month: Some(2),
            nav_status: Some(8),
            regional: None,
        };
        let sentence = original.to_sentence("RA").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Vsd::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn vsd_ravsd_gonmea() {
        let frame =
            parse_frame("$RAVSD,0,4.5,6,@@@@@@@@@@@@@@@@@@@@,220516,01,02,8,*6E").expect("valid");
        let vsd = Vsd::parse(&frame.fields).expect("parse");
        assert_eq!(vsd.type_of_ship, Some(0));
        assert!((vsd.draught.expect("draught") - 4.5).abs() < 0.01);
        assert_eq!(vsd.persons, Some(6));
        assert_eq!(vsd.destination.as_deref(), Some("@@@@@@@@@@@@@@@@@@@@"));
        assert_eq!(vsd.arrival_time.as_deref(), Some("220516"));
        assert_eq!(vsd.arrival_day, Some(1));
        assert_eq!(vsd.arrival_month, Some(2));
        assert_eq!(vsd.nav_status, Some(8));
        assert!(vsd.regional.is_none());
    }
}
