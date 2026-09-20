use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// RSA — Rudder Sensor Angle.
///
/// Wire: `stbdangle,stbdstatus,portangle,portstatus`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rsa {
    /// Starboard (or single) rudder angle in degrees (negative = port).
    pub starboard_angle: Option<f32>,
    /// Starboard rudder status ('A' = valid, 'V' = invalid).
    pub starboard_status: Option<char>,
    /// Port rudder angle in degrees (negative = port).
    pub port_angle: Option<f32>,
    /// Port rudder status ('A' = valid, 'V' = invalid).
    pub port_status: Option<char>,
}

impl Rsa {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            starboard_angle: r.f32(),
            starboard_status: r.char(),
            port_angle: r.f32(),
            port_status: r.char(),
        }
    }
}

impl NmeaEncodable for Rsa {
    const SENTENCE_TYPE: &str = "RSA";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.starboard_angle);
        w.char(self.starboard_status);
        w.f32(self.port_angle);
        w.char(self.port_status);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn rsa_dual_rudder_gonmea() {
        let frame = parse_frame("$IIRSA,10.5,A,0.4,A*70").expect("valid go-nmea RSA frame");
        let rsa = Rsa::parse(&frame.fields);
        assert!((rsa.starboard_angle.expect("stbd") - 10.5).abs() < 0.1);
        assert_eq!(rsa.starboard_status, Some('A'));
        assert!((rsa.port_angle.expect("port") - 0.4).abs() < 0.1);
        assert_eq!(rsa.port_status, Some('A'));
    }

    #[test]
    fn rsa_empty() {
        let f = Rsa {
            starboard_angle: None,
            starboard_status: None,
            port_angle: None,
            port_status: None,
        }
        .to_sentence("II").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let r = Rsa::parse(&frame.fields);
        assert!(r.starboard_angle.is_none());
        assert!(r.starboard_status.is_none());
        assert!(r.port_angle.is_none());
        assert!(r.port_status.is_none());
    }

    #[test]
    fn rsa_encode_roundtrip() {
        let original = Rsa {
            starboard_angle: Some(10.5),
            starboard_status: Some('A'),
            port_angle: Some(-5.2),
            port_status: Some('A'),
        };
        let sentence = original.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Rsa::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn rsa_starboard_only() {
        let frame = parse_frame("$IIRSA,10.5,A,,V*4D").expect("valid RSA frame");
        let rsa = Rsa::parse(&frame.fields);
        assert!((rsa.starboard_angle.expect("stbd") - 10.5).abs() < 0.1);
        assert_eq!(rsa.starboard_status, Some('A'));
        assert!(rsa.port_angle.is_none());
        assert_eq!(rsa.port_status, Some('V'));
    }
}
