use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// VER: Equipment version information.
///
/// Each instance carries one part of a multipart message. Callers reassemble
/// parts using the multipart fields when required.
///
/// Wire: `total,sentence,type,vendor,uid,serial,model,software,hardware,sequence`
#[derive(Debug, Clone, PartialEq, Default)]
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
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
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
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn ver_empty() {
        let frame = parse_frame("$AIVER,,,,,,,,,,*49").expect("valid empty VER frame");
        assert_eq!(
            Ver::parse(&frame.fields),
            Ver {
                total_sentences: None,
                sentence_number: None,
                device_type: None,
                vendor_id: None,
                unique_identifier: None,
                manufacturer_serial_number: None,
                model_code: None,
                software_revision: None,
                hardware_revision: None,
                sequence_id: None,
            }
        );
    }

    #[test]
    fn ver_encode_roundtrip() {
        let original = Ver {
            total_sentences: Some(1),
            sentence_number: Some(1),
            device_type: Some("AI".into()),
            vendor_id: Some("OSG".into()),
            unique_identifier: Some("777888999".into()),
            manufacturer_serial_number: Some("1234567Q".into()),
            model_code: Some("ATA100".into()),
            software_revision: Some("00.2.16".into()),
            hardware_revision: Some("02.0.00".into()),
            sequence_id: Some("0".into()),
        };
        let sentence = original.to_sentence("AI").expect("encode VER");
        let frame = parse_frame(sentence.trim()).expect("re-parse VER");
        assert_eq!(Ver::parse(&frame.fields), original);
    }

    #[test]
    fn ver_values_ocean_signal() {
        let frame = parse_frame(
            "$AIVER,1,1,AI,OSG,777888999,1234567Q,ATA100,00.2.16,02.0.00,0*1F",
        )
        .expect("valid Ocean Signal ATA100 VER frame");
        assert_eq!(
            Ver::parse(&frame.fields),
            Ver {
                total_sentences: Some(1),
                sentence_number: Some(1),
                device_type: Some("AI".into()),
                vendor_id: Some("OSG".into()),
                unique_identifier: Some("777888999".into()),
                manufacturer_serial_number: Some("1234567Q".into()),
                model_code: Some("ATA100".into()),
                software_revision: Some("00.2.16".into()),
                hardware_revision: Some("02.0.00".into()),
                sequence_id: Some("0".into()),
            }
        );
    }
}
