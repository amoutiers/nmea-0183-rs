use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// TRL — AIS transmitter non-functioning log.
///
/// Wire: `total,entry,sequence,switch_off_date,switch_off_time,switch_on_date,switch_on_time,reason`
#[derive(Debug, Clone, PartialEq)]
pub struct Trl {
    /// Total number of log entries; the wire format permits values such as 10.
    pub total_log_entries: Option<u8>,
    /// Entry number within the non-functioning log.
    pub log_entry_number: Option<u8>,
    /// Textual sequential message identifier, preserving any leading zeroes.
    pub sequence_id: Option<String>,
    /// Date the transmitter was switched off, preserved in its wire format.
    pub switch_off_date: Option<String>,
    /// UTC time the transmitter was switched off, preserved in its wire format.
    pub switch_off_time: Option<String>,
    /// Date the transmitter was switched on, preserved in its wire format.
    pub switch_on_date: Option<String>,
    /// UTC time the transmitter was switched on, preserved in its wire format.
    pub switch_on_time: Option<String>,
    pub reason_code: Option<u8>,
}

impl Trl {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            total_log_entries: r.u8(),
            log_entry_number: r.u8(),
            sequence_id: r.string(),
            switch_off_date: r.string(),
            switch_off_time: r.string(),
            switch_on_date: r.string(),
            switch_on_time: r.string(),
            reason_code: r.u8(),
        })
    }
}

impl NmeaEncodable for Trl {
    const SENTENCE_TYPE: &str = "TRL";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.total_log_entries);
        w.u8(self.log_entry_number);
        w.string(self.sequence_id.as_deref());
        w.string(self.switch_off_date.as_deref());
        w.string(self.switch_off_time.as_deref());
        w.string(self.switch_on_date.as_deref());
        w.string(self.switch_on_time.as_deref());
        w.u8(self.reason_code);
        w.finish()
    }
}
