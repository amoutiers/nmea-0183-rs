use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Mss {
    pub strength: Option<u8>,
    pub snr: Option<u8>,
    pub frequency: Option<f32>,
    pub beacon_bps: Option<u16>,
    pub channel: Option<char>,
}

impl Mss {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            strength: r.u8(),
            snr: r.u8(),
            frequency: r.f32(),
            beacon_bps: r.u16(),
            channel: r.char(),
        })
    }
}

impl NmeaEncodable for Mss {
    const SENTENCE_TYPE: &'static str = "MSS";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.strength);
        w.u8(self.snr);
        w.f32(self.frequency);
        w.u16(self.beacon_bps);
        w.char(self.channel);
        w.finish()
    }
}
