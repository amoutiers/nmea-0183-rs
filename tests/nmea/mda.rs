#![cfg(feature = "mda")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Mda;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$WIMDA,,I,+0.985,B,+03.1,C,+5.6,C,40.0,3.0,+3.4,C,90.0,T,85.0,M,10.0,N,,M*1A")
            .expect("valid");
    let mda = Mda::parse(&frame.fields).expect("parse");
    let sentence = mda.to_sentence("WI").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let mda2 = Mda::parse(&frame2.fields).expect("parse");
    assert_eq!(mda, mda2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$WIMDA,,I,+0.985,B,+03.1,C,+5.6,C,40.0,3.0,+3.4,C,90.0,T,85.0,M,10.0,N,,M*1A")
            .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Mda(_)));
}

#[test]
fn roundtrip() {
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
    let parsed = Mda::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn mda_values() {
    let frame =
        parse_frame("$WIMDA,,I,+0.985,B,+03.1,C,+5.6,C,40.0,3.0,+3.4,C,90.0,T,85.0,M,10.0,N,,M*1A")
            .expect("valid");
    let m = Mda::parse(&frame.fields).expect("parse");
    assert!(m.baro_inches.is_none());
    assert_eq!(m.baro_inches_unit, Some('I'));
    assert!((m.baro_bars.expect("baro_bars") - 0.985).abs() < 1e-4);
    assert_eq!(m.baro_bars_unit, Some('B'));
    assert!((m.air_temp.expect("air_temp") - 3.1).abs() < 1e-4);
    assert_eq!(m.air_temp_unit, Some('C'));
    assert!((m.water_temp.expect("water_temp") - 5.6).abs() < 1e-4);
    assert_eq!(m.water_temp_unit, Some('C'));
    assert!((m.rel_humidity.expect("rel_humidity") - 40.0).abs() < 1e-4);
    assert!((m.abs_humidity.expect("abs_humidity") - 3.0).abs() < 1e-4);
    assert!((m.dew_point.expect("dew_point") - 3.4).abs() < 1e-4);
    assert_eq!(m.dew_point_unit, Some('C'));
    assert!((m.wind_dir_true.expect("wind_dir_true") - 90.0).abs() < 1e-4);
    assert_eq!(m.wind_dir_true_unit, Some('T'));
    assert!((m.wind_dir_mag.expect("wind_dir_mag") - 85.0).abs() < 1e-4);
    assert_eq!(m.wind_dir_mag_unit, Some('M'));
    assert!((m.wind_speed_knots.expect("wind_speed_knots") - 10.0).abs() < 1e-4);
    assert_eq!(m.wind_speed_knots_unit, Some('N'));
    assert!(m.wind_speed_ms.is_none());
    assert_eq!(m.wind_speed_ms_unit, Some('M'));
}
