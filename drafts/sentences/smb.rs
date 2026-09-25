use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Smb {
    pub total_sentences: Option<u8>,
    pub sentence_number: Option<u8>,
    pub sequence_id: Option<u8>,
    pub message_number: Option<u32>,
    pub body: Option<String>,
}

impl Smb {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            total_sentences: r.u8(),
            sentence_number: r.u8(),
            sequence_id: r.u8(),
            message_number: r.u32(),
            body: r.string(),
        })
    }
}

impl NmeaEncodable for Smb {
    const SENTENCE_TYPE: &'static str = "SMB";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.total_sentences);
        w.u8(self.sentence_number);
        w.u8(self.sequence_id);
        w.u32(self.message_number);
        w.string(self.body.as_deref());
        w.finish()
    }
}
