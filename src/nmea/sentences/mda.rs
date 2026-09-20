use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// MDA — Meteorological Composite.
///
/// Wire: `baroI,I,baroB,B,airT,C,waterT,C,relHumid,absHumid,dewT,C,windDirT,T,windDirM,M,windSpdN,N,windSpdM,M`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mda {
    /// Barometric pressure in inches of mercury.
    pub baro_inches: Option<f32>,
    /// Barometric pressure unit ('I' = inches of mercury).
    pub baro_inches_unit: Option<char>,
    /// Barometric pressure in bars.
    pub baro_bars: Option<f32>,
    /// Barometric pressure unit ('B' = bars).
    pub baro_bars_unit: Option<char>,
    /// Air temperature in degrees Celsius.
    pub air_temp: Option<f32>,
    /// Air temperature unit ('C' = Celsius).
    pub air_temp_unit: Option<char>,
    /// Water temperature in degrees Celsius.
    pub water_temp: Option<f32>,
    /// Water temperature unit ('C' = Celsius).
    pub water_temp_unit: Option<char>,
    /// Relative humidity in percent.
    pub rel_humidity: Option<f32>,
    /// Absolute humidity in percent.
    pub abs_humidity: Option<f32>,
    /// Dew point temperature in degrees Celsius.
    pub dew_point: Option<f32>,
    /// Dew point unit ('C' = Celsius).
    pub dew_point_unit: Option<char>,
    /// Wind direction true in degrees.
    pub wind_dir_true: Option<f32>,
    /// Wind direction true unit ('T' = true).
    pub wind_dir_true_unit: Option<char>,
    /// Wind direction magnetic in degrees.
    pub wind_dir_mag: Option<f32>,
    /// Wind direction magnetic unit ('M' = magnetic).
    pub wind_dir_mag_unit: Option<char>,
    /// Wind speed in knots.
    pub wind_speed_knots: Option<f32>,
    /// Wind speed unit ('N' = knots).
    pub wind_speed_knots_unit: Option<char>,
    /// Wind speed in meters per second.
    pub wind_speed_ms: Option<f32>,
    /// Wind speed unit ('M' = meters per second).
    pub wind_speed_ms_unit: Option<char>,
}

impl Mda {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        Some(Self {
            baro_inches: r.f32(),
            baro_inches_unit: r.char(),
            baro_bars: r.f32(),
            baro_bars_unit: r.char(),
            air_temp: r.f32(),
            air_temp_unit: r.char(),
            water_temp: r.f32(),
            water_temp_unit: r.char(),
            rel_humidity: r.f32(),
            abs_humidity: r.f32(),
            dew_point: r.f32(),
            dew_point_unit: r.char(),
            wind_dir_true: r.f32(),
            wind_dir_true_unit: r.char(),
            wind_dir_mag: r.f32(),
            wind_dir_mag_unit: r.char(),
            wind_speed_knots: r.f32(),
            wind_speed_knots_unit: r.char(),
            wind_speed_ms: r.f32(),
            wind_speed_ms_unit: r.char(),
        })
    }
}

