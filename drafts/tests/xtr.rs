use nmea_0183_rs::nmea::sentences::Xtr;
use nmea_0183_rs::parse_frame;

#[test]
fn xtr_marnav_empty_values() {
    let frame = parse_frame("$GPXTR,,,*65").expect("valid XTR");
    assert_eq!(
        Xtr::parse(&frame.fields),
        Some(Xtr {
            cross_track_error: None,
            steer_direction: None,
            unit: None
        })
    );
}
