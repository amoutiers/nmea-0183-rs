use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GMP — GNSS Map Projection Fix Data.
///
/// Wire: `time,projection,zone,northing,easting,mode,numSats,hdop,alt,geoidSep,dgpsAge,dgpsStation`
#[derive(Debug, Clone, PartialEq)]
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
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
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
        })
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