impl NmeaEncodable for Mda {
    const SENTENCE_TYPE: &str = "MDA";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.baro_inches);
        w.char(self.baro_inches_unit);
        w.f32(self.baro_bars);
        w.char(self.baro_bars_unit);
        w.f32(self.air_temp);
        w.char(self.air_temp_unit);
        w.f32(self.water_temp);
        w.char(self.water_temp_unit);
        w.f32(self.rel_humidity);
        w.f32(self.abs_humidity);
        w.f32(self.dew_point);
        w.char(self.dew_point_unit);
        w.f32(self.wind_dir_true);
        w.char(self.wind_dir_true_unit);
        w.f32(self.wind_dir_mag);
        w.char(self.wind_dir_mag_unit);
        w.f32(self.wind_speed_knots);
        w.char(self.wind_speed_knots_unit);
        w.f32(self.wind_speed_ms);
        w.char(self.wind_speed_ms_unit);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn mda_empty() {
        let f = Mda {
            baro_inches: None,
            baro_inches_unit: None,
            baro_bars: None,
            baro_bars_unit: None,
            air_temp: None,
            air_temp_unit: None,
            water_temp: None,
            water_temp_unit: None,
            rel_humidity: None,
            abs_humidity: None,
            dew_point: None,
            dew_point_unit: None,
            wind_dir_true: None,
            wind_dir_true_unit: None,
            wind_dir_mag: None,
            wind_dir_mag_unit: None,
            wind_speed_knots: None,
            wind_speed_knots_unit: None,
            wind_speed_ms: None,
            wind_speed_ms_unit: None,
        }
        .to_sentence("WI").expect("encode");
        let frame = parse_frame(f.trim()).expect("valid");
        let m = Mda::parse(&frame.fields).expect("parse");
        assert!(m.baro_inches.is_none());
        assert!(m.air_temp.is_none());
        assert!(m.wind_speed_knots.is_none());
    }

    #[test]
    fn mda_encode_roundtrip() {
        let original = Mda {
            baro_inches: None,
            baro_inches_unit: Some('I'),
            baro_bars: Some(1.013),
            baro_bars_unit: Some('B'),
            air_temp: Some(22.5),
            air_temp_unit: Some('C'),
            water_temp: Some(18.0),
            water_temp_unit: Some('C'),
            rel_humidity: Some(65.0),
            abs_humidity: Some(10.0),
            dew_point: Some(15.5),
            dew_point_unit: Some('C'),
            wind_dir_true: Some(180.0),
            wind_dir_true_unit: Some('T'),
            wind_dir_mag: Some(178.0),
            wind_dir_mag_unit: Some('M'),
            wind_speed_knots: Some(12.0),
            wind_speed_knots_unit: Some('N'),
            wind_speed_ms: Some(6.2),
            wind_speed_ms_unit: Some('M'),
        };
        let sentence = original.to_sentence("WI").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Mda::parse(&frame.fields).expect("re-parse MDA");
        assert_eq!(original, parsed);
    }

    #[test]
    fn mda_full_gonmea() {
        let frame =
            parse_frame("$WIMDA,3.02,I,1.01,B,23.4,C,,,40.2,,12.1,C,19.3,T,20.1,M,13.1,N,1.1,M*62")
                .expect("valid go-nmea MDA frame");
        let mda = Mda::parse(&frame.fields).expect("parse MDA");
        assert!((mda.baro_inches.expect("baro_in") - 3.02).abs() < 0.01);
        assert_eq!(mda.baro_inches_unit, Some('I'));
        assert!((mda.baro_bars.expect("baro_b") - 1.01).abs() < 0.01);
        assert_eq!(mda.baro_bars_unit, Some('B'));
        assert!((mda.air_temp.expect("air_t") - 23.4).abs() < 0.1);
        assert_eq!(mda.air_temp_unit, Some('C'));
        assert!(mda.water_temp.is_none());
        assert!((mda.rel_humidity.expect("rh") - 40.2).abs() < 0.1);
        assert!(mda.abs_humidity.is_none());
        assert!((mda.dew_point.expect("dew") - 12.1).abs() < 0.1);
        assert!((mda.wind_dir_true.expect("wdT") - 19.3).abs() < 0.1);
        assert_eq!(mda.wind_dir_true_unit, Some('T'));
        assert!((mda.wind_dir_mag.expect("wdM") - 20.1).abs() < 0.1);
        assert!((mda.wind_speed_knots.expect("wsN") - 13.1).abs() < 0.1);
        assert_eq!(mda.wind_speed_knots_unit, Some('N'));
        assert!((mda.wind_speed_ms.expect("wsM") - 1.1).abs() < 0.1);
    }

    #[test]
    fn mda_signalk_partial() {
        let frame = parse_frame(
            "$WIMDA,,I,+0.985,B,+03.1,C,+5.6,C,40.0,3.0,+3.4,C,90.0,T,85.0,M,10.0,N,,M*1A",
        )
        .expect("valid signalk MDA frame");
        let mda = Mda::parse(&frame.fields).expect("parse MDA");
        assert!(mda.baro_inches.is_none());
        assert_eq!(mda.baro_inches_unit, Some('I'));
        assert!((mda.baro_bars.expect("baroB") - 0.985).abs() < 0.001);
        assert_eq!(mda.baro_bars_unit, Some('B'));
        assert!((mda.air_temp.expect("airT") - 3.1).abs() < 0.1);
        assert_eq!(mda.air_temp_unit, Some('C'));
        assert!((mda.water_temp.expect("waterT") - 5.6).abs() < 0.1);
        assert!((mda.rel_humidity.expect("rh") - 40.0).abs() < 0.1);
        assert!((mda.abs_humidity.expect("ah") - 3.0).abs() < 0.1);
        assert!((mda.dew_point.expect("dew") - 3.4).abs() < 0.1);
        assert!((mda.wind_dir_true.expect("wdT") - 90.0).abs() < 0.1);
        assert!((mda.wind_dir_mag.expect("wdM") - 85.0).abs() < 0.1);
        assert!((mda.wind_speed_knots.expect("wsN") - 10.0).abs() < 0.1);
        assert!(mda.wind_speed_ms.is_none());
        assert_eq!(mda.wind_speed_ms_unit, Some('M'));
    }
}
