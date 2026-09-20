use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PSKPDPT — Skipper Depth Sentence.
///
/// Wire: `depth,offset,range_scale,echo_strength,channel,transducer_location`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PSKPDPT"`.
/// Encode with `to_sentence("")` (talker is ignored for proprietary types).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pskpdpt {
    /// Water depth in metres.
    pub depth: Option<f32>,
    /// Transducer offset in metres.
    pub offset: Option<f32>,
    /// Maximum depth range scale in metres.
    pub range_scale: Option<f32>,
    /// Echo strength (0–100).
    pub echo_strength: Option<u8>,
    /// Channel number.
    pub channel: Option<u8>,
    /// Transducer location description.
    pub transducer_location: Option<String>,
}

impl Pskpdpt {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let depth = r.f32();
        let offset = r.f32();
        let range_scale = r.f32();
        let echo_strength = r.u8();
        let channel = r.u8();
        let transducer_location = r.string();
        Self {
            depth,
            offset,
            range_scale,
            echo_strength,
            channel,
            transducer_location,
        }
    }
}

impl NmeaEncodable for Pskpdpt {
    const SENTENCE_TYPE: &str = "PSKPDPT";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.depth);
        w.f32(self.offset);
        w.f32(self.range_scale);
        w.u8(self.echo_strength);
        w.u8(self.channel);
        w.string(self.transducer_location.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn pskpdpt_empty() {
        let s = Pskpdpt {
            depth: None,
            offset: None,
            range_scale: None,
            echo_strength: None,
            channel: None,
            transducer_location: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Pskpdpt::parse(&f.fields);
        assert!(p.depth.is_none());
        assert!(p.transducer_location.is_none());
    }

    #[test]
    fn pskpdpt_encode_roundtrip() {
        let original = Pskpdpt {
            depth: Some(2.5),
            offset: Some(0.0),
            range_scale: Some(10.0),
            echo_strength: Some(10),
            channel: Some(3),
            transducer_location: None,
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Pskpdpt::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn pskpdpt_pskpdpt_gonmea() {
        let frame = parse_frame("$PSKPDPT,0002.5,+00.0,0010,10,03,*77").expect("valid");
        let p = Pskpdpt::parse(&frame.fields);
        assert!((p.depth.expect("depth") - 2.5).abs() < 0.01);
        assert!((p.offset.expect("offset") - 0.0).abs() < 0.01);
        assert_eq!(p.range_scale, Some(10.0));
        assert_eq!(p.echo_strength, Some(10));
        assert_eq!(p.channel, Some(3));
        assert!(p.transducer_location.is_none());
    }

    #[test]
    fn pskpdpt_with_location_gonmea() {
        let frame = parse_frame("$PSKPDPT,0002.5,-01.1,0010,10,03,AFT*22").expect("valid");
        let p = Pskpdpt::parse(&frame.fields);
        assert!((p.depth.expect("depth") - 2.5).abs() < 0.01);
        assert!((p.offset.expect("offset") - (-1.1)).abs() < 0.01);
        assert_eq!(p.range_scale, Some(10.0));
        assert_eq!(p.echo_strength, Some(10));
        assert_eq!(p.channel, Some(3));
        assert_eq!(p.transducer_location.as_deref(), Some("AFT"));
    }

    #[test]
    fn pskpdpt_fractional_range_scale() {
        let f = parse_frame("$PSKPDPT,0002.5,+00.0,0010.5,10,03,*6C").expect("valid");
        let p = Pskpdpt::parse(&f.fields);
        assert_eq!(p.range_scale, Some(10.5));
    }

    #[test]
    fn pskpdpt_to_sentence_no_collision() {
        use crate::NmeaSentence;
        let p = Pskpdpt {
            depth: Some(2.5),
            offset: Some(0.0),
            range_scale: Some(10.0),
            echo_strength: Some(10),
            channel: Some(3),
            transducer_location: None,
        };
        // to_sentence ignores the talker for proprietary types.
        let s = p.to_sentence("II").expect("encode");
        assert!(s.starts_with("$PSKPDPT,"), "got {s}");
        let frame = parse_frame(s.trim()).expect("re-parse");
        assert_eq!(frame.sentence_type, "PSKPDPT");
        // Dispatches to the proprietary Pskpdpt, never the standard Dpt.
        assert!(matches!(
            NmeaSentence::parse(&frame),
            NmeaSentence::Pskpdpt(_)
        ));
    }
}
