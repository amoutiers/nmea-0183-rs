use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// GSA — GPS DOP and Active Satellites.
///
/// Wire: `mode,fix_type,prn01..prn12,pdop,hdop,vdop,system_id`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gsa {
    /// Selection mode ('M' = manual, 'A' = automatic).
    pub mode: Option<char>,
    /// Fix type (1 = no fix, 2 = 2D fix, 3 = 3D fix).
    pub fix_type: Option<u8>,
    /// PRN numbers of satellites used in solution (up to 12).
    pub prns: [Option<u8>; 12],
    /// Position dilution of precision.
    pub pdop: Option<f32>,
    /// Horizontal dilution of precision.
    pub hdop: Option<f32>,
    /// Vertical dilution of precision.
    pub vdop: Option<f32>,
    /// GNSS system id (NMEA 4.11+): one hex digit ('1'=GPS, '2'=GLONASS, '3'=Galileo,
    /// '4'=BeiDou, …). Parsed as a single character which keeps only the first
    /// character. NMEA 4.11 mandates a single hex digit; multi-character values from
    /// non-conforming devices are silently truncated.
    pub system_id: Option<char>,
}

impl Gsa {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let mode = r.char();
        let fix_type = r.u8();
        let prns = [
            r.u8(),
            r.u8(),
            r.u8(),
            r.u8(),
            r.u8(),
            r.u8(),
            r.u8(),
            r.u8(),
            r.u8(),
            r.u8(),
            r.u8(),
            r.u8(),
        ];
        let pdop = r.f32();
        let hdop = r.f32();
        let vdop = r.f32();
        let system_id = r.char();
        Some(Self {
            mode,
            fix_type,
            prns,
            pdop,
            hdop,
            vdop,
            system_id,
        })
    }
}

impl NmeaEncodable for Gsa {
    const SENTENCE_TYPE: &str = "GSA";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.mode);
        w.u8(self.fix_type);
        for prn in &self.prns {
            w.u8(*prn);
        }
        w.f32(self.pdop);
        w.f32(self.hdop);
        w.f32(self.vdop);
        w.char(self.system_id);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn gsa_empty() {
        let f = Gsa {
            mode: None,
            fix_type: None,
            prns: [None; 12],
            pdop: None,
            hdop: None,
            vdop: None,
            system_id: None,
        }
        .to_sentence("GP").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let g = Gsa::parse(&frame.fields).expect("parse");
        assert!(g.mode.is_none());
        assert!(g.fix_type.is_none());
        assert!(g.prns.iter().all(|p| p.is_none()));
        assert!(g.pdop.is_none());
    }

    #[test]
    fn gsa_encode_roundtrip() {
        let original = Gsa {
            mode: Some('A'),
            fix_type: Some(3),
            prns: [
                Some(4),
                Some(5),
                None,
                Some(9),
                Some(12),
                None,
                None,
                Some(24),
                None,
                None,
                None,
                None,
            ],
            pdop: Some(2.5),
            hdop: Some(1.3),
            vdop: Some(2.1),
            system_id: None,
        };
        let sentence = original.to_sentence("GP").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Gsa::parse(&frame.fields).expect("re-parse GSA");
        assert_eq!(original, parsed);
    }

    #[test]
    fn gsa_signalk() {
        let frame = parse_frame("$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39")
            .expect("valid signalk GSA frame");
        let gsa = Gsa::parse(&frame.fields).expect("parse GSA");
        assert_eq!(gsa.mode, Some('A'));
        assert_eq!(gsa.fix_type, Some(3));
        assert_eq!(gsa.prns[0], Some(4));
        assert_eq!(gsa.prns[1], Some(5));
        assert!(gsa.prns[2].is_none());
        assert_eq!(gsa.prns[3], Some(9));
        assert_eq!(gsa.prns[4], Some(12));
        assert_eq!(gsa.prns[7], Some(24));
        assert!((gsa.pdop.expect("pdop") - 2.5).abs() < 0.01);
        assert!((gsa.hdop.expect("hdop") - 1.3).abs() < 0.01);
        assert!((gsa.vdop.expect("vdop") - 2.1).abs() < 0.01);
    }

    #[test]
    fn gsa_system_id_4_11() {
        // NMEA 4.11 GSA appends a GNSS system id; '4' (BeiDou) must round-trip.
        let frame = parse_frame("$GNGSA,A,3,13,12,22,19,08,21,,,,,,,1.05,0.64,0.83,4*0B")
            .expect("valid GN GSA frame");
        let gsa = Gsa::parse(&frame.fields).expect("parse GSA");
        assert_eq!(gsa.system_id, Some('4'));

        let sentence = gsa.to_sentence("GN").expect("encode");
        let frame2 = parse_frame(sentence.trim()).expect("re-parse");
        let gsa2 = Gsa::parse(&frame2.fields).expect("re-parse");
        assert_eq!(gsa2.system_id, Some('4'));
    }

    #[test]
    fn gsa_system_id_gonmea() {
        // NMEA 4.11 adds system_id after vdop; struct silently ignores it
        let frame = parse_frame("$GNGSA,A,3,13,12,22,19,08,21,,,,,,,1.05,0.64,0.83,4*0B")
            .expect("valid go-nmea GSA frame");
        let gsa = Gsa::parse(&frame.fields).expect("parse GSA");
        assert_eq!(gsa.mode, Some('A'));
        assert_eq!(gsa.fix_type, Some(3));
        assert_eq!(gsa.prns[0], Some(13));
        assert_eq!(gsa.prns[1], Some(12));
        assert_eq!(gsa.prns[2], Some(22));
        assert_eq!(gsa.prns[3], Some(19));
        assert_eq!(gsa.prns[4], Some(8));
        assert_eq!(gsa.prns[5], Some(21));
        assert!(gsa.prns[6].is_none());
        assert!((gsa.pdop.expect("pdop") - 1.05).abs() < 0.01);
        assert!((gsa.hdop.expect("hdop") - 0.64).abs() < 0.01);
        assert!((gsa.vdop.expect("vdop") - 0.83).abs() < 0.01);
    }
}
