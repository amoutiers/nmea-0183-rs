use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// MLA — GLONASS L1 C/A almanac data.
///
/// Wire: `#alm,alm#,slot,N,hlth_freq,ecc,delta_t_dot,w,t_16_msb,delta_t,t_pi,l,delta_i,t_12_lsb,t`
#[derive(Debug, Clone, PartialEq)]
pub struct Mla {
    /// Number of almanac messages in the set.
    pub almanac_count: Option<u8>,
    /// Current almanac message number, preserving its zero-padded representation.
    pub almanac_number: Option<String>,
    /// NMEA GLONASS satellite identifier (65-96).
    pub slot: Option<u8>,
    /// Calendar day count within the four-year period from the last leap year.
    pub calendar_day_count: Option<String>,
    /// Bit-packed health and carrier frequency data (two hexadecimal digits).
    pub health_and_frequency: Option<String>,
    /// Bit-packed eccentricity (four hexadecimal digits).
    pub eccentricity: Option<String>,
    /// Bit-packed rate of change of orbital period (two hexadecimal digits).
    pub delta_t_dot: Option<String>,
    /// Bit-packed argument of perigee (four hexadecimal digits).
    pub argument_of_perigee: Option<String>,
    /// Bit-packed most-significant clock offset (four hexadecimal digits).
    pub t_16_msb: Option<String>,
    /// Bit-packed correction to the mean Draconian period (six hexadecimal digits).
    pub delta_t: Option<String>,
    /// Bit-packed GLONASS time of ascending-node equator crossing (six hexadecimal digits).
    pub t_pi: Option<String>,
    /// Bit-packed longitude of ascending-node equator crossing (six hexadecimal digits).
    pub longitude_of_ascending_node: Option<String>,
    /// Bit-packed correction to nominal inclination (six hexadecimal digits).
    pub delta_i: Option<String>,
    /// Bit-packed least-significant clock offset (three hexadecimal digits).
    pub t_12_lsb: Option<String>,
    /// Bit-packed coarse time-scale shift (three hexadecimal digits).
    pub time_scale_shift: Option<String>,
}

impl Mla {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            almanac_count: r.u8(),
            almanac_number: r.string(),
            slot: r.u8(),
            calendar_day_count: r.string(),
            health_and_frequency: r.string(),
            eccentricity: r.string(),
            delta_t_dot: r.string(),
            argument_of_perigee: r.string(),
            t_16_msb: r.string(),
            delta_t: r.string(),
            t_pi: r.string(),
            longitude_of_ascending_node: r.string(),
            delta_i: r.string(),
            t_12_lsb: r.string(),
            time_scale_shift: r.string(),
        })
    }
}

impl NmeaEncodable for Mla {
    const SENTENCE_TYPE: &str = "MLA";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.almanac_count);
        w.string(self.almanac_number.as_deref());
        w.u8(self.slot);
        w.string(self.calendar_day_count.as_deref());
        w.string(self.health_and_frequency.as_deref());
        w.string(self.eccentricity.as_deref());
        w.string(self.delta_t_dot.as_deref());
        w.string(self.argument_of_perigee.as_deref());
        w.string(self.t_16_msb.as_deref());
        w.string(self.delta_t.as_deref());
        w.string(self.t_pi.as_deref());
        w.string(self.longitude_of_ascending_node.as_deref());
        w.string(self.delta_i.as_deref());
        w.string(self.t_12_lsb.as_deref());
        w.string(self.time_scale_shift.as_deref());
        w.finish()
    }
}
