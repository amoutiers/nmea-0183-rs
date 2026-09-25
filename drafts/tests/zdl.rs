use nmea_0183_rs::nmea::sentences::Zdl;
use nmea_0183_rs::parse_frame;

#[test]
fn zdl_marnav_values() {
    let frame = parse_frame("$GPZDL,383401,12.3,R*28").expect("valid ZDL");
    assert_eq!(
        Zdl::parse(&frame.fields),
        Some(Zdl {
            time: Some("383401".into()),
            distance: Some(12.3),
            point_type: Some('R')
        })
    );
}
