use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HSC — Heading Steering Command.
///
/// Wire: `cmd_heading_true,T,cmd_heading_mag,M[,status]`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Hsc {
    /// Commanded heading true in degrees.
    pub cmd_heading_true: Option<f32>,
    /// Commanded heading magnetic in degrees.
    pub cmd_heading_mag: Option<f32>,
    /// Status indicator ('A' = valid). Optional in older NMEA versions.
    pub status: Option<char>,
}

impl Hsc {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let cmd_heading_true = r.f32();
        r.skip(); // T
        let cmd_heading_mag = r.f32();
        r.skip(); // M
        let status = r.char();
        Some(Self {
            cmd_heading_true,
            cmd_heading_mag,
            status,
        })
    }
}

impl NmeaEncodable for Hsc {
    const SENTENCE_TYPE: &str = "HSC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.cmd_heading_true);
        w.fixed('T');
        w.f32(self.cmd_heading_mag);
        w.fixed('M');
        // Only emit status field if present (optional in older NMEA versions).
        if self.status.is_some() {
            w.char(self.status);
        }
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn hsc_empty() {
        let f = Hsc {
            cmd_heading_true: None,
            cmd_heading_mag: None,
            status: None,
        }
        .to_sentence("FT").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let h = Hsc::parse(&frame.fields).expect("parse");
        assert!(h.cmd_heading_true.is_none());
        assert!(h.cmd_heading_mag.is_none());
        assert!(h.status.is_none());
    }

    #[test]
    fn hsc_encode_roundtrip() {
        let original = Hsc {
            cmd_heading_true: Some(40.12),
            cmd_heading_mag: Some(39.11),
            status: Some('A'),
        };
        let sentence = original.to_sentence("FT").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Hsc::parse(&frame.fields).expect("re-parse HSC");
        assert_eq!(original, parsed);
    }

    #[test]
    fn hsc_no_status_signalk() {
        let frame = parse_frame("$FTHSC,40.12,T,39.11,M*5E").expect("valid signalk HSC frame");
        let hsc = Hsc::parse(&frame.fields).expect("parse HSC");
        assert!((hsc.cmd_heading_true.expect("true") - 40.12).abs() < 0.01);
        assert!((hsc.cmd_heading_mag.expect("mag") - 39.11).abs() < 0.01);
        assert!(hsc.status.is_none());
    }
}
