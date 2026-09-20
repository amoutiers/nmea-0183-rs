use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// TXT — Text Transmission.
///
/// Wire: `num_msg,msg_num,msg_type,text`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Txt {
    /// Total number of messages in this sequence.
    pub num_msg: Option<u8>,
    /// Message number within the sequence (1-based).
    pub msg_num: Option<u8>,
    /// Message type identifier.
    pub msg_type: Option<u8>,
    /// Text message content.
    pub text: Option<String>,
}

impl Txt {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        let num_msg = r.u8();
        let msg_num = r.u8();
        let msg_type = r.u8();
        // Text is the final field but may itself contain commas, which parse_frame
        // split into separate fields; rejoin everything from index 3 onward.
        let text = if fields.len() > 3 {
            let joined = fields[3..].join(",");
            if joined.is_empty() { None } else { Some(joined) }
        } else {
            None
        };
        Self { num_msg, msg_num, msg_type, text }
    }
}

impl NmeaEncodable for Txt {
    const SENTENCE_TYPE: &str = "TXT";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.num_msg);
        w.u8(self.msg_num);
        w.u8(self.msg_type);
        w.string(self.text.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn txt_empty() {
        let f = Txt {
            num_msg: None,
            msg_num: None,
            msg_type: None,
            text: None,
        }
        .to_sentence("GP").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let t = Txt::parse(&frame.fields);
        assert!(t.num_msg.is_none());
        assert!(t.msg_num.is_none());
        assert!(t.msg_type.is_none());
        assert!(t.text.is_none());
    }

    #[test]
    fn txt_encode_roundtrip() {
        let original = Txt {
            num_msg: Some(1),
            msg_num: Some(1),
            msg_type: Some(2),
            text: Some("u-blox ag - www.u-blox.com".to_string()),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Txt::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn txt_full_gonmea() {
        let frame =
            parse_frame("$GPTXT,01,01,02,u-blox ag - www.u-blox.com*50").expect("valid TXT frame");
        let txt = Txt::parse(&frame.fields);
        assert_eq!(txt.num_msg, Some(1));
        assert_eq!(txt.msg_num, Some(1));
        assert_eq!(txt.msg_type, Some(2));
        assert_eq!(txt.text, Some("u-blox ag - www.u-blox.com".to_string()));
    }

    #[test]
    fn txt_text_with_embedded_comma() {
        // On the wire "...,02,Hello, World" -> parse_frame splits into ["Hello"," World"];
        // Txt::parse must rejoin them into the full text.
        let t = Txt::parse(&["01", "01", "02", "Hello", " World"]);
        assert_eq!(t.text, Some("Hello, World".to_string()));
    }
}
