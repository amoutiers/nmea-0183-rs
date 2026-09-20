#![cfg(feature = "osd")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Osd;
use nmea_0183_rs::parse_frame;

#[test]
fn decode_encode() {
    let frame = parse_frame("$RAOSD,179.0,A,179.0,M,00.0,M,,,N*76").expect("valid");
    let osd = Osd::parse(&frame.fields);
    let sentence = osd.to_sentence("RA").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let osd2 = Osd::parse(&frame2.fields);
    assert_eq!(osd, osd2);
}

#[test]
fn osd_values() {
    // Fixture: Radar own-ship data (heading 179° magnetic, stopped, no set/drift).
    let frame = parse_frame("$RAOSD,179.0,A,179.0,M,00.0,M,,,N*76").expect("valid");
    let osd = Osd::parse(&frame.fields);
    assert!((osd.heading.expect("heading") - 179.0).abs() < 1e-4);
    assert_eq!(osd.heading_status, Some('A'));
    assert!((osd.vessel_course.expect("course") - 179.0).abs() < 1e-4);
    assert_eq!(osd.course_ref, Some('M'));
    assert!((osd.vessel_speed.expect("speed") - 0.0).abs() < 1e-4);
    assert_eq!(osd.speed_ref, Some('M'));
    assert_eq!(osd.vessel_set, None);
    assert_eq!(osd.vessel_drift, None);
    assert_eq!(osd.speed_units, Some('N'));
}

#[test]
fn roundtrip() {
    let original = Osd {
        heading: Some(179.0),
        heading_status: Some('A'),
        vessel_course: Some(179.0),
        course_ref: Some('M'),
        vessel_speed: Some(0.0),
        speed_ref: Some('M'),
        vessel_set: None,
        vessel_drift: None,
        speed_units: Some('N'),
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Osd::parse(&frame.fields);
    assert_eq!(original, parsed);
}
