use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PGRMT — Garmin Sensor Status Information.
///
/// Wire: `product_info,rom_checksum,receiver_failure,stored_data,rtc_lost,osc_drift,data_collection,sensor_temp,sensor_config`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PGRMT"`.
/// Encode with `to_sentence("")`.
#[derive(Debug, Clone, PartialEq)]
pub struct Pgrmt {
    /// Product/software information string.
    pub product_info: Option<String>,
    /// ROM checksum test ('P' = passed, 'F' = failed).
    pub rom_checksum: Option<char>,
    /// Receiver failure discrete ('P' = passed, 'F' = failed).
    pub receiver_failure: Option<char>,
    /// Stored data lost ('R' = retained, 'L' = lost).
    pub stored_data: Option<char>,
    /// Real-time clock lost ('R' = retained, 'L' = lost).
    pub rtc_lost: Option<char>,
    /// Oscillator drift discrete ('P' = passed, 'F' = failed).
    pub osc_drift: Option<char>,
    /// Data collection discrete ('C' = complete, 'I' = incomplete).
    pub data_collection: Option<char>,
    /// Sensor temperature in degrees C.
    pub sensor_temp: Option<String>,
    /// Sensor configuration ('S' = sensitivity, 'D' = default, 'N' = not available).
    pub sensor_config: Option<char>,
}

impl Pgrmt {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let product_info = r.string();
        let rom_checksum = r.char();
        let receiver_failure = r.char();
        let stored_data = r.char();
        let rtc_lost = r.char();
        let osc_drift = r.char();
        let data_collection = r.char();
        let sensor_temp = r.string();
        let sensor_config = r.char();
        Some(Self {
            product_info,
            rom_checksum,
            receiver_failure,
            stored_data,
            rtc_lost,
            osc_drift,
            data_collection,
            sensor_temp,
            sensor_config,
        })
    }
}

impl NmeaEncodable for Pgrmt {
    const SENTENCE_TYPE: &str = "PGRMT";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.product_info.as_deref());
        w.char(self.rom_checksum);
        w.char(self.receiver_failure);
        w.char(self.stored_data);
        w.char(self.rtc_lost);
        w.char(self.osc_drift);
        w.char(self.data_collection);
        w.string(self.sensor_temp.as_deref());
        w.char(self.sensor_config);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn pgrmt_empty() {
        let s = Pgrmt {
            product_info: None,
            rom_checksum: None,
            receiver_failure: None,
            stored_data: None,
            rtc_lost: None,
            osc_drift: None,
            data_collection: None,
            sensor_temp: None,
            sensor_config: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Pgrmt::parse(&f.fields).expect("parse");
        assert!(p.product_info.is_none());
        assert!(p.sensor_config.is_none());
    }

    #[test]
    fn pgrmt_encode_roundtrip() {
        let original = Pgrmt {
            product_info: Some("GPS24xd-HVS".to_string()),
            rom_checksum: Some('P'),
            receiver_failure: Some('P'),
            stored_data: Some('R'),
            rtc_lost: Some('R'),
            osc_drift: Some('P'),
            data_collection: Some('C'),
            sensor_temp: Some("25".to_string()),
            sensor_config: Some('S'),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Pgrmt::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn pgrmt_pgrmt_gonmea() {
        let frame = parse_frame("$PGRMT,GPS24xd-HVS VER 2.30,,,,,,,,*10").expect("valid");
        let p = Pgrmt::parse(&frame.fields).expect("parse");
        assert_eq!(p.product_info.as_deref(), Some("GPS24xd-HVS VER 2.30"));
        assert!(p.rom_checksum.is_none());
        assert!(p.sensor_config.is_none());
    }
}
