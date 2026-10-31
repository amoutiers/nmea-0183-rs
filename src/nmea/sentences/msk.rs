use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// MSK: MSK receiver interface.
///
/// Wire: `frequency,frequency_mode,beacon_bps,bps_mode,mss_interval,channel,status`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Msk {
    /// Beacon frequency in kHz.
    pub frequency: Option<f32>,
    pub frequency_mode: Option<char>,
    pub beacon_bps: Option<u16>,
    pub bps_mode: Option<char>,
    /// Interval between MSS status messages in seconds.
    pub mss_interval: Option<f32>,
    /// Optional receiver channel number.
    pub channel: Option<char>,
    /// Optional sentence status flag.
    pub status: Option<char>,
}

impl Msk {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            frequency: r.f32(),
            frequency_mode: r.char(),
            beacon_bps: r.u16(),
            bps_mode: r.char(),
            mss_interval: r.f32(),
            channel: r.char(),
            status: r.char(),
        }
    }
}

impl NmeaEncodable for Msk {
    const SENTENCE_TYPE: &str = "MSK";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.frequency);
        w.char(self.frequency_mode);
        w.u16(self.beacon_bps);
        w.char(self.bps_mode);
        w.f32(self.mss_interval);
        w.char(self.channel);
        w.char(self.status);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn msk_empty() {
        assert_eq!(Msk::parse(&[""; 7]), Msk::default());
    }

    #[test]
    fn msk_encode_roundtrip() {
        let original = Msk {
            frequency: Some(318.5),
            frequency_mode: Some('M'),
            beacon_bps: Some(200),
            bps_mode: Some('A'),
            mss_interval: Some(2.5),
            channel: Some('2'),
            status: Some('R'),
        };
        let sentence = original.to_sentence("GP").expect("encode MSK");
        let frame = parse_frame(&sentence).expect("re-parse MSK");
        assert_eq!(Msk::parse(&frame.fields), original);
    }

    #[test]
    fn msk_malformed_fields() {
        assert_eq!(
            Msk::parse(&["NaN", "auto", "65536", "manual", "inf", "12", "RR"]),
            Msk::default(),
        );
    }

    #[test]
    fn msk_missing_fields() {
        assert_eq!(Msk::parse(&[]), Msk::default());
    }

    #[test]
    fn msk_values_pynmeagps() {
        // https://github.com/semuconsulting/pynmeagps/blob/master/tests/pygpsdata-nmea4.log
        let frame = parse_frame("$GPMSK,318.0,A,100,M,2*45").expect("valid pynmeagps MSK");
        let msk = Msk::parse(&frame.fields);
        assert!((msk.frequency.expect("frequency") - 318.0).abs() < 0.001);
        assert!((msk.mss_interval.expect("MSS interval") - 2.0).abs() < 0.001);
        assert_eq!(msk.beacon_bps, Some(100));
    }
}
