use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VER — Equipment version information.
///
/// Each instance carries one part of a multipart message; callers reassemble
/// parts using the multipart fields when required.
///
/// Wire: `total,sentence,type,vendor,uid,serial,model,software,hardware,sequence`
#[derive(Debug, Clone, PartialEq)]
pub struct Ver {
    /// Number of sentences in the version-information message.
    pub total_sentences: Option<u8>,
    /// This sentence's one-based number.
    pub sentence_number: Option<u8>,
    /// Equipment type, such as `AI` for AIS.
    pub device_type: Option<String>,
    /// Manufacturer's three-character vendor ID.
    pub vendor_id: Option<String>,
    /// Equipment unique identifier, retained as text to preserve leading zeroes.
    pub unique_identifier: Option<String>,
    /// Manufacturer serial number, retained verbatim.
    pub manufacturer_serial_number: Option<String>,
    /// Manufacturer product or model code.
    pub model_code: Option<String>,
    /// Software revision, retained verbatim.
    pub software_revision: Option<String>,
    /// Hardware revision, retained verbatim.
    pub hardware_revision: Option<String>,
    /// Sequential message identifier, retained as text to preserve its wire representation.
    pub sequence_id: Option<String>,
}

impl Ver {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            total_sentences: r.u8(),
            sentence_number: r.u8(),
            device_type: r.string(),
            vendor_id: r.string(),
            unique_identifier: r.string(),
            manufacturer_serial_number: r.string(),
            model_code: r.string(),
            software_revision: r.string(),
            hardware_revision: r.string(),
            sequence_id: r.string(),
        })
    }
}

impl NmeaEncodable for Ver {
    const SENTENCE_TYPE: &str = "VER";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.total_sentences);
        w.u8(self.sentence_number);
        w.string(self.device_type.as_deref());
        w.string(self.vendor_id.as_deref());
        w.string(self.unique_identifier.as_deref());
        w.string(self.manufacturer_serial_number.as_deref());
        w.string(self.model_code.as_deref());
        w.string(self.software_revision.as_deref());
        w.string(self.hardware_revision.as_deref());
        w.string(self.sequence_id.as_deref());
        w.finish()
    }
}
