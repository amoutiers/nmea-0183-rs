use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// WNC: Distance, Waypoint to Waypoint.
///
/// Wire: `distance_nm,N,distance_km,K,destination_waypoint,origin_waypoint`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Wnc {
    /// Distance between the waypoints in nautical miles.
    pub distance_nm: Option<f32>,
    /// Distance between the waypoints in kilometres.
    pub distance_km: Option<f32>,
    /// Destination ('TO') waypoint identifier.
    pub destination_waypoint: Option<String>,
    /// Origin ('FROM') waypoint identifier.
    pub origin_waypoint: Option<String>,
}

impl Wnc {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let distance_nm = r.f32();
        r.skip(); // N
        let distance_km = r.f32();
        r.skip(); // K
        Self {
            distance_nm,
            distance_km,
            destination_waypoint: r.string(),
            origin_waypoint: r.string(),
        }
    }
}

impl NmeaEncodable for Wnc {
    const SENTENCE_TYPE: &str = "WNC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.distance_nm);
        w.fixed('N');
        w.f32(self.distance_km);
        w.fixed('K');
        w.string(self.destination_waypoint.as_deref());
        w.string(self.origin_waypoint.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn wnc_empty() {
        let frame = parse_frame("$GPWNC,,,,,,*4D").expect("valid empty WNC");
        let wnc = Wnc::parse(&frame.fields);
        assert_eq!(wnc, Wnc::default());

        let sentence = wnc.to_sentence("GP").expect("encode empty WNC");
        let (body, _) = sentence.split_once('*').expect("checksum");
        assert_eq!(body, "$GPWNC,,N,,K,,");
    }

    #[test]
    fn wnc_encode_roundtrip() {
        let original = Wnc {
            distance_nm: Some(3.5),
            distance_km: Some(6.482),
            destination_waypoint: Some("DEST".into()),
            origin_waypoint: Some("START".into()),
        };
        let sentence = original.to_sentence("GP").expect("encode WNC");
        let frame = parse_frame(&sentence).expect("re-parse WNC");
        assert_eq!(Wnc::parse(&frame.fields), original);
    }

    #[test]
    fn wnc_fixed_indicators() {
        let wnc = Wnc::parse(&["12.3", "", "5.6", "wrong", "POINT1", "POINT2"]);
        let sentence = wnc.to_sentence("GP").expect("encode WNC");
        let (body, _) = sentence.split_once('*').expect("checksum");
        assert_eq!(body, "$GPWNC,12.3,N,5.6,K,POINT1,POINT2");
    }

    #[test]
    fn wnc_malformed_fields() {
        for invalid in ["bad", "NaN", "inf"] {
            let wnc = Wnc::parse(&[invalid, "N", invalid, "K", "DEST", "START"]);
            assert_eq!(
                wnc,
                Wnc {
                    destination_waypoint: Some("DEST".into()),
                    origin_waypoint: Some("START".into()),
                    ..Default::default()
                }
            );
        }
    }

    #[test]
    fn wnc_missing_fields() {
        assert_eq!(Wnc::parse(&[]), Wnc::default());
        assert_eq!(
            Wnc::parse(&["12.3"]),
            Wnc {
                distance_nm: Some(12.3),
                ..Default::default()
            }
        );
    }

    #[test]
    fn wnc_values_marnav() {
        let frame = parse_frame("$GPWNC,12.3,N,5.6,K,POINT1,POINT2*78")
            .expect("valid marnav WNC vector");
        let wnc = Wnc::parse(&frame.fields);
        assert!((wnc.distance_nm.expect("distance_nm") - 12.3).abs() < 1e-4);
        assert!((wnc.distance_km.expect("distance_km") - 5.6).abs() < 1e-4);
        assert_eq!(wnc.destination_waypoint.as_deref(), Some("POINT1"));
        assert_eq!(wnc.origin_waypoint.as_deref(), Some("POINT2"));
    }
}
