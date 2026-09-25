use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ACS — AIS channel management information source.
///
/// Wire: `sequence,mmsi,time,day,month,year`
#[derive(Debug, Clone, PartialEq)]
pub struct Acs {
    /// Sequence number binding this sentence to the preceding ACA sentence.
    pub sequence: Option<u8>,
    /// MMSI of the information originator, retained as text to preserve leading zeroes.
    pub mmsi: Option<String>,
    /// UTC receipt time, retained as text to preserve its wire representation.
    pub time: Option<String>,
    /// UTC day, retained as text to preserve leading zeroes.
    pub day: Option<String>,
    /// UTC month, retained as text to preserve leading zeroes.
    pub month: Option<String>,
    /// UTC year.
    pub year: Option<String>,
}

impl Acs {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            sequence: r.u8(),
            mmsi: r.string(),
            time: r.string(),
            day: r.string(),
            month: r.string(),
            year: r.string(),
        })
    }
}

impl NmeaEncodable for Acs {
    const SENTENCE_TYPE: &str = "ACS";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.sequence);
        w.string(self.mmsi.as_deref());
        w.string(self.time.as_deref());
        w.string(self.day.as_deref());
        w.string(self.month.as_deref());
        w.string(self.year.as_deref());
        w.finish()
    }
}
