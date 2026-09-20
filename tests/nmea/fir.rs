#![cfg(feature = "fir")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Fir;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame(
        "$FRFIR,E,103000,FD,PT,000,007,A,V,Fire Alarm : TEST PT7 Name TEST DZ2 Name*7A",
    )
    .expect("valid");
    let fir = Fir::parse(&frame.fields);
    let sentence = fir.to_sentence("FR").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let fir2 = Fir::parse(&frame2.fields);
    assert_eq!(fir, fir2);
}

#[test]
fn dispatch() {
    let frame = parse_frame(
        "$FRFIR,E,103000,FD,PT,000,007,A,V,Fire Alarm : TEST PT7 Name TEST DZ2 Name*7A",
    )
    .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Fir(_)));
}

#[test]
fn roundtrip() {
    let original = Fir {
        fire_type: Some('E'),
        time: Some("103000".to_string()),
        system: Some("FD".to_string()),
        division1: Some("PT".to_string()),
        division2: Some("000".to_string()),
        detector_number: Some("007".to_string()),
        condition: Some('A'),
        ack_state: Some('V'),
        message: Some("Fire Alarm".to_string()),
    };
    let sentence = original.to_sentence("FR").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Fir::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn fir_values() {
    let frame = parse_frame(
        "$FRFIR,E,103000,FD,PT,000,007,A,V,Fire Alarm : TEST PT7 Name TEST DZ2 Name*7A",
    )
    .expect("valid");
    let fir = Fir::parse(&frame.fields);
    assert_eq!(fir.fire_type, Some('E'));
    assert_eq!(fir.time.as_deref(), Some("103000"));
    assert_eq!(fir.system.as_deref(), Some("FD"));
    assert_eq!(fir.division1.as_deref(), Some("PT"));
    assert_eq!(fir.division2.as_deref(), Some("000"));
    assert_eq!(fir.detector_number.as_deref(), Some("007"));
    assert_eq!(fir.condition, Some('A'));
    assert_eq!(fir.ack_state, Some('V'));
    assert_eq!(
        fir.message.as_deref(),
        Some("Fire Alarm : TEST PT7 Name TEST DZ2 Name")
    );
}
