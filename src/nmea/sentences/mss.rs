use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// MSS: MSK receiver signal status.
///
/// Wire: `strength,snr,frequency,beacon_bps,channel`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mss {
    /// Signal strength in dB relative to 1 microvolt per meter.
    pub strength: Option<f32>,
    /// Signal-to-noise ratio in dB.
    pub snr: Option<f32>,
    /// Beacon frequency in kHz.
    pub frequency: Option<f32>,
    pub beacon_bps: Option<u16>,
    pub channel: Option<char>,
}

impl Mss {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            strength: r.f32(),
            snr: r.f32(),
            frequency: r.f32(),
            beacon_bps: r.u16(),
            channel: r.char(),
        }
    }
}

impl NmeaEncodable for Mss {
    const SENTENCE_TYPE: &str = "MSS";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.strength);
        w.f32(self.snr);
        w.f32(self.frequency);
        w.u16(self.beacon_bps);
        w.char(self.channel);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn mss_empty() {
        assert_eq!(Mss::parse(&[""; 5]), Mss::default());
    }

    #[test]
    fn mss_encode_roundtrip() {
        let original = Mss {
            strength: Some(26.4),
            snr: Some(7.2),
            frequency: Some(283.5),
            beacon_bps: Some(200),
            channel: Some('2'),
        };
        let sentence = original.to_sentence("GP").expect("encode MSS");
        let frame = parse_frame(&sentence).expect("re-parse MSS");
        assert_eq!(Mss::parse(&frame.fields), original);
    }

    #[test]
    fn mss_malformed_fields() {
        assert_eq!(Mss::parse(&["NaN", "inf", "bad", "-1", "12"]), Mss::default());
    }

    #[test]
    fn mss_missing_fields() {
        assert_eq!(Mss::parse(&[]), Mss::default());
    }

    #[test]
    fn mss_values_trimble() {
        // https://help.fieldsystems.trimble.com/r750/nmea0183-messages-mss.htm
        let frame = parse_frame("$GPMSS,26.4,7.2,283.5,200,2*61").expect("valid Trimble MSS");
        let mss = Mss::parse(&frame.fields);
        assert!((mss.strength.expect("signal strength") - 26.4).abs() < 0.001);
        assert!((mss.snr.expect("SNR") - 7.2).abs() < 0.001);
        assert!((mss.frequency.expect("frequency") - 283.5).abs() < 0.001);
        assert_eq!(mss.beacon_bps, Some(200));
        assert_eq!(mss.channel, Some('2'));
    }
}
