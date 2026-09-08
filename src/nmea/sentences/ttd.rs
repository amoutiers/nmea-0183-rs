use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// TTD — Tracked Target Data.
///
/// Wire: `!**TTD,num_frags,frag_num,msg_id,payload,fill_bits`
///
/// Note: TTD uses hex-encoded fragment counts (e.g. "1A"). These are stored
/// as strings to preserve the original encoding. The `msg_id` is a u8.
#[derive(Debug, Clone, PartialEq)]
pub struct Ttd {
    /// Total number of sentences (may be hex string).
    pub num_frags: Option<String>,
    /// Fragment number (may be hex string).
    pub frag_num: Option<String>,
    /// Sequential message identifier.
    pub msg_id: Option<u8>,
    /// Encoded payload (armored ASCII).
    pub payload: Option<String>,
    /// Number of fill bits (0–5).
    pub fill_bits: Option<u8>,
}

impl Ttd {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let num_frags = r.string();
        let frag_num = r.string();
        let msg_id = r.u8();
        let payload = r.string();
        let fill_bits = r.u8();
        Some(Self {
            num_frags,
            frag_num,
            msg_id,
            payload,
            fill_bits,
        })
    }
}

impl NmeaEncodable for Ttd {
    const PREFIX: char = '!';
    const SENTENCE_TYPE: &str = "TTD";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.num_frags.as_deref());
        w.string(self.frag_num.as_deref());
        w.u8(self.msg_id);
        w.string(self.payload.as_deref());
        w.u8(self.fill_bits);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn ttd_empty() {
        let s = Ttd {
            num_frags: None,
            frag_num: None,
            msg_id: None,
            payload: None,
            fill_bits: None,
        }
        .to_sentence("**").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let t = Ttd::parse(&f.fields).expect("parse");
        assert!(t.num_frags.is_none());
        assert!(t.payload.is_none());
        assert!(t.fill_bits.is_none());
    }

    #[test]
    fn ttd_encode_roundtrip() {
        let original = Ttd {
            num_frags: Some("01".to_string()),
            frag_num: Some("01".to_string()),
            msg_id: Some(1),
            payload: Some("testpayload".to_string()),
            fill_bits: Some(0),
        };
        let sentence = original.to_sentence("**").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Ttd::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn ttd_rattd_gonmea() {
        let original = Ttd {
            num_frags: Some("1A".to_string()),
            frag_num: Some("01".to_string()),
            msg_id: Some(1),
            payload: Some("trackdata".to_string()),
            fill_bits: Some(0),
        };
        let sentence = original.to_sentence("**").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let t = Ttd::parse(&frame.fields).expect("parse TTD");
        assert_eq!(t.num_frags, Some("1A".to_string()));
        assert_eq!(t.frag_num, Some("01".to_string()));
        assert_eq!(t.msg_id, Some(1));
        assert_eq!(t.payload, Some("trackdata".to_string()));
        assert_eq!(t.fill_bits, Some(0));
    }
}
