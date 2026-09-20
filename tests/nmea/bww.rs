#![cfg(feature = "bww")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Bww;
use nmea_0183_rs::parse_frame;

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPBWW,097.0,T,103.2,M,POINTB,POINTA*41").expect("valid");
    let bww = Bww::parse(&frame.fields);
    let sentence = bww.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let bww2 = Bww::parse(&frame2.fields);
    assert_eq!(bww, bww2);
}

#[test]
fn roundtrip() {
    let original = Bww {
        bear_true: Some(97.0),
        bear_true_type: Some('T'),
        bear_mag: Some(103.2),
        bear_mag_type: Some('M'),
        wpt_dest: Some("POINTB".to_string()),
        wpt_origin: Some("POINTA".to_string()),
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Bww::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn bww_values() {
    let frame = parse_frame("$GPBWW,097.0,T,103.2,M,POINTB,POINTA*41").expect("valid");
    let b = Bww::parse(&frame.fields);
    assert!((b.bear_true.expect("bear_true") - 97.0).abs() < 1e-4);
    assert_eq!(b.bear_true_type, Some('T'));
    assert!((b.bear_mag.expect("bear_mag") - 103.2).abs() < 1e-4);
    assert_eq!(b.bear_mag_type, Some('M'));
    assert_eq!(b.wpt_dest.as_deref(), Some("POINTB"));
    assert_eq!(b.wpt_origin.as_deref(), Some("POINTA"));
}
