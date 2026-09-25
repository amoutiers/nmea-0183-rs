use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Msk {
    pub frequency: Option<f32>,
    pub frequency_mode: Option<char>,
    pub beacon_bps: Option<u16>,
    pub bps_mode: Option<char>,
    pub mms_frequency: Option<f32>,
    pub channel: Option<char>,
    pub status: Option<char>,
}

impl Msk {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            frequency: r.f32(),
            frequency_mode: r.char(),
            beacon_bps: r.u16(),
            bps_mode: r.char(),
            mms_frequency: r.f32(),
            channel: r.char(),
            status: r.char(),
        })
    }
}

impl NmeaEncodable for Msk {
    const SENTENCE_TYPE: &'static str = "MSK";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.frequency);
        w.char(self.frequency_mode);
        w.u16(self.beacon_bps);
        w.char(self.bps_mode);
        w.f32(self.mms_frequency);
        w.char(self.channel);
        w.char(self.status);
        w.finish()
    }
}
