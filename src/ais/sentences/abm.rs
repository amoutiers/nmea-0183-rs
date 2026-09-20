use super::field::{encode_char, encode_u8, encode_u32, read_char, read_string, read_u8, read_u32};

/// ABM — AIS addressed binary and safety-related message.
///
/// Source: go-nmea fixtures, derived from the Furuno FAR-15XX marine radar manual.
///
/// Wire: `num_frags,frag_num,msg_id,mmsi,channel,vdl_msg_num,payload,fill_bits`
///
/// Note: ABM sentences use the `!` prefix on the wire but field parsing
/// is identical to `$`-prefixed sentences at the field layer.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Abm {
    /// Total number of sentences needed.
    pub num_frags: Option<u8>,
    /// Fragment number of this sentence.
    pub frag_num: Option<u8>,
    /// Sequential message identifier.
    pub msg_id: Option<u8>,
    /// Destination MMSI.
    pub mmsi: Option<u32>,
    /// AIS channel selection (0=no broadcast, 1=A, 2=B, 3=both).
    pub channel: Option<char>,
    /// VDL message number.
    pub vdl_msg_num: Option<u8>,
    /// Encoded AIS payload (armored ASCII).
    pub payload: Option<String>,
    /// Number of fill bits (0–5).
    pub fill_bits: Option<u8>,
}

impl Abm {
    pub const SENTENCE_TYPE: &'static str = "ABM";

    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut idx = 0;
        let num_frags = read_u8(fields, &mut idx);
        let frag_num = read_u8(fields, &mut idx);
        let msg_id = read_u8(fields, &mut idx);
        let mmsi = read_u32(fields, &mut idx);
        let channel = read_char(fields, &mut idx);
        let vdl_msg_num = read_u8(fields, &mut idx);
        let payload = read_string(fields, &mut idx);
        let fill_bits = read_u8(fields, &mut idx);
        Self {
            num_frags,
            frag_num,
            msg_id,
            mmsi,
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
            encode_u32(self.mmsi),
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
    fn abm_empty() {
        let s = Abm {
            num_frags: None,
            frag_num: None,
            msg_id: None,
            mmsi: None,
            channel: None,
            vdl_msg_num: None,
            payload: None,
            fill_bits: None,
        }
        .to_sentence("AI")
        .expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let a = Abm::parse(&f.fields);
        assert!(a.num_frags.is_none());
        assert!(a.mmsi.is_none());
        assert!(a.payload.is_none());
    }

    #[test]
    fn abm_encode_roundtrip() {
        let original = Abm {
            num_frags: Some(1),
            frag_num: Some(1),
            msg_id: Some(0),
            mmsi: Some(123456789),
            channel: Some('1'),
            vdl_msg_num: Some(6),
            payload: Some("test".to_string()),
            fill_bits: Some(0),
        };
        let sentence = original.to_sentence("AI").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Abm::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn abm_encode_rejects_invalid_payload() {
        let abm = Abm {
            num_frags: Some(1),
            frag_num: Some(1),
            msg_id: Some(0),
            mmsi: Some(123456789),
            channel: Some('1'),
            vdl_msg_num: Some(6),
            payload: Some("invalid,payload".to_string()),
            fill_bits: Some(0),
        };

        assert_eq!(
            abm.encode(),
            Err(crate::EncodeError::InvalidFieldCharacter(','))
        );
    }

    #[test]
    fn abm_aiabm_gonmea() {
        let frame = parse_frame("!AIABM,26,2,1,3381581370,3,8,177KQJ5000G?tO`K>RA1wUbN0TKH,0*02")
            .expect("valid");
        let a = Abm::parse(&frame.fields);
        assert_eq!(a.num_frags, Some(26));
        assert_eq!(a.frag_num, Some(2));
        assert_eq!(a.msg_id, Some(1));
        assert_eq!(a.mmsi, Some(3381581370));
        assert_eq!(a.channel, Some('3'));
        assert_eq!(a.vdl_msg_num, Some(8));
        assert_eq!(a.payload.as_deref(), Some("177KQJ5000G?tO`K>RA1wUbN0TKH"));
        assert_eq!(a.fill_bits, Some(0));
    }
}
