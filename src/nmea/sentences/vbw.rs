use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VBW — Dual Ground/Water Speed.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vbw {
    /// Longitudinal water speed in knots.
    pub long_water_spd: Option<f32>,
    /// Transverse water speed in knots.
    pub trans_water_spd: Option<f32>,
    /// Water speed status ('A' = valid, 'V' = invalid).
    pub water_spd_status: Option<char>,
    /// Longitudinal ground speed in knots.
    pub long_ground_spd: Option<f32>,
    /// Transverse ground speed in knots.
    pub trans_ground_spd: Option<f32>,
    /// Ground speed status ('A' = valid, 'V' = invalid).
    pub ground_spd_status: Option<char>,
    /// Stern transverse water speed in knots.
    pub stern_trans_water_spd: Option<f32>,
    /// Stern water speed status ('A' = valid, 'V' = invalid).
    pub stern_water_spd_status: Option<char>,
    /// Stern transverse ground speed in knots.
    pub stern_trans_ground_spd: Option<f32>,
    /// Stern ground speed status ('A' = valid, 'V' = invalid).
    pub stern_ground_spd_status: Option<char>,
}

impl Vbw {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            long_water_spd: r.f32(),
            trans_water_spd: r.f32(),
            water_spd_status: r.char(),
            long_ground_spd: r.f32(),
            trans_ground_spd: r.f32(),
            ground_spd_status: r.char(),
            stern_trans_water_spd: r.f32(),
            stern_water_spd_status: r.char(),
            stern_trans_ground_spd: r.f32(),
            stern_ground_spd_status: r.char(),
        })
    }
}

impl NmeaEncodable for Vbw {
    const SENTENCE_TYPE: &str = "VBW";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.long_water_spd);
        w.f32(self.trans_water_spd);
        w.char(self.water_spd_status);
        w.f32(self.long_ground_spd);
        w.f32(self.trans_ground_spd);
        w.char(self.ground_spd_status);
        w.f32(self.stern_trans_water_spd);
        w.char(self.stern_water_spd_status);
        w.f32(self.stern_trans_ground_spd);
        w.char(self.stern_ground_spd_status);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn vbw_12_pynmeagps() {
        let frame =
            parse_frame("$GPVBW,12.3,0.07,A,11.78,0.12,A*6F").expect("valid pynmeagps VBW frame");
        let vbw = Vbw::parse(&frame.fields).expect("parse VBW");
        assert!((vbw.long_water_spd.expect("long_water") - 12.3).abs() < 0.1);
        assert!((vbw.trans_water_spd.expect("trans_water") - 0.07).abs() < 0.01);
        assert_eq!(vbw.water_spd_status, Some('A'));
        assert!((vbw.long_ground_spd.expect("long_ground") - 11.78).abs() < 0.01);
        assert!((vbw.trans_ground_spd.expect("trans_ground") - 0.12).abs() < 0.01);
        assert_eq!(vbw.ground_spd_status, Some('A'));
    }

    #[test]
    fn vbw_empty() {
        let frame = parse_frame("$IIVBW,,,,,,,,,*6F").expect("valid VBW frame");
        let vbw = Vbw::parse(&frame.fields).expect("parse VBW");
        assert!(vbw.long_water_spd.is_none());
        assert!(vbw.trans_water_spd.is_none());
        assert!(vbw.water_spd_status.is_none());
        assert!(vbw.long_ground_spd.is_none());
        assert!(vbw.trans_ground_spd.is_none());
        assert!(vbw.ground_spd_status.is_none());
        assert!(vbw.stern_trans_water_spd.is_none());
        assert!(vbw.stern_water_spd_status.is_none());
        assert!(vbw.stern_trans_ground_spd.is_none());
        assert!(vbw.stern_ground_spd_status.is_none());
    }

    #[test]
    fn vbw_stern_gonmea() {
        let frame = parse_frame("$VMVBW,-7.1,0.1,A,,,V,,V,,V*65").expect("valid go-nmea VBW frame");
        let vbw = Vbw::parse(&frame.fields).expect("parse VBW");
        assert!((vbw.long_water_spd.expect("long_water") - (-7.1)).abs() < 0.01);
        assert!((vbw.trans_water_spd.expect("trans_water") - 0.1).abs() < 0.01);
        assert_eq!(vbw.water_spd_status, Some('A'));
        assert!(vbw.long_ground_spd.is_none());
        assert!(vbw.trans_ground_spd.is_none());
        assert_eq!(vbw.ground_spd_status, Some('V'));
        assert!(vbw.stern_trans_water_spd.is_none());
        assert_eq!(vbw.stern_water_spd_status, Some('V'));
        assert!(vbw.stern_trans_ground_spd.is_none());
        assert_eq!(vbw.stern_ground_spd_status, Some('V'));
    }

    #[test]
    fn vbw_encode_roundtrip() {
        let original = Vbw {
            long_water_spd: Some(5.2),
            trans_water_spd: Some(0.1),
            water_spd_status: Some('A'),
            long_ground_spd: Some(5.3),
            trans_ground_spd: Some(0.2),
            ground_spd_status: Some('A'),
            stern_trans_water_spd: None,
            stern_water_spd_status: None,
            stern_trans_ground_spd: None,
            stern_ground_spd_status: None,
        };
        let sentence = original.to_sentence("II").expect("encode");
        let f = parse_frame(sentence.trim()).expect("re-parse VBW frame");
        let parsed = Vbw::parse(&f.fields).expect("parse VBW from re-encoded frame");
        assert_eq!(original, parsed);
    }
}
