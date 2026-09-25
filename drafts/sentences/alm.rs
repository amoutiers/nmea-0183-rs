use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ALM — GPS Almanac Data.
///
/// Wire: `total,msg,PRN,GPS week,SV health,eccentricity,reference time,inclination,
/// omegadot,sqrt(A),omega,long ascension node,mean anomaly,af0,af1`
#[derive(Debug, Clone, PartialEq)]
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
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
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
        })
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
