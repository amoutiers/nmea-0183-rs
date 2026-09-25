use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GLC — Loran-C Position.
///
/// Wire: `gri,master_toa,master_status,td1,td1_status,...,td5,td5_status`
#[derive(Debug, Clone, PartialEq)]
pub struct Glc {
    /// Loran-C chain group repetition interval.
    pub gri: Option<u16>,
    /// Master-station time of arrival in microseconds.
    pub master_toa: Option<f32>,
    /// Master-station signal status.
    pub master_status: Option<char>,
    /// First secondary-station time difference in microseconds.
    pub td1: Option<f32>,
    /// First secondary-station signal status.
    pub td1_status: Option<char>,
    /// Second secondary-station time difference in microseconds.
    pub td2: Option<f32>,
    /// Second secondary-station signal status.
    pub td2_status: Option<char>,
    /// Third secondary-station time difference in microseconds.
    pub td3: Option<f32>,
    /// Third secondary-station signal status.
    pub td3_status: Option<char>,
    /// Fourth secondary-station time difference in microseconds.
    pub td4: Option<f32>,
    /// Fourth secondary-station signal status.
    pub td4_status: Option<char>,
    /// Fifth secondary-station time difference in microseconds.
    pub td5: Option<f32>,
    /// Fifth secondary-station signal status.
    pub td5_status: Option<char>,
}

impl Glc {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            gri: r.u16(),
            master_toa: r.f32(),
            master_status: r.char(),
            td1: r.f32(),
            td1_status: r.char(),
            td2: r.f32(),
            td2_status: r.char(),
            td3: r.f32(),
            td3_status: r.char(),
            td4: r.f32(),
            td4_status: r.char(),
            td5: r.f32(),
            td5_status: r.char(),
        })
    }
}

impl NmeaEncodable for Glc {
    const SENTENCE_TYPE: &str = "GLC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u16(self.gri);
        w.f32(self.master_toa);
        w.char(self.master_status);
        w.f32(self.td1);
        w.char(self.td1_status);
        w.f32(self.td2);
        w.char(self.td2_status);
        w.f32(self.td3);
        w.char(self.td3_status);
        w.f32(self.td4);
        w.char(self.td4_status);
        w.f32(self.td5);
        w.char(self.td5_status);
        w.finish()
    }
}
