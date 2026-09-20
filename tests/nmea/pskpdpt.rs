#![cfg(feature = "pskpdpt")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Pskpdpt;
use nmea_0183_rs::parse_frame;

#[test]
fn decode_encode() {
    let frame = parse_frame("$PSKPDPT,0002.5,+00.0,0010,10,03,*77").expect("valid");
    let pskpdpt = Pskpdpt::parse(&frame.fields).expect("parse");
    let sentence = pskpdpt.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let pskpdpt2 = Pskpdpt::parse(&frame2.fields).expect("parse");
    assert_eq!(pskpdpt, pskpdpt2);
}

#[test]
fn roundtrip() {
    let original = Pskpdpt {
        depth: Some(2.5),
        offset: Some(0.0),
        range_scale: Some(10.0),
        echo_strength: Some(10),
        channel: Some(3),
        transducer_location: None,
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pskpdpt::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn pskpdpt_values() {
    let frame = parse_frame("$PSKPDPT,0002.5,+00.0,0010,10,03,*77").expect("valid");
    let p = Pskpdpt::parse(&frame.fields).expect("parse");
    assert!((p.depth.expect("depth") - 2.5).abs() < 1e-4);
    assert!((p.offset.expect("offset") - 0.0).abs() < 1e-4);
    assert!((p.range_scale.expect("range_scale") - 10.0).abs() < 1e-4);
    assert_eq!(p.echo_strength, Some(10));
    assert_eq!(p.channel, Some(3));
    assert!(p.transducer_location.is_none());
}
