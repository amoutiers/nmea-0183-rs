use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// RPM — Engine Revolutions.
///
/// Wire: `source,engine_shaft_num,rpm,pitch,status`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rpm {
    /// Source indicator ('E' = engine, 'S' = shaft).
    pub source: Option<char>,
    /// Engine or shaft number.
    pub engine_shaft_num: Option<u8>,
    /// Speed in revolutions per minute.
    pub rpm: Option<f32>,
    /// Propeller pitch in percent.
    pub pitch: Option<f32>,
    /// Data validity ('A' = valid, 'V' = invalid).
    pub status: Option<char>,
}

impl Rpm {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            source: r.char(),
            engine_shaft_num: r.u8(),
            rpm: r.f32(),
            pitch: r.f32(),
            status: r.char(),
        }
    }
}

impl NmeaEncodable for Rpm {
    const SENTENCE_TYPE: &str = "RPM";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.source);
        w.u8(self.engine_shaft_num);
        w.f32(self.rpm);
        w.f32(self.pitch);
        w.char(self.status);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn rpm_empty() {
        let f = Rpm {
            source: None,
            engine_shaft_num: None,
            rpm: None,
            pitch: None,
            status: None,
        }
        .to_sentence("II").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let r = Rpm::parse(&frame.fields);
        assert!(r.source.is_none());
        assert!(r.engine_shaft_num.is_none());
        assert!(r.rpm.is_none());
        assert!(r.status.is_none());
    }

    #[test]
    fn rpm_encode_roundtrip() {
        let original = Rpm {
            source: Some('E'),
            engine_shaft_num: Some(1),
            rpm: Some(2418.2),
            pitch: Some(10.5),
            status: Some('A'),
        };
        let sentence = original.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Rpm::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn rpm_engine() {
        let frame = parse_frame("$IIRPM,E,1,2418.2,10.5,A*5F").expect("valid RPM frame");
        let rpm = Rpm::parse(&frame.fields);
        assert_eq!(rpm.source, Some('E'));
        assert_eq!(rpm.engine_shaft_num, Some(1));
        assert!((rpm.rpm.expect("rpm") - 2418.2).abs() < 0.1);
        assert!((rpm.pitch.expect("pitch") - 10.5).abs() < 0.1);
        assert_eq!(rpm.status, Some('A'));
    }

    #[test]
    fn rpm_shaft_gonmea() {
        let frame = parse_frame("$RCRPM,S,0,74.6,30.0,A*56").expect("valid go-nmea RPM frame");
        let rpm = Rpm::parse(&frame.fields);
        assert_eq!(rpm.source, Some('S'));
        assert_eq!(rpm.engine_shaft_num, Some(0));
        assert!((rpm.rpm.expect("rpm") - 74.6).abs() < 0.1);
        assert!((rpm.pitch.expect("pitch") - 30.0).abs() < 0.1);
        assert_eq!(rpm.status, Some('A'));
    }
}
