use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// APB — Autopilot Sentence B.
///
/// Wire: `lcgwarn,lccwarn,ctrkerr,dirs,ctrkunit,aalmcirc,aalmperp,bear_o2d,bear_o2d_type,wpt,bear_dest,bear_dest_type,bear_steer,bear_steer_type,mode`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Apb {
    /// Loran-C general warning ('A' = OK, 'V' = warning).
    pub lcgwarn: Option<char>,
    /// Loran-C cycle lock warning ('A' = OK, 'V' = warning).
    pub lccwarn: Option<char>,
    /// Cross-track error magnitude in nautical miles.
    pub ctrkerr: Option<f32>,
    /// Direction to steer ('L' = left, 'R' = right).
    pub dirs: Option<char>,
    /// Cross-track error unit ('N' = nautical miles).
    pub ctrkunit: Option<char>,
    /// Arrival circle entered ('A' = entered, 'V' = not entered).
    pub aalmcirc: Option<char>,
    /// Perpendicular passed ('A' = passed, 'V' = not passed).
    pub aalmperp: Option<char>,
    /// Bearing origin to destination in degrees.
    pub bear_o2d: Option<f32>,
    /// Bearing origin to destination type ('T' = true, 'M' = magnetic).
    pub bear_o2d_type: Option<char>,
    /// Destination waypoint identifier.
    pub wpt: Option<String>,
    /// Bearing present position to destination in degrees.
    pub bear_dest: Option<f32>,
    /// Bearing to destination type ('T' = true, 'M' = magnetic).
    pub bear_dest_type: Option<char>,
    /// Heading to steer to destination in degrees.
    pub bear_steer: Option<f32>,
    /// Heading to steer type ('T' = true, 'M' = magnetic).
    pub bear_steer_type: Option<char>,
    /// Mode indicator ('A' = autonomous, 'D' = differential, etc.).
    pub mode: Option<char>,
}

impl Apb {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            lcgwarn: r.char(),
            lccwarn: r.char(),
            ctrkerr: r.f32(),
            dirs: r.char(),
            ctrkunit: r.char(),
            aalmcirc: r.char(),
            aalmperp: r.char(),
            bear_o2d: r.f32(),
            bear_o2d_type: r.char(),
            wpt: r.string(),
            bear_dest: r.f32(),
            bear_dest_type: r.char(),
            bear_steer: r.f32(),
            bear_steer_type: r.char(),
            mode: r.char(),
        })
    }
}

impl NmeaEncodable for Apb {
    const SENTENCE_TYPE: &str = "APB";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.lcgwarn);
        w.char(self.lccwarn);
        w.f32(self.ctrkerr);
        w.char(self.dirs);
        w.char(self.ctrkunit);
        w.char(self.aalmcirc);
        w.char(self.aalmperp);
        w.f32(self.bear_o2d);
        w.char(self.bear_o2d_type);
        w.string(self.wpt.as_deref());
        w.f32(self.bear_dest);
        w.char(self.bear_dest_type);
        w.f32(self.bear_steer);
        w.char(self.bear_steer_type);
        w.char(self.mode);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn apb_empty() {
        let f = Apb {
            lcgwarn: None,
            lccwarn: None,
            ctrkerr: None,
            dirs: None,
            ctrkunit: None,
            aalmcirc: None,
            aalmperp: None,
            bear_o2d: None,
            bear_o2d_type: None,
            wpt: None,
            bear_dest: None,
            bear_dest_type: None,
            bear_steer: None,
            bear_steer_type: None,
            mode: None,
        }
        .to_sentence("GP").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let a = Apb::parse(&frame.fields).expect("parse");
        assert!(a.lcgwarn.is_none());
        assert!(a.wpt.is_none());
        assert!(a.mode.is_none());
    }

    #[test]
    fn apb_encode_roundtrip() {
        let original = Apb {
            lcgwarn: Some('A'),
            lccwarn: Some('A'),
            ctrkerr: Some(0.1),
            dirs: Some('R'),
            ctrkunit: Some('N'),
            aalmcirc: Some('V'),
            aalmperp: Some('V'),
            bear_o2d: Some(11.0),
            bear_o2d_type: Some('M'),
            wpt: Some("DEST".to_string()),
            bear_dest: Some(11.0),
            bear_dest_type: Some('M'),
            bear_steer: Some(11.0),
            bear_steer_type: Some('M'),
            mode: Some('A'),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Apb::parse(&frame.fields).expect("re-parse APB");
        assert_eq!(original, parsed);
    }

    #[test]
    fn apb_signalk_full() {
        let frame =
            parse_frame("$GPAPB,A,A,0.10,R,N,V,V,011,M,DEST,011,M,011,M*3C").expect("valid");
        let apb = Apb::parse(&frame.fields).expect("parse APB");
        assert_eq!(apb.lcgwarn, Some('A'));
        assert_eq!(apb.lccwarn, Some('A'));
        assert!((apb.ctrkerr.expect("ctrkerr") - 0.10).abs() < 0.01);
        assert_eq!(apb.dirs, Some('R'));
        assert_eq!(apb.ctrkunit, Some('N'));
        assert_eq!(apb.aalmcirc, Some('V'));
        assert_eq!(apb.aalmperp, Some('V'));
        assert!((apb.bear_o2d.expect("bear_o2d") - 11.0).abs() < 0.1);
        assert_eq!(apb.bear_o2d_type, Some('M'));
        assert_eq!(apb.wpt, Some("DEST".to_string()));
        assert!((apb.bear_dest.expect("bear_dest") - 11.0).abs() < 0.1);
        assert_eq!(apb.bear_dest_type, Some('M'));
        assert!((apb.bear_steer.expect("bear_steer") - 11.0).abs() < 0.1);
        assert_eq!(apb.bear_steer_type, Some('M'));
        assert!(apb.mode.is_none());
    }
}
