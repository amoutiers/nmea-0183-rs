use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Fsi {
    pub transmit_frequency: Option<String>,
    pub receive_frequency: Option<String>,
    pub operating_mode: Option<String>,
    pub power: Option<u8>,
    pub status: Option<char>,
}

impl Fsi {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            transmit_frequency: r.string(),
            receive_frequency: r.string(),
            operating_mode: r.string(),
            power: r.u8(),
            status: r.char(),
        })
    }
}

impl NmeaEncodable for Fsi {
    const SENTENCE_TYPE: &str = "FSI";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.transmit_frequency.as_deref());
        w.string(self.receive_frequency.as_deref());
        w.string(self.operating_mode.as_deref());
        w.u8(self.power);
        w.char(self.status);
        w.finish()
    }
}
