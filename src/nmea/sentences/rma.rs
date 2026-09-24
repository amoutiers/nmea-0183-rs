use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// RMA — Recommended Minimum Specific LORAN-C Data.
///
/// Wire: `status,lat,NS,lon,EW,td_a,td_b,sog,cog,mag_var,mag_var_ew`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rma {
    /// Status: 'A' = valid; 'V' = warning.
    pub status: Option<char>,
    /// Latitude in NMEA format (DDMM.MMMM).
    pub lat: Option<f64>,
    /// Latitude hemisphere: 'N' or 'S'.
    pub ns: Option<char>,
    /// Longitude in NMEA format (DDDMM.MMMM).
    pub lon: Option<f64>,
    /// Longitude hemisphere: 'E' or 'W'.
    pub ew: Option<char>,
    /// LORAN-C time difference A, in microseconds.
    pub td_a: Option<f32>,
    /// LORAN-C time difference B, in microseconds.
    pub td_b: Option<f32>,
    /// Speed over ground in knots.
    pub sog: Option<f32>,
    /// Course over ground in degrees true.
    pub cog: Option<f32>,
    /// Magnetic variation in degrees.
    pub mag_var: Option<f32>,
    /// Magnetic variation direction: 'E' or 'W'.
    pub mag_var_ew: Option<char>,
}

impl Rma {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            status: r.char(),
            lat: r.f64(),
            ns: r.char(),
            lon: r.f64(),
            ew: r.char(),
            td_a: r.f32(),
            td_b: r.f32(),
            sog: r.f32(),
            cog: r.f32(),
            mag_var: r.f32(),
            mag_var_ew: r.char(),
        }
    }
}

impl NmeaEncodable for Rma {
    const SENTENCE_TYPE: &str = "RMA";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.status);
        w.lat(self.lat);
        w.char(self.ns);
        w.lon(self.lon);
        w.char(self.ew);
        w.f32(self.td_a);
        w.f32(self.td_b);
        w.f32(self.sog);
        w.f32(self.cog);
        w.f32(self.mag_var);
        w.char(self.mag_var_ew);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn rma_empty() {
        let frame = parse_frame("$GPRMA,,,,,,,,,,,*65").expect("valid empty RMA");
        assert_eq!(Rma::parse(&frame.fields), Rma::default());
    }

    #[test]
    fn rma_encode_roundtrip() {
        let original = Rma {
            status: Some('A'),
            lat: Some(5327.03942),
            ns: Some('N'),
            lon: Some(11214.42462),
            ew: Some('W'),
            td_a: Some(12345.6),
            td_b: Some(23456.7),
            sog: Some(23.1),
            cog: Some(23.0),
            mag_var: Some(14.8),
            mag_var_ew: Some('W'),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        assert_eq!(Rma::parse(&frame.fields), original);
    }
}
