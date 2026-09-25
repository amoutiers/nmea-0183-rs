use nmea_0183_rs::nmea::sentences::Wnc;
use nmea_0183_rs::parse_frame;

#[test]
fn wnc_marnav_values() {
    let frame = parse_frame("$GPWNC,12.3,N,5.6,K,POINT1,POINT2*78").expect("valid WNC");
    let wnc = Wnc::parse(&frame.fields).expect("parse WNC");
    assert_eq!(wnc.distance_nm, Some(12.3));
    assert_eq!(wnc.distance_km, Some(5.6));
    assert_eq!(wnc.destination_waypoint.as_deref(), Some("POINT1"));
    assert_eq!(wnc.origin_waypoint.as_deref(), Some("POINT2"));
}
