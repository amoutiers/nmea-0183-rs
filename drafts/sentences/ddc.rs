use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// DDC — Display dimming control.
///
/// Wire: `preset,brightness,palette,sentence_status[,command_mode]`
#[derive(Debug, Clone, PartialEq)]
pub struct Ddc {
    /// Display dimming preset (D = day, K = dusk, N = night, O = off).
    pub preset: Option<char>,
    /// Display brightness percentage (00 to 99).
    pub brightness: Option<u8>,
    /// Colour palette (D = day, K = dusk, N = night, O = off).
    pub palette: Option<char>,
    /// Sentence status (R = report, C = configuration command).
    pub sentence_status: Option<char>,
    /// Command mode (O = operational, P = preset); absent in the legacy format.
    pub command_mode: Option<char>,
}

impl Ddc {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let preset = r.char();
        let brightness = r.u8();
        let palette = r.char();
        let sentence_status = r.char();
        let command_mode = if fields.len() > 4 { r.char() } else { None };
        Some(Self {
            preset,
            brightness,
            palette,
            sentence_status,
            command_mode,
        })
    }
}

impl NmeaEncodable for Ddc {
    const SENTENCE_TYPE: &str = "DDC";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.preset);
        let brightness = self.brightness.map(|value| format!("{value:02}"));
        w.string(brightness.as_deref());
        w.char(self.palette);
        w.char(self.sentence_status);
        if self.command_mode.is_some() {
            w.char(self.command_mode);
        }
        w.finish()
    }
}
