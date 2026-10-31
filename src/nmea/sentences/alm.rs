use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ALM: GPS almanac data.
///
/// Wire: `total,msg,PRN,GPS week,SV health,eccentricity,reference time,inclination,
/// omegadot,sqrt(A),omega,long ascension node,mean anomaly,af0,af1`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Alm {
    pub total_messages: Option<u8>,
    pub message_number: Option<u8>,
    pub prn: Option<u8>,
    pub gps_week: Option<u16>,
    /// GPS ICD bit-packed fields, preserved in their wire hexadecimal form.
    pub sv_health: Option<String>,
    pub eccentricity: Option<String>,
    pub almanac_reference_time: Option<String>,
    pub inclination_angle: Option<String>,
    pub right_ascension_rate: Option<String>,
    pub sqrt_semi_major_axis: Option<String>,
    pub argument_of_perigee: Option<String>,
    pub longitude_of_ascension_node: Option<String>,
    pub mean_anomaly: Option<String>,
    pub clock_parameter_af0: Option<String>,
    pub clock_parameter_af1: Option<String>,
}

impl Alm {
    /// Parse fields from a decoded NMEA frame.
    /// Missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
            total_messages: r.u8(),
            message_number: r.u8(),
            prn: r.u8(),
            gps_week: r.u16(),
            sv_health: r.string(),
            eccentricity: r.string(),
            almanac_reference_time: r.string(),
            inclination_angle: r.string(),
            right_ascension_rate: r.string(),
            sqrt_semi_major_axis: r.string(),
            argument_of_perigee: r.string(),
            longitude_of_ascension_node: r.string(),
            mean_anomaly: r.string(),
            clock_parameter_af0: r.string(),
            clock_parameter_af1: r.string(),
        }
    }
}

impl NmeaEncodable for Alm {
    const SENTENCE_TYPE: &str = "ALM";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.total_messages);
        w.u8(self.message_number);
        w.u8(self.prn);
        w.u16(self.gps_week);
        w.string(self.sv_health.as_deref());
        w.string(self.eccentricity.as_deref());
        w.string(self.almanac_reference_time.as_deref());
        w.string(self.inclination_angle.as_deref());
        w.string(self.right_ascension_rate.as_deref());
        w.string(self.sqrt_semi_major_axis.as_deref());
        w.string(self.argument_of_perigee.as_deref());
        w.string(self.longitude_of_ascension_node.as_deref());
        w.string(self.mean_anomaly.as_deref());
        w.string(self.clock_parameter_af0.as_deref());
        w.string(self.clock_parameter_af1.as_deref());
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn alm_empty() {
        assert_eq!(Alm::parse(&[""; 15]), Alm::default());
    }

    #[test]
    fn alm_encode_roundtrip() {
        let original = Alm {
            total_messages: Some(30),
            message_number: Some(1),
            prn: Some(1),
            gps_week: Some(2210),
            sv_health: Some("00".into()),
            eccentricity: Some("617b".into()),
            clock_parameter_af1: Some("ffe".into()),
            ..Default::default()
        };
        let sentence = original.to_sentence("GP").expect("encode ALM");
        let frame = parse_frame(&sentence).expect("re-parse ALM");
        assert_eq!(Alm::parse(&frame.fields), original);
    }

    #[test]
    fn alm_malformed_fields() {
        assert_eq!(Alm::parse(&["bad", "256", "-1", "65536"]), Alm::default());
    }

    #[test]
    fn alm_missing_fields() {
        assert_eq!(Alm::parse(&[]), Alm::default());
    }

    #[test]
    fn alm_values_novatel_oem7() {
        // https://docs.novatel.com/OEM7/Content/Logs/GPALM.htm
        let frame = parse_frame(
            "$GPALM,30,01,01,2210,00,617b,0f,1da7,fd70,a10d0a,24de91,6fe696,16263f,17c,ffe*7B",
        ).expect("valid NovAtel ALM");
        let alm = Alm::parse(&frame.fields);
        assert_eq!(alm.gps_week, Some(2210));
        assert_eq!(alm.sv_health.as_deref(), Some("00"));
        assert_eq!(alm.sqrt_semi_major_axis.as_deref(), Some("a10d0a"));
        assert_eq!(alm.clock_parameter_af1.as_deref(), Some("ffe"));
    }
}
