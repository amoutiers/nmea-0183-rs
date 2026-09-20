#![cfg(feature = "vhw")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::Vhw;
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$SDVHW,182.5,T,181.8,M,0.0,N,0.0,K*4C").expect("valid");
    let vhw = Vhw::parse(&frame.fields);
    let sentence = vhw.to_sentence("SD").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let vhw2 = Vhw::parse(&frame2.fields);
    assert_eq!(vhw, vhw2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$SDVHW,182.5,T,181.8,M,0.0,N,0.0,K*4C").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Vhw(_)));
}

#[test]
fn roundtrip() {
    let original = Vhw {
        heading_true: Some(182.5),
        heading_mag: Some(181.8),
        speed_kts: Some(12.5),
        speed_kmh: Some(23.1),
    };
    let sentence = original.to_sentence("SD").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Vhw::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn vhw_values() {
    // (a) value half
    let frame = parse_frame("$SDVHW,182.5,T,181.8,M,0.0,N,0.0,K*4C").expect("valid VHW frame");
    let x = Vhw::parse(&frame.fields);
    assert!((x.heading_true.expect("heading_true") - 182.5).abs() < 1e-2);
    assert!((x.heading_mag.expect("heading_mag") - 181.8).abs() < 1e-2);
    assert!((x.speed_kts.expect("speed_kts") - 0.0).abs() < 1e-2);
    assert!((x.speed_kmh.expect("speed_kmh") - 0.0).abs() < 1e-2);

    // (b) wire half — 0.0 encodes as "0"
    let s = x.to_sentence("SD").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(body, "SDVHW,182.5,T,181.8,M,0,N,0,K");
}
