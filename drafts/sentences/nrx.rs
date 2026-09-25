use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Nrx {
    pub total_sentences: Option<u8>,
    pub sentence_number: Option<u8>,
    pub sequence_id: Option<u8>,
    pub message_code: Option<String>,
    pub frequency_table: Option<char>,
    pub time: Option<String>,
    pub day: Option<u8>,
    pub month: Option<u8>,
    pub year: Option<u16>,
    pub total_characters: Option<u16>,
    pub bad_characters: Option<u16>,
    pub status: Option<char>,
    pub body: Option<String>,
}

impl Nrx {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            total_sentences: r.u8(),
            sentence_number: r.u8(),
            sequence_id: r.u8(),
            message_code: r.string(),
            frequency_table: r.char(),
            time: r.string(),
            day: r.u8(),
            month: r.u8(),
            year: r.u16(),
            total_characters: r.u16(),
            bad_characters: r.u16(),
            status: r.char(),
            body: r.string(),
        })
    }
}

impl NmeaEncodable for Nrx {
    const SENTENCE_TYPE: &'static str = "NRX";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.total_sentences);
        w.u8(self.sentence_number);
        w.u8(self.sequence_id);
        w.string(self.message_code.as_deref());
        w.char(self.frequency_table);
        w.string(self.time.as_deref());
        w.u8(self.day);
        w.u8(self.month);
        w.u16(self.year);
        w.u16(self.total_characters);
        w.u16(self.bad_characters);
        w.char(self.status);
        w.string(self.body.as_deref());
        w.finish()
    }
}
