use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GFA: GNSS fix accuracy and integrity.
///
/// Wire: `time,hpl,vpl,std_x,std_y,theta,std_h,sal,integrity_status`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gfa {
    /// UTC time of the associated GGA or GNS fix (HHMMSS.SS).
    pub time: Option<String>,
    /// Horizontal protection level in meters.
    pub horizontal_protection_level: Option<f32>,
    /// Vertical protection level in meters.
    pub vertical_protection_level: Option<f32>,
    /// Standard deviation of the semi-major axis of the error ellipse in meters.
    pub semi_major_std_dev: Option<f32>,
    /// Standard deviation of the semi-minor axis of the error ellipse in meters.
    pub semi_minor_std_dev: Option<f32>,
    /// Semi-major-axis orientation in degrees from true north.
    pub semi_major_orientation: Option<f32>,
    /// Standard deviation of altitude in meters.
    pub altitude_std_dev: Option<f32>,
    /// Selected accuracy level in meters.
    pub selected_accuracy_level: Option<f32>,
    /// Variable-length integrity status, such as `SCC`.
    pub integrity_status: Option<String>,
}

impl Gfa {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            time: r.string(),
            horizontal_protection_level: r.f32(),
            vertical_protection_level: r.f32(),
            semi_major_std_dev: r.f32(),
            semi_minor_std_dev: r.f32(),
            semi_major_orientation: r.f32(),
            altitude_std_dev: r.f32(),
            selected_accuracy_level: r.f32(),
            integrity_status: r.string(),
        }
    }
}

impl NmeaEncodable for Gfa {
    const SENTENCE_TYPE: &str = "GFA";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.f32(self.horizontal_protection_level);
        w.f32(self.vertical_protection_level);
        w.f32(self.semi_major_std_dev);
        w.f32(self.semi_minor_std_dev);
        w.f32(self.semi_major_orientation);
        w.f32(self.altitude_std_dev);
        w.f32(self.selected_accuracy_level);
        w.string(self.integrity_status.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    fn assert_close(actual: Option<f32>, expected: f32) {
        assert!((actual.expect("value present") - expected).abs() < 0.001);
    }

    #[test]
    fn gfa_empty() {
        let frame = parse_frame("$GNGFA,,,,,,,,,*65").expect("valid empty GFA frame");
        assert_eq!(
            Gfa::parse(&frame.fields),
            Gfa {
                time: None,
                horizontal_protection_level: None,
                vertical_protection_level: None,
                semi_major_std_dev: None,
                semi_minor_std_dev: None,
                semi_major_orientation: None,
                altitude_std_dev: None,
                selected_accuracy_level: None,
                integrity_status: None,
            }
        );
    }

    #[test]
    fn gfa_encode_roundtrip() {
        let original = Gfa {
            time: Some("224229.00".into()),
            horizontal_protection_level: Some(1.7),
            vertical_protection_level: Some(2.9),
            semi_major_std_dev: Some(0.43),
            semi_minor_std_dev: Some(0.22),
            semi_major_orientation: Some(14.4868),
            altitude_std_dev: Some(0.83),
            selected_accuracy_level: Some(10.0),
            integrity_status: Some("SCC".into()),
        };
        let sentence = original.to_sentence("GN").expect("encode GFA");
        let frame = parse_frame(sentence.trim()).expect("re-parse GFA");
        assert_eq!(Gfa::parse(&frame.fields), original);
    }

    #[test]
    fn gfa_values_deere_navcom() {
        let frame = parse_frame(
            "$GNGFA,224229.00,0001.7,0002.9,000.43,000.22,014.4868,000.83,0010.0,SCC*0C",
        )
        .expect("valid Deere/NavCom GFA frame");
        let gfa = Gfa::parse(&frame.fields);
        assert_eq!(gfa.time.as_deref(), Some("224229.00"));
        assert_close(gfa.horizontal_protection_level, 1.7);
        assert_close(gfa.vertical_protection_level, 2.9);
        assert_close(gfa.semi_major_std_dev, 0.43);
        assert_close(gfa.semi_minor_std_dev, 0.22);
        assert_close(gfa.semi_major_orientation, 14.4868);
        assert_close(gfa.altitude_std_dev, 0.83);
        assert_close(gfa.selected_accuracy_level, 10.0);
        assert_eq!(gfa.integrity_status.as_deref(), Some("SCC"));
    }
}
