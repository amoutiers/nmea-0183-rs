use nmea_0183_rs::nmea::sentences::Zfo;
use nmea_0183_rs::parse_frame;

#[test]
fn zfo_marnav_values() {
    let frame = parse_frame("$GPZFO,123456.1,000010,POINT1*0C").expect("valid ZFO");
    assert_eq!(
        Zfo::parse(&frame.fields),
        Some(Zfo {
            observation_time: Some("123456.1".into()),
            elapsed_time: Some("000010".into()),
            origin_waypoint: Some("POINT1".into())
        })
    );
}
