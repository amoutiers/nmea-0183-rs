use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// DBT — Depth Below Transducer.
///
/// Wire: `depth_f,f,depth_m,M,depth_F,F`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dbt {
    /// Depth below transducer in feet.
    pub depth_feet: Option<f32>,
    /// Depth below transducer in meters.
    pub depth_meters: Option<f32>,
    /// Depth below transducer in fathoms.
    pub depth_fathoms: Option<f32>,
}

impl Dbt {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let depth_feet = r.f32();
        r.skip();
        let depth_meters = r.f32();
        r.skip();
        let depth_fathoms = r.f32();
        Self {
            depth_feet,
            depth_meters,
            depth_fathoms,
        }
    }
}

impl NmeaEncodable for Dbt {
    const SENTENCE_TYPE: &str = "DBT";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.depth_feet);
        w.fixed('f');
        w.f32(self.depth_meters);
        w.fixed('M');
        w.f32(self.depth_fathoms);
        w.fixed('F');
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn dbt_empty() {
        let f = parse_frame("$IIDBT,,,,,,*52").expect("valid");
        let d = Dbt::parse(&f.fields);
        assert!(d.depth_feet.is_none());
        assert!(d.depth_meters.is_none());
        assert!(d.depth_fathoms.is_none());
    }

    #[test]
    fn dbt_gpsd() {
        let f = parse_frame("$SDDBT,7.7,f,2.3,M,1.3,F*05").expect("valid DBT frame");
        let d = Dbt::parse(&f.fields);
        assert!((d.depth_meters.expect("depth_meters present") - 2.3).abs() < 0.01);
    }

    #[test]
    fn dbt_encode_roundtrip() {
        let original = Dbt {
            depth_feet: Some(35.53),
            depth_meters: Some(10.83),
            depth_fathoms: Some(5.85),
        };
        let sentence = original.to_sentence("II").expect("encode");
        assert!(sentence.starts_with("$IIDBT,"));

        let frame = parse_frame(sentence.trim()).expect("re-parse DBT sentence");
        let parsed = Dbt::parse(&frame.fields);

        assert_eq!(original, parsed);
    }

    #[test]
    fn dbt_encode_exact_fields() {
        let dbt = Dbt { depth_feet: Some(7.7), depth_meters: Some(2.3), depth_fathoms: Some(1.3) };
        assert_eq!(dbt.encode().expect("encode"), vec!["7.7", "f", "2.3", "M", "1.3", "F"]);
    }

    #[test]
    fn dbt_signalk() {
        let f = parse_frame("$IIDBT,035.53,f,010.83,M,005.85,F*23").expect("valid DBT frame");
        let d = Dbt::parse(&f.fields);
        assert!((d.depth_meters.expect("depth_meters present") - 10.83).abs() < 0.01);
    }
}
