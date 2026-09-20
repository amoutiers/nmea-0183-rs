use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// MWD — Wind Direction & Speed.
///
/// Wire: `dirT,T,dirM,M,speedN,N,speedM,M`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mwd {
    /// Wind direction true in degrees.
    pub wind_dir_true: Option<f32>,
    /// Wind direction magnetic in degrees.
    pub wind_dir_mag: Option<f32>,
    /// Wind speed in knots.
    pub wind_speed_kts: Option<f32>,
    /// Wind speed in meters per second.
    pub wind_speed_ms: Option<f32>,
}

impl Mwd {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let wind_dir_true = r.f32();
        r.skip(); // T
        let wind_dir_mag = r.f32();
        r.skip(); // M
        let wind_speed_kts = r.f32();
        r.skip(); // N
        let wind_speed_ms = r.f32();
        Some(Self {
            wind_dir_true,
            wind_dir_mag,
            wind_speed_kts,
            wind_speed_ms,
        })
    }
}

impl NmeaEncodable for Mwd {
    const SENTENCE_TYPE: &str = "MWD";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.wind_dir_true);
        w.fixed('T');
        w.f32(self.wind_dir_mag);
        w.fixed('M');
        w.f32(self.wind_speed_kts);
        w.fixed('N');
        w.f32(self.wind_speed_ms);
        w.fixed('M');
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn mwd_empty() {
        let f = parse_frame("$IIMWD,,,,,,,,*5E").expect("valid");
        let m = Mwd::parse(&f.fields).expect("parse");
        assert!(m.wind_dir_true.is_none());
        assert!(m.wind_dir_mag.is_none());
        assert!(m.wind_speed_kts.is_none());
        assert!(m.wind_speed_ms.is_none());
    }

    #[test]
    fn mwd_encode_partial() {
        let mwd = Mwd {
            wind_dir_true: None,
            wind_dir_mag: Some(46.0),
            wind_speed_kts: Some(10.1),
            wind_speed_ms: None,
        };
        let fields = mwd.encode().expect("encode");
        assert_eq!(fields[0], ""); // empty for None
        assert_eq!(fields[1], "T"); // fixed
        assert_eq!(fields[2], "46"); // mag
        assert_eq!(fields[3], "M"); // fixed
    }

    #[test]
    fn mwd_full_signalk() {
        let frame = parse_frame("$IIMWD,046.,T,046.,M,10.1,N,05.2,M*43").expect("valid");
        let mwd = Mwd::parse(&frame.fields).expect("parse MWD");
        assert!((mwd.wind_dir_true.expect("dir") - 46.0).abs() < 0.1);
        assert!((mwd.wind_dir_mag.expect("mag") - 46.0).abs() < 0.1);
        assert!((mwd.wind_speed_kts.expect("kts") - 10.1).abs() < 0.1);
        assert!((mwd.wind_speed_ms.expect("ms") - 5.2).abs() < 0.1);
    }

    #[test]
    fn mwd_partial_mag_only_signalk() {
        let frame = parse_frame("$IIMWD,,,046.,M,10.1,N,05.2,M*0B").expect("valid");
        let mwd = Mwd::parse(&frame.fields).expect("parse MWD");
        assert!(mwd.wind_dir_true.is_none());
        assert!((mwd.wind_dir_mag.expect("mag") - 46.0).abs() < 0.1);
    }

    #[test]
    fn mwd_partial_true_only_signalk() {
        let frame = parse_frame("$IIMWD,046.,T,,,,,5.2,M*72").expect("valid");
        let mwd = Mwd::parse(&frame.fields).expect("parse MWD");
        assert!((mwd.wind_dir_true.expect("true") - 46.0).abs() < 0.1);
        assert!(mwd.wind_dir_mag.is_none());
        assert!(mwd.wind_speed_kts.is_none());
        assert!((mwd.wind_speed_ms.expect("ms") - 5.2).abs() < 0.1);
    }
    #[test]
    fn mwd_encode_exact_fields() {
        let mwd = Mwd {
            wind_dir_true: Some(270.0),
            wind_dir_mag: Some(268.5),
            wind_speed_kts: Some(12.4),
            wind_speed_ms: Some(6.4),
        };
        assert_eq!(mwd.encode().expect("encode"), vec!["270", "T", "268.5", "M", "12.4", "N", "6.4", "M"]);
    }

    #[test]
    fn mwd_encode_roundtrip() {
        let mwd = Mwd {
            wind_dir_true: Some(270.0),
            wind_dir_mag: Some(268.5),
            wind_speed_kts: Some(12.4),
            wind_speed_ms: Some(6.4),
        };
        let sentence = mwd.to_sentence("WI").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let mwd2 = Mwd::parse(&frame.fields).expect("re-parse MWD");
        assert_eq!(mwd, mwd2);
    }

    #[test]
    fn mwd_strict_sentence_rejects_invalid_talker() {
        let mwd = Mwd::parse(&[]).expect("lenient typed parse");
        assert!(mwd.to_sentence("wi").is_ok());
        assert!(mwd.to_sentence_strict("wi").is_err());
        mwd.to_sentence_strict("WI").expect("strict MWD");
    }
}
