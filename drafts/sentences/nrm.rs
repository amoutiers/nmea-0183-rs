use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

#[derive(Debug, Clone, PartialEq)]
pub struct Nrm {
    pub function: Option<char>,
    pub frequency_table: Option<char>,
    pub transmit_coverage_mask: Option<String>,
    pub message_type_mask: Option<String>,
    pub status: Option<char>,
}

impl Nrm {
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            function: r.char(),
            frequency_table: r.char(),
            transmit_coverage_mask: r.string(),
            message_type_mask: r.string(),
            status: r.char(),
        })
    }
}

impl NmeaEncodable for Nrm {
    const SENTENCE_TYPE: &'static str = "NRM";
    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.function);
        w.char(self.frequency_table);
        w.string(self.transmit_coverage_mask.as_deref());
        w.string(self.message_type_mask.as_deref());
        w.char(self.status);
        w.finish()
    }
}
