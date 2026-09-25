use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// LCD — Loran-C Signal Data.
///
/// Wire: `gri,master_snr,master_ecd,s1_snr,s1_ecd,...,s5_snr,s5_ecd`
#[derive(Debug, Clone, PartialEq)]
pub struct Lcd {
    /// Loran-C chain group repetition interval, in tens of microseconds.
    pub gri: Option<u16>,
    /// Master-station relative signal-to-noise ratio.
    pub master_snr: Option<u16>,
    /// Master-station relative envelope-to-cycle difference.
    pub master_ecd: Option<i16>,
    /// First secondary-station relative signal-to-noise ratio.
    pub secondary_1_snr: Option<u16>,
    /// First secondary-station relative envelope-to-cycle difference.
    pub secondary_1_ecd: Option<i16>,
    /// Second secondary-station relative signal-to-noise ratio.
    pub secondary_2_snr: Option<u16>,
    /// Second secondary-station relative envelope-to-cycle difference.
    pub secondary_2_ecd: Option<i16>,
    /// Third secondary-station relative signal-to-noise ratio.
    pub secondary_3_snr: Option<u16>,
    /// Third secondary-station relative envelope-to-cycle difference.
    pub secondary_3_ecd: Option<i16>,
    /// Fourth secondary-station relative signal-to-noise ratio.
    pub secondary_4_snr: Option<u16>,
    /// Fourth secondary-station relative envelope-to-cycle difference.
    pub secondary_4_ecd: Option<i16>,
    /// Fifth secondary-station relative signal-to-noise ratio.
    pub secondary_5_snr: Option<u16>,
    /// Fifth secondary-station relative envelope-to-cycle difference.
    pub secondary_5_ecd: Option<i16>,
}

impl Lcd {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            gri: r.u16(),
            master_snr: r.u16(),
            master_ecd: r.i16(),
            secondary_1_snr: r.u16(),
            secondary_1_ecd: r.i16(),
            secondary_2_snr: r.u16(),
            secondary_2_ecd: r.i16(),
            secondary_3_snr: r.u16(),
            secondary_3_ecd: r.i16(),
            secondary_4_snr: r.u16(),
            secondary_4_ecd: r.i16(),
            secondary_5_snr: r.u16(),
            secondary_5_ecd: r.i16(),
        })
    }
}

impl NmeaEncodable for Lcd {
    const SENTENCE_TYPE: &str = "LCD";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u16(self.gri);
        w.u16(self.master_snr);
        w.i16(self.master_ecd);
        w.u16(self.secondary_1_snr);
        w.i16(self.secondary_1_ecd);
        w.u16(self.secondary_2_snr);
        w.i16(self.secondary_2_ecd);
        w.u16(self.secondary_3_snr);
        w.i16(self.secondary_3_ecd);
        w.u16(self.secondary_4_snr);
        w.i16(self.secondary_4_ecd);
        w.u16(self.secondary_5_snr);
        w.i16(self.secondary_5_ecd);
        w.finish()
    }
}
