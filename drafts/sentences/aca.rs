use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// ACA — AIS Regional Channel Assignment Message.
///
/// Wire: `sequence,NE lat,NS,NE lon,EW,SW lat,NS,SW lon,EW,transition zone,
/// channel A,A bandwidth,channel B,B bandwidth,TX/RX mode,power,source,in use,time`
#[derive(Debug, Clone, PartialEq)]
pub struct Aca {
    pub sequence: Option<u8>,
    /// Northeast latitude in raw NMEA `DDMM.MMMM` form.
    pub ne_lat: Option<f64>,
    pub ne_ns: Option<char>,
    /// Northeast longitude in raw NMEA `DDDMM.MMMM` form.
    pub ne_lon: Option<f64>,
    pub ne_ew: Option<char>,
    /// Southwest latitude in raw NMEA `DDMM.MMMM` form.
    pub sw_lat: Option<f64>,
    pub sw_ns: Option<char>,
    /// Southwest longitude in raw NMEA `DDDMM.MMMM` form.
    pub sw_lon: Option<f64>,
    pub sw_ew: Option<char>,
    pub transition_zone_size: Option<u8>,
    pub channel_a: Option<u16>,
    pub channel_a_bandwidth: Option<u8>,
    pub channel_b: Option<u16>,
    pub channel_b_bandwidth: Option<u8>,
    pub tx_rx_mode: Option<u8>,
    pub power_level: Option<u8>,
    pub information_source: Option<char>,
    pub in_use_flag: Option<u8>,
    /// UTC time of the in-use change, preserved as `HHMMSS.SS` text.
    pub in_use_change_time: Option<String>,
}

impl Aca {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            sequence: r.u8(),
            ne_lat: r.f64(),
            ne_ns: r.char(),
            ne_lon: r.f64(),
            ne_ew: r.char(),
            sw_lat: r.f64(),
            sw_ns: r.char(),
            sw_lon: r.f64(),
            sw_ew: r.char(),
            transition_zone_size: r.u8(),
            channel_a: r.u16(),
            channel_a_bandwidth: r.u8(),
            channel_b: r.u16(),
            channel_b_bandwidth: r.u8(),
            tx_rx_mode: r.u8(),
            power_level: r.u8(),
            information_source: r.char(),
            in_use_flag: r.u8(),
            in_use_change_time: r.string(),
        })
    }
}

impl NmeaEncodable for Aca {
    const SENTENCE_TYPE: &'static str = "ACA";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.u8(self.sequence);
        w.lat(self.ne_lat);
        w.char(self.ne_ns);
        w.lon(self.ne_lon);
        w.char(self.ne_ew);
        w.lat(self.sw_lat);
        w.char(self.sw_ns);
        w.lon(self.sw_lon);
        w.char(self.sw_ew);
        w.u8(self.transition_zone_size);
        w.u16(self.channel_a);
        w.u8(self.channel_a_bandwidth);
        w.u16(self.channel_b);
        w.u8(self.channel_b_bandwidth);
        w.u8(self.tx_rx_mode);
        w.u8(self.power_level);
        w.char(self.information_source);
        w.u8(self.in_use_flag);
        w.string(self.in_use_change_time.as_deref());
        w.finish()
    }
}
