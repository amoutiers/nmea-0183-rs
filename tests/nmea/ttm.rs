#![cfg(feature = "ttm")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Ttm;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$RATTM,02,1.43,170.5,T,0.16,264.4,T,1.42,36.9,N,,T,,,M*2A").expect("valid");
    let ttm = Ttm::parse(&frame.fields).expect("parse");
    let sentence = ttm.to_sentence("RA").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let ttm2 = Ttm::parse(&frame2.fields).expect("parse");
    assert_eq!(ttm, ttm2);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$RATTM,02,1.43,170.5,T,0.16,264.4,T,1.42,36.9,N,,T,,,M*2A").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Ttm(_)));
}

#[test]
fn roundtrip() {
    let original = Ttm {
        target_num: Some(2),
        dist: Some(1.43),
        bearing: Some(170.5),
        bearing_type: Some('T'),
        speed: Some(0.16),
        course: Some(264.4),
        course_type: Some('T'),
        dist_cpa: Some(1.42),
        time_cpa: Some(36.9),
        speed_units: Some('N'),
        name: None,
        status: Some('T'),
        ref_target: None,
        time: None,
        acq_type: Some('M'),
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Ttm::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn ttm_values() {
    // Wire: $RATTM,02,1.43,170.5,T,0.16,264.4,T,1.42,36.9,N,,T,,,M*2A
    // Trailing fields after speed_units 'N': ,,T,,,M
    //   name=""    -> None
    //   status='T' -> Some('T')
    //   ref_target="" -> None
    //   time=""    -> None
    //   acq_type='M' -> Some('M')
    let frame = parse_frame("$RATTM,02,1.43,170.5,T,0.16,264.4,T,1.42,36.9,N,,T,,,M*2A")
        .expect("valid TTM frame");
    let x = Ttm::parse(&frame.fields).expect("parse TTM");
    assert_eq!(x.target_num, Some(2));
    assert!((x.dist.expect("dist") - 1.43).abs() < 1e-2);
    assert!((x.bearing.expect("bearing") - 170.5).abs() < 1e-2);
    assert_eq!(x.bearing_type, Some('T'));
    assert!((x.speed.expect("speed") - 0.16).abs() < 1e-2);
    assert!((x.course.expect("course") - 264.4).abs() < 1e-2);
    assert_eq!(x.course_type, Some('T'));
    assert!((x.dist_cpa.expect("dist_cpa") - 1.42).abs() < 1e-2);
    assert!((x.time_cpa.expect("time_cpa") - 36.9).abs() < 1e-2);
    assert_eq!(x.speed_units, Some('N'));
    assert!(x.name.is_none());
    assert_eq!(x.status, Some('T'));
    assert!(x.ref_target.is_none());
    assert!(x.time.is_none());
    assert_eq!(x.acq_type, Some('M'));
}
