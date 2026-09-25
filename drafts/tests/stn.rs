use nmea_0183_rs::nmea::sentences::Stn;
use nmea_0183_rs::parse_frame;

#[test]
fn stn_pynmeagps_values() {
    let frame = parse_frame("$GPSTN,34*75").expect("valid STN");
    assert_eq!(
        Stn::parse(&frame.fields),
        Some(Stn {
            talker_id: Some("34".into())
        })
    );
}
