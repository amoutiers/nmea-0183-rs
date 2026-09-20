use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ALF — Alert Sentence.
///
/// Field layout follows NMEA 0183 ALF and the pynmeagps sentence definitions.
///
/// Wire: `num_frags,frag_num,msg_id,time,category,priority,state,manufacturer,alert_id,instance,revision,escalation,text`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Alf {
    /// Total number of sentences (fragments) needed for this message.
    pub num_frags: Option<u8>,
    /// Fragment number of this sentence.
    pub frag_num: Option<u8>,
    /// Sequential message identifier.
    pub msg_id: Option<u8>,
    /// UTC time of alert (hhmmss).
    pub time: Option<String>,
    /// Alert category (A=alarm, W=warning, C=caution).
    pub category: Option<char>,
    /// Alert priority (B=bridge, N=navigation, S=safety).
    pub priority: Option<char>,
    /// Alert state (A=active, S=silenced, U=unacknowledged, N=normal).
    pub state: Option<char>,
    /// Manufacturer mnemonic code.
    pub manufacturer: Option<String>,
    /// Alert identifier.
    pub alert_id: Option<String>,
    /// Alert instance number.
    pub instance: Option<u8>,
    /// Alert revision counter.
    pub revision: Option<u8>,
    /// Escalation counter.
    pub escalation: Option<u8>,
    /// Alert text.
    pub text: Option<String>,
}

impl Alf {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let num_frags = r.u8();
        let frag_num = r.u8();
        let msg_id = r.u8();
        let time = r.string();
        let category = r.char();
        let priority = r.char();
        let state = r.char();
        let manufacturer = r.string();
        let alert_id = r.string();
        let instance = r.u8();
        let revision = r.u8();
        let escalation = r.u8();
        let text = r.string();
        Some(Self {
            num_frags,
            frag_num,
            msg_id,
            time,
            category,
            priority,
            state,
            manufacturer,
            alert_id,
            instance,
            revision,
            escalation,
            text,
        })
    }
}

impl NmeaEncodable for Alf {
    const SENTENCE_TYPE: &str = "ALF";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.num_frags);
        w.u8(self.frag_num);
        w.u8(self.msg_id);
        w.string(self.time.as_deref());
        w.char(self.category);
        w.char(self.priority);
        w.char(self.state);
        w.string(self.manufacturer.as_deref());
        w.string(self.alert_id.as_deref());
        w.u8(self.instance);
        w.u8(self.revision);
        w.u8(self.escalation);
        w.string(self.text.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn alf_empty() {
        let s = Alf {
            num_frags: None,
            frag_num: None,
            msg_id: None,
            time: None,
            category: None,
            priority: None,
            state: None,
            manufacturer: None,
            alert_id: None,
            instance: None,
            revision: None,
            escalation: None,
            text: None,
        }
        .to_sentence("VD")
        .expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let a = Alf::parse(&f.fields).expect("parse");
        assert!(a.num_frags.is_none());
        assert!(a.text.is_none());
    }

    #[test]
    fn alf_encode_roundtrip() {
        let original = Alf {
            num_frags: Some(1),
            frag_num: Some(0),
            msg_id: Some(1),
            time: Some("220516".to_string()),
            category: Some('B'),
            priority: Some('A'),
            state: Some('S'),
            manufacturer: Some("SAL".to_string()),
            alert_id: Some("001".to_string()),
            instance: Some(1),
            revision: Some(2),
            escalation: Some(0),
            text: Some("My alarm".to_string()),
        };
        let sentence = original.to_sentence("VD").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Alf::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn alf_vdalf_gonmea() {
        let f = parse_frame("$VDALF,1,0,1,220516,B,A,S,SAL,001,1,2,0,My alarm*2C")
            .expect("valid ALF");
        let a = Alf::parse(&f.fields).expect("parse ALF");
        assert_eq!(a.num_frags, Some(1));
        assert_eq!(a.frag_num, Some(0));
        assert_eq!(a.msg_id, Some(1));
        assert_eq!(a.time, Some("220516".to_string()));
        assert_eq!(a.category, Some('B'));
        assert_eq!(a.priority, Some('A'));
        assert_eq!(a.state, Some('S'));
        assert_eq!(a.manufacturer, Some("SAL".to_string()));
        assert_eq!(a.alert_id, Some("001".to_string()));
        assert_eq!(a.instance, Some(1));
        assert_eq!(a.revision, Some(2));
        assert_eq!(a.escalation, Some(0));
        assert_eq!(a.text, Some("My alarm".to_string()));
    }
}
