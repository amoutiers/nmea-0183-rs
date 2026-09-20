#![cfg(feature = "tll")]
use nmea_0183_rs::nmea::NmeaEncodable;
use nmea_0183_rs::nmea::sentences::Tll;
use nmea_0183_rs::parse_frame;

#[test]
fn decode_encode() {
    let frame = parse_frame("$RATLL,,3647.422,N,01432.592,E,,,,*58").expect("valid");
    let tll = Tll::parse(&frame.fields);
    let sentence = tll.to_sentence("RA").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let tll2 = Tll::parse(&frame2.fields);
    assert_eq!(tll, tll2);
}

#[test]
fn roundtrip() {
    let original = Tll {
        target_num: Some(1),
        lat: Some(3647.422),
        ns: Some('N'),
        lon: Some(1432.592),
        ew: Some('E'),
        name: Some("TGT01".to_string()),
        time: Some("120000".to_string()),
        status: Some('T'),
        ref_target: None,
    };
    let sentence = original.to_sentence("RA").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Tll::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn tll_values() {
    // Wire: $RATLL,,3647.422,N,01432.592,E,,,,*58
    // target_num absent (empty field), lat/lon are raw DDMM format
    let frame = parse_frame("$RATLL,,3647.422,N,01432.592,E,,,,*58").expect("valid TLL frame");
    let x = Tll::parse(&frame.fields);
    assert!(x.target_num.is_none());
    assert!((x.lat.expect("lat") - 3647.422).abs() < 1e-2);
    assert_eq!(x.ns, Some('N'));
    assert!((x.lon.expect("lon") - 1432.592).abs() < 1e-2);
    assert_eq!(x.ew, Some('E'));
    assert!(x.name.is_none());
    assert!(x.time.is_none());
    assert!(x.status.is_none());
    assert!(x.ref_target.is_none());
}
