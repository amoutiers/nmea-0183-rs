use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PASHR — Roll and Pitch Sentence (Ashtech/Trimble).
///
/// Wire: `time,heading,T,roll,pitch,heave,roll_accuracy,pitch_accuracy,heading_accuracy,gnss_quality,imu_alignment`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PASHR"`.
/// Encode with `to_sentence("")` (talker is ignored for proprietary types).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pashr {
    /// UTC time of fix (hhmmss.sss).
    pub time: Option<String>,
    /// True heading in degrees.
    pub heading: Option<f32>,
    /// Roll angle in degrees (negative = port).
    pub roll: Option<f32>,
    /// Pitch angle in degrees (negative = bow down).
    pub pitch: Option<f32>,
    /// Heave in metres.
    pub heave: Option<f32>,
    /// Roll accuracy (1-sigma) in degrees.
    pub roll_accuracy: Option<f32>,
    /// Pitch accuracy (1-sigma) in degrees.
    pub pitch_accuracy: Option<f32>,
    /// Heading accuracy (1-sigma) in degrees.
    pub heading_accuracy: Option<f32>,
    /// GNSS quality (0=no fix, 1=non-diff, 2=diff).
    pub gnss_quality: Option<u8>,
    /// IMU alignment status (0=not aligned, 1=aligned).
    pub imu_alignment: Option<u8>,
}

impl Pashr {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let time = r.string();
        let heading = r.f32();
        r.skip(); // T
        let roll = r.f32();
        let pitch = r.f32();
        let heave = r.f32();
        let roll_accuracy = r.f32();
        let pitch_accuracy = r.f32();
        let heading_accuracy = r.f32();
        let gnss_quality = r.u8();
        let imu_alignment = r.u8();
        Some(Self {
            time,
            heading,
            roll,
            pitch,
            heave,
            roll_accuracy,
            pitch_accuracy,
            heading_accuracy,
            gnss_quality,
            imu_alignment,
        })
    }
}

impl NmeaEncodable for Pashr {
    const SENTENCE_TYPE: &str = "PASHR";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.f32(self.heading);
        w.fixed('T');
        w.f32(self.roll);
        w.f32(self.pitch);
        w.f32(self.heave);
        w.f32(self.roll_accuracy);
        w.f32(self.pitch_accuracy);
        w.f32(self.heading_accuracy);
        w.u8(self.gnss_quality);
        w.u8(self.imu_alignment);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn pashr_empty() {
        let s = Pashr {
            time: None,
            heading: None,
            roll: None,
            pitch: None,
            heave: None,
            roll_accuracy: None,
            pitch_accuracy: None,
            heading_accuracy: None,
            gnss_quality: None,
            imu_alignment: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Pashr::parse(&f.fields).expect("parse");
        assert!(p.time.is_none());
        assert!(p.heading.is_none());
    }

    #[test]
    fn pashr_encode_roundtrip() {
        let original = Pashr {
            time: Some("085335.000".to_string()),
            heading: Some(224.19),
            roll: Some(-1.26),
            pitch: Some(0.83),
            heave: Some(0.10),
            roll_accuracy: Some(0.101),
            pitch_accuracy: Some(0.113),
            heading_accuracy: Some(0.267),
            gnss_quality: Some(1),
            imu_alignment: Some(0),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Pashr::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn pashr_pashr_gonmea() {
        let frame =
            parse_frame("$PASHR,085335.000,224.19,T,-01.26,+00.83,+00.10,0.101,0.113,0.267,1,0*07")
                .expect("valid");
        let p = Pashr::parse(&frame.fields).expect("parse");
        assert_eq!(p.time.as_deref(), Some("085335.000"));
        assert!((p.heading.expect("heading") - 224.19).abs() < 0.01);
        assert_eq!(p.gnss_quality, Some(1));
        assert_eq!(p.imu_alignment, Some(0));
    }
}
