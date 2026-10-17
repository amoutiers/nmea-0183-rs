use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HMS — Heading Monitor Set.
///
/// Wire: `sensor1,sensor2,max_difference_deg`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Hms {
    /// Heading sensor 1 identifier, retained as text because its domain is not specified.
    pub sensor1: Option<String>,
    /// Heading sensor 2 identifier, retained as text because its domain is not specified.
    pub sensor2: Option<String>,
    /// Maximum permitted difference between sensors, in degrees.
    pub max_difference_deg: Option<f32>,
}

impl Hms {
    /// Parse fields from a decoded NMEA frame; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            sensor1: r.string(),
            sensor2: r.string(),
            max_difference_deg: r.f32(),
        }
    }
}

impl NmeaEncodable for Hms {
    const SENTENCE_TYPE: &str = "HMS";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.sensor1.as_deref());
        w.string(self.sensor2.as_deref());
        w.f32(self.max_difference_deg);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    const FIXTURE: &str = "$--HMS,01,02,1.2*54";

    fn checksum(body: &str) -> u8 {
        body.bytes().fold(0, |value, byte| value ^ byte)
    }

    #[test]
    fn hms_empty() {
        assert_eq!(
            Hms::parse(&["", "", ""]),
            Hms {
                sensor1: None,
                sensor2: None,
                max_difference_deg: None,
            }
        );
    }

    #[test]
    fn hms_encode_roundtrip() {
        let original = Hms {
            sensor1: Some("01".to_string()),
            sensor2: Some("02".to_string()),
            max_difference_deg: Some(1.2),
        };
        let sentence = original.to_sentence("HE").expect("encode HMS");
        let frame = parse_frame(sentence.trim()).expect("re-parse HMS");
        assert_eq!(original, Hms::parse(&frame.fields));
    }

    #[test]
    fn hms_values_sim183() {
        // Published in the Ninglu SIM183 NMEA0183 Tester Manual, page 12.
        // https://iprorwxhqiiqli5q.leadongcdn.cn/NLT-SIM183-SSEN%2BSIM183%2BNMEA0183%2BTester%2BManual%2BV150804-aidqiBppKkkRliSqroimplkk.pdf
        let (body, reported_checksum) = FIXTURE[1..].split_once('*').expect("checksum separator");
        assert_eq!(
            checksum(body),
            u8::from_str_radix(reported_checksum, 16).expect("hex checksum")
        );

        let frame = parse_frame(FIXTURE).expect("valid SIM183 HMS frame");
        let hms = Hms::parse(&frame.fields);
        assert_eq!(hms.sensor1.as_deref(), Some("01"));
        assert_eq!(hms.sensor2.as_deref(), Some("02"));
        assert!((hms.max_difference_deg.expect("maximum difference") - 1.2).abs() < 1e-4);
    }
}
