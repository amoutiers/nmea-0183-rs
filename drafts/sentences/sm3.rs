use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Sm3 {
    pub status: Option<char>,
    pub message_number: Option<u32>,
    pub les_sequence: Option<u32>,
    pub les_id: Option<u16>,
    pub ocean_region: Option<u8>,
    pub priority: Option<u8>,
    pub service: Option<u8>,
    pub presentation: Option<u8>,
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub latitude: Option<f64>,
    pub ns: Option<char>,
    pub longitude: Option<f64>,
    pub ew: Option<char>,
    pub radius: Option<u32>,
}

impl Sm3 {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            status: r.char(),
            message_number: r.u32(),
            les_sequence: r.u32(),
            les_id: r.u16(),
            ocean_region: r.u8(),
            priority: r.u8(),
            service: r.u8(),
            presentation: r.u8(),
            year: r.u16(),
            month: r.u8(),
            day: r.u8(),
            hour: r.u8(),
            minute: r.u8(),
            latitude: r.f64(),
            ns: r.char(),
            longitude: r.f64(),
            ew: r.char(),
            radius: r.u32(),
        })
    }
}

impl NmeaEncodable for Sm3 {
    const SENTENCE_TYPE: &'static str = "SM3";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.status);
        w.u32(self.message_number);
        w.u32(self.les_sequence);
        w.u16(self.les_id);
        w.u8(self.ocean_region);
        w.u8(self.priority);
        w.u8(self.service);
        w.u8(self.presentation);
        w.u16(self.year);
        w.u8(self.month);
        w.u8(self.day);
        w.u8(self.hour);
        w.u8(self.minute);
        w.lat(self.latitude);
        w.char(self.ns);
        w.lon(self.longitude);
        w.char(self.ew);
        w.u32(self.radius);
        w.finish()
    }
}
