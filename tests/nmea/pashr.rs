#![cfg(feature = "pashr")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Pashr;
use nmea_0183_rs::parse_frame;

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$PASHR,085335.000,224.19,T,-01.26,+00.83,+00.10,0.101,0.113,0.267,1,0*07")
            .expect("valid");
    let pashr = Pashr::parse(&frame.fields);
    let sentence = pashr.to_sentence("").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let pashr2 = Pashr::parse(&frame2.fields);
    assert_eq!(pashr, pashr2);
}

#[test]
fn roundtrip() {
    let original = Pashr {
        time: Some("085335.000".to_string()),
        heading: Some(224.19),
        roll: Some(-1.26),
        pitch: Some(0.83),
        heave: Some(0.10),
        roll_accuracy: Some(0.101),
        pitch_accuracy: Some(0.113),
        heading_accuracy: Some(0.267),
        gnss_quality: Some(1),
        imu_alignment: Some(0),
    };
    let sentence = original.to_sentence("").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Pashr::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn pashr_values() {
    // (a) value half
    let frame =
        parse_frame("$PASHR,085335.000,224.19,T,-01.26,+00.83,+00.10,0.101,0.113,0.267,1,0*07")
            .expect("valid");
    let x = Pashr::parse(&frame.fields);
    assert_eq!(x.time.as_deref(), Some("085335.000"));
    assert!((x.heading.expect("heading") - 224.19).abs() < 1e-2);
    assert!((x.roll.expect("roll") - (-1.26)).abs() < 1e-2);
    assert!((x.pitch.expect("pitch") - 0.83).abs() < 1e-2);
    assert!((x.heave.expect("heave") - 0.10).abs() < 1e-2);
    assert!((x.roll_accuracy.expect("roll_accuracy") - 0.101).abs() < 1e-2);
    assert!((x.pitch_accuracy.expect("pitch_accuracy") - 0.113).abs() < 1e-2);
    assert!((x.heading_accuracy.expect("heading_accuracy") - 0.267).abs() < 1e-2);
    assert_eq!(x.gnss_quality, Some(1));
    assert_eq!(x.imu_alignment, Some(0));

    // (b) wire half — proprietary: to_sentence("") body starts with PASHR,
    // normalizations: -01.26→-1.26, +00.83→0.83, +00.10→0.1
    let s = x.to_sentence("").expect("encode");
    let body = s.trim().trim_start_matches('$');
    let body = &body[..body.rfind('*').expect("cksum")];
    assert_eq!(
        body,
        "PASHR,085335.000,224.19,T,-1.26,0.83,0.1,0.101,0.113,0.267,1,0"
    );
}
