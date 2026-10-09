use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GMP: GNSS map projection fix data.
///
/// Wire: `time,projection,zone,northing,easting,mode,num_sats,hdop,altitude,geoid_sep,dgps_age,dgps_station`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gmp {
    /// UTC time of position (HHMMSS.SS format).
    pub time: Option<String>,
    /// Variable-length map projection identifier, such as `UTM` or `LOC`.
    pub map_projection: Option<String>,
    /// Variable-length map projection zone, such as `M20`.
    pub map_zone: Option<String>,
    /// Northern component of the grid or local coordinates.
    pub northing: Option<f32>,
    /// Eastern component of the grid or local coordinates.
    pub easting: Option<f32>,
    /// Variable-length constellation mode indicator, such as `DA`.
    pub mode: Option<String>,
    /// Total number of satellites in use.
    pub num_sats: Option<u8>,
    /// Horizontal dilution of precision.
    pub hdop: Option<f32>,
    /// Antenna altitude in meters relative to mean sea level or local coordinates.
    pub altitude: Option<f32>,
    /// Geoidal separation in meters.
    pub geoid_sep: Option<f32>,
    /// Age of differential data in seconds.
    pub dgps_age: Option<f32>,
    /// Differential reference station ID.
    pub dgps_station: Option<String>,
}

impl Gmp {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            time: r.string(),
            map_projection: r.string(),
            map_zone: r.string(),
            northing: r.f32(),
            easting: r.f32(),
            mode: r.string(),
            num_sats: r.u8(),
            hdop: r.f32(),
            altitude: r.f32(),
            geoid_sep: r.f32(),
            dgps_age: r.f32(),
            dgps_station: r.string(),
        }
    }
}

impl NmeaEncodable for Gmp {
    const SENTENCE_TYPE: &str = "GMP";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.time.as_deref());
        w.string(self.map_projection.as_deref());
        w.string(self.map_zone.as_deref());
        w.f32(self.northing);
        w.f32(self.easting);
        w.string(self.mode.as_deref());
        w.u8(self.num_sats);
        w.f32(self.hdop);
        w.f32(self.altitude);
        w.f32(self.geoid_sep);
        w.f32(self.dgps_age);
        w.string(self.dgps_station.as_deref());
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
    fn gmp_empty() {
        let frame = parse_frame("$GNGMP,,,,,,,,,,,,*53").expect("valid empty GMP frame");
        assert_eq!(
            Gmp::parse(&frame.fields),
            Gmp {
                time: None,
                map_projection: None,
                map_zone: None,
                northing: None,
                easting: None,
                mode: None,
                num_sats: None,
                hdop: None,
                altitude: None,
                geoid_sep: None,
                dgps_age: None,
                dgps_station: None,
            }
        );
    }

    #[test]
    fn gmp_encode_roundtrip() {
        let original = Gmp {
            time: Some("122310.2".into()),
            map_projection: Some("UTM".into()),
            map_zone: Some("M20".into()),
            northing: Some(12345.56),
            easting: Some(65543.21),
            mode: Some("DA".into()),
            num_sats: Some(14),
            hdop: Some(0.9),
            altitude: Some(1005.543),
            geoid_sep: Some(6.5),
            dgps_age: Some(5.2),
            dgps_station: Some("23".into()),
        };
        let sentence = original.to_sentence("GN").expect("encode GMP");
        let frame = parse_frame(sentence.trim()).expect("re-parse GMP");
        assert_eq!(Gmp::parse(&frame.fields), original);
    }

    #[test]
    fn gmp_values_nmea_301() {
        let frame = parse_frame(
            "$GNGMP,122310.2,UTM,M20,12345.56,65543.21,DA,14,0.9,1005.543,6.5,5.2,23*75",
        )
        .expect("valid NMEA 3.01 GMP frame");
        let gmp = Gmp::parse(&frame.fields);
        assert_eq!(gmp.time.as_deref(), Some("122310.2"));
        assert_eq!(gmp.map_projection.as_deref(), Some("UTM"));
        assert_eq!(gmp.map_zone.as_deref(), Some("M20"));
        assert_close(gmp.northing, 12345.56);
        assert_close(gmp.easting, 65543.21);
        assert_eq!(gmp.mode.as_deref(), Some("DA"));
        assert_eq!(gmp.num_sats, Some(14));
        assert_close(gmp.hdop, 0.9);
        assert_close(gmp.altitude, 1005.543);
        assert_close(gmp.geoid_sep, 6.5);
        assert_close(gmp.dgps_age, 5.2);
        assert_eq!(gmp.dgps_station.as_deref(), Some("23"));
    }
}
