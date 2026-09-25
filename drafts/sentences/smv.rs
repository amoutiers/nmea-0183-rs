use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// SMV — SafetyNET vessel in distress.
///
/// Each instance carries one part of a multipart message; callers reassemble
/// parts using the multipart fields when required.
///
/// Wire: `total,sentence,sequence,message,mmsi,name,lat,N/S,lon,E/W,year,month,day,hour,minute,status`
#[derive(Debug, Clone, PartialEq)]
pub struct Smv {
    /// Number of sentences in the message.
    pub total_sentences: Option<u8>,
    /// This sentence's one-based number; absent for a single-sentence message.
    pub sentence_number: Option<u8>,
    /// Identifier for associating multipart sentences.
    pub sequential_message_id: Option<u8>,
    /// Rescue Coordination Centre unique message number.
    pub message_number: Option<u32>,
    /// Vessel MMSI, retained as text to preserve leading zeroes.
    pub mmsi: Option<String>,
    /// Vessel name.
    pub vessel_name: Option<String>,
    /// Latitude in NMEA format (DDMM.MMMM).
    pub latitude: Option<f64>,
    /// Latitude hemisphere: `N` or `S`.
    pub ns: Option<char>,
    /// Longitude in NMEA format (DDDMM.MMMM).
    pub longitude: Option<f64>,
    /// Longitude hemisphere: `E` or `W`.
    pub ew: Option<char>,
    /// UTC year, retained as text to preserve its four-digit wire form.
    pub year: Option<String>,
    /// UTC month, retained as text to preserve its two-digit wire form.
    pub month: Option<String>,
    /// UTC day, retained as text to preserve its two-digit wire form.
    pub day: Option<String>,
    /// UTC hour, retained as text to preserve its two-digit wire form.
    pub hour: Option<String>,
    /// UTC minute, retained as text to preserve its two-digit wire form.
    pub minute: Option<String>,
    /// Distress status: `D` = active, `C` = cancelled.
    pub status: Option<char>,
}

impl Smv {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            total_sentences: r.u8(),
            sentence_number: r.u8(),
            sequential_message_id: r.u8(),
            message_number: r.u32(),
            mmsi: r.string(),
            vessel_name: r.string(),
            latitude: r.f64(),
            ns: r.char(),
            longitude: r.f64(),
            ew: r.char(),
            year: r.string(),
            month: r.string(),
            day: r.string(),
            hour: r.string(),
            minute: r.string(),
            status: r.char(),
        })
    }
}

impl NmeaEncodable for Smv {
    const SENTENCE_TYPE: &str = "SMV";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.total_sentences);
        w.u8(self.sentence_number);
        w.u8(self.sequential_message_id);
        w.u32(self.message_number);
        w.string(self.mmsi.as_deref());
        w.string(self.vessel_name.as_deref());
        w.lat(self.latitude);
        w.char(self.ns);
        w.lon(self.longitude);
        w.char(self.ew);
        w.string(self.year.as_deref());
        w.string(self.month.as_deref());
        w.string(self.day.as_deref());
        w.string(self.hour.as_deref());
        w.string(self.minute.as_deref());
        w.char(self.status);
        w.finish()
    }
}
