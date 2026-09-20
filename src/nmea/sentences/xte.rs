use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// XTE — Cross Track Error.
///
/// Wire: `gwarn,lccwarn,ctrkerr,dirs,disunit[,mode]`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Xte {
    /// General warning flag ('A' = OK, 'V' = warning).
    pub gwarn: Option<char>,
    /// Loran-C cycle lock warning ('A' = OK, 'V' = warning).
    pub lccwarn: Option<char>,
    /// Cross-track error magnitude in nautical miles.
    pub ctrkerr: Option<f32>,
    /// Direction to steer ('L' = left, 'R' = right).
    pub dirs: Option<char>,
    /// Distance unit ('N' = nautical miles).
    pub disunit: Option<char>,
    /// Mode indicator ('A' = autonomous, 'D' = differential, etc.).
    pub mode: Option<char>,
}

impl Xte {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            gwarn: r.char(),
            lccwarn: r.char(),
            ctrkerr: r.f32(),
            dirs: r.char(),
            disunit: r.char(),
            mode: r.char(),
        }
    }
}

impl NmeaEncodable for Xte {
    const SENTENCE_TYPE: &str = "XTE";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.gwarn);
        w.char(self.lccwarn);
        w.f32(self.ctrkerr);
        w.char(self.dirs);
        w.char(self.disunit);
        w.char(self.mode);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn xte_empty() {
        let f = Xte {
            gwarn: None,
            lccwarn: None,
            ctrkerr: None,
            dirs: None,
            disunit: None,
            mode: None,
        }
        .to_sentence("GP").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let x = Xte::parse(&frame.fields);
        assert!(x.gwarn.is_none());
        assert!(x.ctrkerr.is_none());
        assert!(x.mode.is_none());
    }

    #[test]
    fn xte_encode_roundtrip() {
        let original = Xte {
            gwarn: Some('A'),
            lccwarn: Some('A'),
            ctrkerr: Some(0.67),
            dirs: Some('L'),
            disunit: Some('N'),
            mode: Some('D'),
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Xte::parse(&frame.fields);
        assert_eq!(original, parsed);
    }

    #[test]
    fn xte_faa_mode_gonmea() {
        let frame = parse_frame("$GPXTE,V,V,,,N,S*43").expect("valid go-nmea XTE frame");
        let xte = Xte::parse(&frame.fields);
        assert_eq!(xte.gwarn, Some('V'));
        assert_eq!(xte.lccwarn, Some('V'));
        assert!(xte.ctrkerr.is_none());
        assert!(xte.dirs.is_none());
        assert_eq!(xte.disunit, Some('N'));
        assert_eq!(xte.mode, Some('S'));
    }

    #[test]
    fn xte_pynmeagps() {
        let frame = parse_frame("$GPXTE,A,A,0.67,L,N*6F").expect("valid pynmeagps XTE frame");
        let xte = Xte::parse(&frame.fields);
        assert_eq!(xte.gwarn, Some('A'));
        assert_eq!(xte.lccwarn, Some('A'));
        assert!((xte.ctrkerr.expect("ctrkerr") - 0.67).abs() < 0.01);
        assert_eq!(xte.dirs, Some('L'));
        assert_eq!(xte.disunit, Some('N'));
        assert!(xte.mode.is_none());
    }
}
