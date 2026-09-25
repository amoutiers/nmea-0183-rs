use nmea_0183_rs::nmea::sentences::Mob;
use nmea_0183_rs::parse_frame;

#[test]
fn mob_marnav_values() {
    let frame = parse_frame(
        "$INMOB,ABCDE,V,123456.987,0,311219,132456.876,12.34,N,123.45,E,12.34,23.45,269104520,0*53",
    )
    .expect("valid MOB");
    let mob = Mob::parse(&frame.fields).expect("parse MOB");
    assert_eq!(mob.emitter_id.as_deref(), Some("ABCDE"));
    assert_eq!(mob.activation_date, Some(311219));
    assert_eq!(mob.lat, Some(12.34));
    assert_eq!(mob.mmsi.as_deref(), Some("269104520"));
}
