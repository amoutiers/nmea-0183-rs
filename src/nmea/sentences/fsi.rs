use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// FSI: Frequency set information.
///
/// Wire: `transmit_frequency,receive_frequency,operating_mode,power,status`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Fsi {
    /// Transmitting frequency in 100 Hz increments, preserved as text.
    pub transmit_frequency: Option<String>,
    /// Receiving frequency in 100 Hz increments, preserved as text.
    pub receive_frequency: Option<String>,
    pub operating_mode: Option<char>,
    pub power: Option<u8>,
    pub status: Option<char>,
}

impl Fsi {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            transmit_frequency: r.string(),
            receive_frequency: r.string(),
            operating_mode: r.char(),
            power: r.u8(),
            status: r.char(),
        }
    }
}

impl NmeaEncodable for Fsi {
    const SENTENCE_TYPE: &str = "FSI";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.transmit_frequency.as_deref());
        w.string(self.receive_frequency.as_deref());
        w.char(self.operating_mode);
        w.u8(self.power);
        w.char(self.status);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn fsi_empty() {
        assert_eq!(Fsi::parse(&[""; 5]), Fsi::default());
    }

    #[test]
    fn fsi_encode_roundtrip() {
        let original = Fsi {
            transmit_frequency: Some("015600".into()),
            receive_frequency: Some("015602".into()),
            operating_mode: Some('F'),
            power: Some(5),
            status: Some('C'),
        };
        let sentence = original.to_sentence("GP").expect("encode FSI");
        let frame = parse_frame(&sentence).expect("re-parse FSI");
        assert_eq!(Fsi::parse(&frame.fields), original);
    }

    #[test]
    fn fsi_malformed_fields() {
        assert_eq!(Fsi::parse(&["", "", "FM", "bad", "RR"]), Fsi::default());
    }

    #[test]
    fn fsi_missing_fields() {
        assert_eq!(Fsi::parse(&[]), Fsi::default());
    }

    #[test]
    fn fsi_values_marnav() {
        // https://github.com/mariokonrad/marnav/blob/master/test/marnav/nmea/Test_nmea_fsi.cpp
        let frame = parse_frame("$GPFSI,156000,156025,,,*60").expect("valid marnav FSI");
        let fsi = Fsi::parse(&frame.fields);
        assert_eq!(fsi.transmit_frequency.as_deref(), Some("156000"));
        assert_eq!(fsi.receive_frequency.as_deref(), Some("156025"));
    }
}
