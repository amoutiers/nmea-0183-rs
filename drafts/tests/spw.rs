use nmea_0183_rs::nmea::sentences::Spw;
use nmea_0183_rs::parse_frame;

#[test]
fn spw_pynmeagps_values() {
    let frame = parse_frame("$IISPW,EPV,211000001,2,SESAME*1A").expect("valid SPW");
    assert_eq!(
        Spw::parse(&frame.fields),
        Some(Spw {
            protected_sentence: Some("EPV".into()),
            identifier: Some("211000001".into()),
            password_level: Some(2),
            password: Some("SESAME".into())
        })
    );
}
