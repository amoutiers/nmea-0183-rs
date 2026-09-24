use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GRS — GNSS Range Residuals.
///
/// Wire: `time,mode,residual01..residual12,system_id,signal_id`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Grs {
    /// UTC time of the residual measurement (HHMMSS.SS).
    pub time: Option<String>,
    /// Residual calculation mode (0 = post-fit, 1 = pre-fit).
    pub mode: Option<u8>,
    /// Range residuals for the first twelve satellites, in metres.
    pub residuals: [Option<f32>; 12],
    /// GNSS system identifier (NMEA 4.10+), encoded as one hexadecimal digit.
    pub system_id: Option<char>,
    /// GNSS signal identifier (NMEA 4.10+), encoded as one hexadecimal digit.
    pub signal_id: Option<char>,
}

impl Grs {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let time = r.string();
        let mode = r.u8();
        let residuals = [
            r.f32(),
            r.f32(),
            r.f32(),
            r.f32(),
            r.f32(),
            r.f32(),
            r.f32(),
            r.f32(),
            r.f32(),
            r.f32(),
            r.f32(),
            r.f32(),
        ];
        let system_id = r.char();
        let signal_id = r.char();
        Self {
            time,
            mode,
            residuals,
            system_id,
            signal_id,
        }
    }
}

impl NmeaEncodable for Grs {
    const SENTENCE_TYPE: &str = "GRS";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.u8(self.mode);
        for residual in &self.residuals {
            w.f32(*residual);
        }
        w.char(self.system_id);
        w.char(self.signal_id);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn grs_empty() {
        let frame = parse_frame("$GNGRS,,,,,,,,,,,,,,,,*4F").expect("valid empty GRS");
        assert_eq!(Grs::parse(&frame.fields), Grs::default());
    }

    #[test]
    fn grs_encode_roundtrip() {
        let original = Grs {
            time: Some("103607.00".to_string()),
            mode: Some(1),
            residuals: [
                Some(-2.1), Some(0.2), Some(2.7), Some(-0.4), None, None, None, None, None, None,
                None, None,
            ],
            system_id: Some('1'),
            signal_id: Some('1'),
        };
        let sentence = original.to_sentence("GN").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        assert_eq!(Grs::parse(&frame.fields), original);
    }
}
