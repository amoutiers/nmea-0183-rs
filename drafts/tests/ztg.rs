use nmea_0183_rs::nmea::sentences::Ztg;
use nmea_0183_rs::parse_frame;

#[test]
fn ztg_marnav_values() {
    let frame = parse_frame("$GPZTG,123456.1,000010,POINT1*16").expect("valid ZTG");
    assert_eq!(
        Ztg::parse(&frame.fields),
        Some(Ztg {
            observation_time: Some("123456.1".into()),
            time_to_go: Some("000010".into()),
            destination_waypoint: Some("POINT1".into())
        })
    );
}
