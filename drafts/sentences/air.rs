use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// AIR — AIS interrogation request (NMEA 0183 3.01).
///
/// Wire: `station_1_mmsi,message_1_1,subsection_1_1,message_1_2,subsection_1_2,station_2_mmsi,message_2_1,subsection_2_1`
#[derive(Debug, Clone, PartialEq)]
pub struct Air {
    /// MMSI of the first interrogated station, retained as text to preserve leading zeroes.
    pub station_1_mmsi: Option<String>,
    /// First requested ITU-R M.1371 message from station 1.
    pub station_1_message_1: Option<String>,
    /// Optional sub-section of the first requested message from station 1.
    pub station_1_message_1_subsection: Option<String>,
    /// Second requested ITU-R M.1371 message from station 1.
    pub station_1_message_2: Option<String>,
    /// Optional sub-section of the second requested message from station 1.
    pub station_1_message_2_subsection: Option<String>,
    /// MMSI of the second interrogated station, retained as text to preserve leading zeroes.
    pub station_2_mmsi: Option<String>,
    /// Requested ITU-R M.1371 message from station 2.
    pub station_2_message_1: Option<String>,
    /// Optional sub-section of the requested message from station 2.
    pub station_2_message_1_subsection: Option<String>,
}

impl Air {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            station_1_mmsi: r.string(),
            station_1_message_1: r.string(),
            station_1_message_1_subsection: r.string(),
            station_1_message_2: r.string(),
            station_1_message_2_subsection: r.string(),
            station_2_mmsi: r.string(),
            station_2_message_1: r.string(),
            station_2_message_1_subsection: r.string(),
        })
    }
}

impl NmeaEncodable for Air {
    const SENTENCE_TYPE: &str = "AIR";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.station_1_mmsi.as_deref());
        w.string(self.station_1_message_1.as_deref());
        w.string(self.station_1_message_1_subsection.as_deref());
        w.string(self.station_1_message_2.as_deref());
        w.string(self.station_1_message_2_subsection.as_deref());
        w.string(self.station_2_mmsi.as_deref());
        w.string(self.station_2_message_1.as_deref());
        w.string(self.station_2_message_1_subsection.as_deref());
        w.finish()
    }
}
