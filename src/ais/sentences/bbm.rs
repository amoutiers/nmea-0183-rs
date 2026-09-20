use super::field::{encode_char, encode_u8, read_char, read_string, read_u8};

/// BBM — AIS broadcast binary message.
///
/// Source: go-nmea fixtures, derived from the Furuno FAR-15XX marine radar manual.
///
/// Wire: `num_frags,frag_num,msg_id,channel,vdl_msg_num,payload,fill_bits`
///
/// Note: BBM sentences use the `!` prefix on the wire but field parsing
/// is identical to `$`-prefixed sentences at the field layer.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Bbm {
    /// Total number of sentences needed.
    pub num_frags: Option<u8>,
    /// Fragment number of this sentence.
    pub frag_num: Option<u8>,
    /// Sequential message identifier.
    pub msg_id: Option<u8>,
    /// AIS channel selection (0=no broadcast, 1=A, 2=B, 3=both).
    pub channel: Option<char>,
    /// VDL message number.
    pub vdl_msg_num: Option<u8>,
    /// Encoded AIS payload (armored ASCII).
    pub payload: Option<String>,
    /// Number of fill bits (0-5).
    pub fill_bits: Option<u8>,
}

impl Bbm {
    pub const SENTENCE_TYPE: &'static str = "BBM";

    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut idx = 0;
        let num_frags = read_u8(fields, &mut idx);
        let frag_num = read_u8(fields, &mut idx);
        let msg_id = read_u8(fields, &mut idx);
        let channel = read_char(fields, &mut idx);
        let vdl_msg_num = read_u8(fields, &mut idx);
        let payload = read_string(fields, &mut idx);
        let fill_bits = read_u8(fields, &mut idx);
        Self {
            num_frags,
            frag_num,
            msg_id,
            channel,
            vdl_msg_num,
            payload,
            fill_bits,
        }
    }

    pub fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let fields = vec![
            encode_u8(self.num_frags),
            encode_u8(self.frag_num),
            encode_u8(self.msg_id),
            encode_char(self.channel),
            encode_u8(self.vdl_msg_num),
            self.payload.clone().unwrap_or_default(),
            encode_u8(self.fill_bits),
        ];
        for field in &fields {
            crate::frame::validate_field(field)?;
        }
        Ok(fields)
    }

    pub fn to_sentence(&self, talker: &str) -> Result<String, crate::EncodeError> {
        let fields = self.encode()?;
        let field_refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        crate::encode_frame('!', talker, Self::SENTENCE_TYPE, &field_refs)
    }

    /// Encode and validate the strict frame envelope, not the application fields.
    pub fn to_sentence_strict(&self, talker: &str) -> Result<String, crate::StrictEncodeError> {
        let sentence = self.to_sentence(talker)?;
        crate::validate_sentence(&sentence)?;
        Ok(sentence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn bbm_empty() {
        let s = Bbm {
            num_frags: None,
            frag_num: None,
            msg_id: None,
            channel: None,
            vdl_msg_num: None,
            payload: None,
            fill_bits: None,
        }
        .to_sentence("AI")
        .expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let b = Bbm::parse(&f.fields);
        assert!(b.num_frags.is_none());
        assert!(b.channel.is_none());
        assert!(b.payload.is_none());
    }

    #[test]
    fn bbm_encode_roundtrip() {
        let original = Bbm {
            num_frags: Some(1),
            frag_num: Some(1),
            msg_id: Some(0),
            channel: Some('A'),
            vdl_msg_num: Some(6),
            payload: Some("test".to_string()),
            fill_bits: Some(0),
        };
        let sentence = original.to_sentence("AI").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Bbm::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn bbm_encode_rejects_invalid_payload() {
        let bbm = Bbm {
            num_frags: Some(1),
            frag_num: Some(1),
            msg_id: Some(0),
            channel: Some('A'),
            vdl_msg_num: Some(6),
            payload: Some("invalid*payload".to_string()),
            fill_bits: Some(0),
        };

        assert_eq!(
            bbm.encode(),
            Err(crate::EncodeError::InvalidFieldCharacter('*'))
        );
    }

    #[test]
    fn bbm_aibbm_gonmea() {
        let frame =
            parse_frame("!AIBBM,26,2,1,3,8,177KQJ5000G?tO`K>RA1wUbN0TKH,0*2C").expect("valid");
        let b = Bbm::parse(&frame.fields);
        assert_eq!(b.num_frags, Some(26));
        assert_eq!(b.frag_num, Some(2));
        assert_eq!(b.msg_id, Some(1));
        assert_eq!(b.channel, Some('3'));
        assert_eq!(b.vdl_msg_num, Some(8));
        assert_eq!(b.payload.as_deref(), Some("177KQJ5000G?tO`K>RA1wUbN0TKH"));
        assert_eq!(b.fill_bits, Some(0));
    }
}
