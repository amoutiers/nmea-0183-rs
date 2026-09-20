use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// RMB — Recommended Minimum Navigation Information.
///
/// Wire: `status,ctrkerr,dirs,wpt_origin,wpt_dest,dest_lat,ns,dest_lon,ew,range,bearing,velclos,arrstatus,valstatus`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rmb {
    /// Data status ('A' = active, 'V' = void).
    pub status: Option<char>,
    /// Cross-track error in nautical miles.
    pub ctrkerr: Option<f32>,
    /// Direction to steer ('L' = left, 'R' = right).
    pub dirs: Option<char>,
    /// Origin waypoint identifier.
    pub wpt_origin: Option<String>,
    /// Destination waypoint identifier.
    pub wpt_dest: Option<String>,
    /// Destination latitude in NMEA format (ddmm.mm).
    pub dest_lat: Option<f64>,
    /// North/South indicator ('N' or 'S').
    pub ns: Option<char>,
    /// Destination longitude in NMEA format (dddmm.mm).
    pub dest_lon: Option<f64>,
    /// East/West indicator ('E' or 'W').
    pub ew: Option<char>,
    /// Range to destination in nautical miles.
    pub range: Option<f32>,
    /// True bearing to destination in degrees.
    pub bearing: Option<f32>,
    /// Closing velocity toward destination in knots.
    pub velclos: Option<f32>,
    /// Arrival status ('A' = arrived, 'V' = not arrived).
    pub arrstatus: Option<char>,
    /// FAA mode indicator ('A' = autonomous, 'D' = differential, etc.).
    pub valstatus: Option<char>,
}

impl Rmb {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None` in the returned value.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            status: r.char(),
            ctrkerr: r.f32(),
            dirs: r.char(),
            wpt_origin: r.string(),
            wpt_dest: r.string(),
            dest_lat: r.f64(),
            ns: r.char(),
            dest_lon: r.f64(),
            ew: r.char(),
            range: r.f32(),
            bearing: r.f32(),
            velclos: r.f32(),
            arrstatus: r.char(),
            valstatus: r.char(),
        }
    }
}

impl NmeaEncodable for Rmb {
    const SENTENCE_TYPE: &str = "RMB";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.char(self.status);
        w.f32(self.ctrkerr);
        w.char(self.dirs);
        w.string(self.wpt_origin.as_deref());
        w.string(self.wpt_dest.as_deref());
        w.lat(self.dest_lat);
        w.char(self.ns);
        w.lon(self.dest_lon);
        w.char(self.ew);
        w.f32(self.range);
        w.f32(self.bearing);
        w.f32(self.velclos);
        w.char(self.arrstatus);
        w.char(self.valstatus);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn rmb_empty() {
        let f = parse_frame("$GPRMB,,,,,,,,,,,,,,*4A").expect("valid");
        let r = Rmb::parse(&f.fields);
        assert!(r.status.is_none());
        assert!(r.ctrkerr.is_none());
        assert!(r.dirs.is_none());
        assert!(r.wpt_origin.is_none());
        assert!(r.wpt_dest.is_none());
        assert!(r.dest_lat.is_none());
        assert!(r.range.is_none());
        assert!(r.bearing.is_none());
        assert!(r.velclos.is_none());
        assert!(r.arrstatus.is_none());
        assert!(r.valstatus.is_none());
    }

    #[test]
    fn rmb_encode_roundtrip() {
        let rmb = Rmb {
            status: Some('A'),
            ctrkerr: Some(0.5),
            dirs: Some('L'),
            wpt_origin: Some("001".to_string()),
            wpt_dest: Some("002".to_string()),
            dest_lat: Some(4653.55),
            ns: Some('N'),
            dest_lon: Some(7115.984),
            ew: Some('W'),
            range: Some(2.505),
            bearing: Some(334.205),
            velclos: Some(0.0),
            arrstatus: Some('V'),
            valstatus: Some('A'),
        };
        let sentence = rmb.to_sentence("EC").expect("encode");
        assert!(sentence.starts_with("$ECRMB,"));
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let rmb2 = Rmb::parse(&frame.fields);
        assert_eq!(rmb.status, rmb2.status);
        assert_eq!(rmb.wpt_dest, rmb2.wpt_dest);
        assert_eq!(rmb.valstatus, rmb2.valstatus);
    }

    #[test]
    fn rmb_full_signalk() {
        let frame =
            parse_frame("$ECRMB,A,0.000,L,001,002,4653.550,N,07115.984,W,2.505,334.205,0.000,V*04")
                .expect("valid");
        let rmb = Rmb::parse(&frame.fields);
        assert_eq!(rmb.status, Some('A'));
        assert!((rmb.ctrkerr.expect("ctrkerr") - 0.0).abs() < 0.001);
        assert_eq!(rmb.dirs, Some('L'));
        assert_eq!(rmb.wpt_origin, Some("001".to_string()));
        assert_eq!(rmb.wpt_dest, Some("002".to_string()));
        assert!((rmb.dest_lat.expect("lat") - 4653.55).abs() < 0.01);
        assert_eq!(rmb.ns, Some('N'));
        assert!((rmb.dest_lon.expect("lon") - 7115.984).abs() < 0.01);
        assert_eq!(rmb.ew, Some('W'));
        assert!((rmb.range.expect("range") - 2.505).abs() < 0.01);
        assert!((rmb.bearing.expect("bearing") - 334.205).abs() < 0.01);
        assert!((rmb.velclos.expect("velclos") - 0.0).abs() < 0.001);
        assert_eq!(rmb.arrstatus, Some('V'));
        assert!(rmb.valstatus.is_none());
    }

    #[test]
    fn rmb_pynmeagps() {
        let frame =
            parse_frame("$GPRMB,A,0.66,L,003,004,4917.24,N,12309.57,W,001.3,052.5,000.5,V*20")
                .expect("valid pynmeagps RMB frame");
        let rmb = Rmb::parse(&frame.fields);
        assert_eq!(rmb.status, Some('A'));
        assert!((rmb.ctrkerr.expect("ctrkerr") - 0.66).abs() < 0.01);
        assert_eq!(rmb.wpt_dest, Some("004".to_string()));
    }
}
