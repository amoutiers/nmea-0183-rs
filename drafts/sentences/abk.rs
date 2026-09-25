use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ABK — AIS addressed and binary broadcast acknowledgement.
///
/// Wire: `mmsi,channel,message_id,sequence,ack_type`
#[derive(Debug, Clone, PartialEq)]
pub struct Abk {
    /// MMSI of the addressed AIS unit, retained as text to preserve leading zeroes.
    pub mmsi: Option<String>,
    /// AIS channel of reception.
    pub channel: Option<char>,
    /// ITU-R M.1371 message identifier.
    pub message_id: Option<u8>,
    /// Message sequence number, retained as text to preserve its wire representation.
    pub sequence: Option<String>,
    /// Type of acknowledgement.
    pub ack_type: Option<u8>,
}

impl Abk {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            mmsi: r.string(),
            channel: r.char(),
            message_id: r.u8(),
            sequence: r.string(),
            ack_type: r.u8(),
        })
    }
}

impl NmeaEncodable for Abk {
    const SENTENCE_TYPE: &str = "ABK";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.mmsi.as_deref());
        w.char(self.channel);
        w.u8(self.message_id);
        w.string(self.sequence.as_deref());
        w.u8(self.ack_type);
        w.finish()
    }
}
