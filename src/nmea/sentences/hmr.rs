use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// HMR — Heading Monitor Receive.
///
/// Wire: `sensor1_id,sensor2_id,difference_limit,sensor_difference,warning,sensor1_heading,sensor1_status,sensor1_type,sensor1_deviation,sensor1_deviation_ew,sensor2_heading,sensor2_status,sensor2_type,sensor2_deviation,sensor2_deviation_ew,variation,variation_ew`
#[derive(Debug, Clone, PartialEq, Default)]
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
    /// Parse fields from a decoded NMEA frame; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Self {
        let mut r = FieldReader::new(fields);
        Self {
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
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    const FIXTURE: &str = "$--HMR,01,02,1.2,0.1,A,123.4,A,M,0.1,E,123.5,A,M,0.1,E,1.2,E*52";

    fn checksum(body: &str) -> u8 {
        body.bytes().fold(0, |value, byte| value ^ byte)
    }

    #[test]
    fn hmr_empty() {
        assert_eq!(
            Hmr::parse(&["", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", ""]),
            Hmr {
                sensor_1_id: None,
                sensor_2_id: None,
                difference_limit: None,
                sensor_difference: None,
                warning_flag: None,
                sensor_1_heading: None,
                sensor_1_status: None,
                sensor_1_type: None,
                sensor_1_deviation: None,
                sensor_1_deviation_ew: None,
                sensor_2_heading: None,
                sensor_2_status: None,
                sensor_2_type: None,
                sensor_2_deviation: None,
                sensor_2_deviation_ew: None,
                variation: None,
                variation_ew: None,
            }
        );
    }

    #[test]
    fn hmr_encode_roundtrip() {
        let original = Hmr::parse(&[
            "01", "02", "1.2", "0.1", "A", "123.4", "A", "M", "0.1", "E", "123.5", "A",
            "M", "0.1", "E", "1.2", "E",
        ]);
        let sentence = original.to_sentence("HC").expect("encode HMR");
        let frame = parse_frame(sentence.trim()).expect("re-parse HMR");
        assert_eq!(original, Hmr::parse(&frame.fields));
    }

    #[test]
    fn hmr_values_sim183() {
        // Published in the Ninglu SIM183 NMEA0183 Tester Manual, page 12.
        // https://www.manualslib.com/manual/2441097/Ninglu-Nmea0183.html
        let (body, reported_checksum) = FIXTURE[1..].split_once('*').expect("checksum separator");
        assert_eq!(
            checksum(body),
            u8::from_str_radix(reported_checksum, 16).expect("hex checksum")
        );

        let frame = parse_frame(FIXTURE).expect("valid SIM183 HMR frame");
        let hmr = Hmr::parse(&frame.fields);
        assert_eq!(hmr.sensor_1_id.as_deref(), Some("01"));
        assert_eq!(hmr.sensor_2_id.as_deref(), Some("02"));
        assert!((hmr.difference_limit.expect("difference limit") - 1.2).abs() < 1e-4);
        assert!((hmr.sensor_difference.expect("sensor difference") - 0.1).abs() < 1e-4);
        assert_eq!(hmr.warning_flag, Some('A'));
        assert!((hmr.sensor_1_heading.expect("sensor 1 heading") - 123.4).abs() < 1e-4);
        assert_eq!(hmr.sensor_1_status, Some('A'));
        assert_eq!(hmr.sensor_1_type, Some('M'));
        assert!((hmr.sensor_1_deviation.expect("sensor 1 deviation") - 0.1).abs() < 1e-4);
        assert_eq!(hmr.sensor_1_deviation_ew, Some('E'));
        assert!((hmr.sensor_2_heading.expect("sensor 2 heading") - 123.5).abs() < 1e-4);
        assert_eq!(hmr.sensor_2_status, Some('A'));
        assert_eq!(hmr.sensor_2_type, Some('M'));
        assert!((hmr.sensor_2_deviation.expect("sensor 2 deviation") - 0.1).abs() < 1e-4);
        assert_eq!(hmr.sensor_2_deviation_ew, Some('E'));
        assert!((hmr.variation.expect("variation") - 1.2).abs() < 1e-4);
        assert_eq!(hmr.variation_ew, Some('E'));
    }
}
