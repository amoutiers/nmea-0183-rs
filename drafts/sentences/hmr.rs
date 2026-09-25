use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HMR — Heading Monitor Receive.
///
/// Wire: `sensor1_id,sensor2_id,difference_limit,sensor_difference,warning,sensor1_heading,sensor1_status,sensor1_type,sensor1_deviation,sensor1_deviation_ew,sensor2_heading,sensor2_status,sensor2_type,sensor2_deviation,sensor2_deviation_ew,variation,variation_ew`
#[derive(Debug, Clone, PartialEq)]
pub struct Hmr {
    /// Heading sensor 1 ID, retained as text because leading zeroes are significant.
    pub sensor_1_id: Option<String>,
    /// Heading sensor 2 ID, retained as text because leading zeroes are significant.
    pub sensor_2_id: Option<String>,
    /// Maximum permitted difference between sensors, in degrees.
    pub difference_limit: Option<f32>,
    /// Actual difference between sensors, in degrees.
    pub sensor_difference: Option<f32>,
    /// 'A' when the sensor difference is within the limit, 'V' when it exceeds it.
    pub warning_flag: Option<char>,
    /// Sensor 1 heading, in degrees.
    pub sensor_1_heading: Option<f32>,
    /// Sensor 1 status: 'A' when data is valid.
    pub sensor_1_status: Option<char>,
    /// Sensor 1 type: 'T' (true) or 'M' (magnetic).
    pub sensor_1_type: Option<char>,
    /// Sensor 1 deviation, in degrees.
    pub sensor_1_deviation: Option<f32>,
    /// Sensor 1 deviation direction: 'E' or 'W'.
    pub sensor_1_deviation_ew: Option<char>,
    /// Sensor 2 heading, in degrees.
    pub sensor_2_heading: Option<f32>,
    /// Sensor 2 status: 'A' when data is valid.
    pub sensor_2_status: Option<char>,
    /// Sensor 2 type: 'T' (true) or 'M' (magnetic).
    pub sensor_2_type: Option<char>,
    /// Sensor 2 deviation, in degrees.
    pub sensor_2_deviation: Option<f32>,
    /// Sensor 2 deviation direction: 'E' or 'W'.
    pub sensor_2_deviation_ew: Option<char>,
    /// Magnetic variation, in degrees.
    pub variation: Option<f32>,
    /// Magnetic variation direction: 'E' or 'W'.
    pub variation_ew: Option<char>,
}

impl Hmr {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            sensor_1_id: r.string(),
            sensor_2_id: r.string(),
            difference_limit: r.f32(),
            sensor_difference: r.f32(),
            warning_flag: r.char(),
            sensor_1_heading: r.f32(),
            sensor_1_status: r.char(),
            sensor_1_type: r.char(),
            sensor_1_deviation: r.f32(),
            sensor_1_deviation_ew: r.char(),
            sensor_2_heading: r.f32(),
            sensor_2_status: r.char(),
            sensor_2_type: r.char(),
            sensor_2_deviation: r.f32(),
            sensor_2_deviation_ew: r.char(),
            variation: r.f32(),
            variation_ew: r.char(),
        })
    }
}

impl NmeaEncodable for Hmr {
    const SENTENCE_TYPE: &str = "HMR";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.string(self.sensor_1_id.as_deref());
        w.string(self.sensor_2_id.as_deref());
        w.f32(self.difference_limit);
        w.f32(self.sensor_difference);
        w.char(self.warning_flag);
        w.f32(self.sensor_1_heading);
        w.char(self.sensor_1_status);
        w.char(self.sensor_1_type);
        w.f32(self.sensor_1_deviation);
        w.char(self.sensor_1_deviation_ew);
        w.f32(self.sensor_2_heading);
        w.char(self.sensor_2_status);
        w.char(self.sensor_2_type);
        w.f32(self.sensor_2_deviation);
        w.char(self.sensor_2_deviation_ew);
        w.f32(self.variation);
        w.char(self.variation_ew);
        w.finish()
    }
}
