use nmea_0183_rs::nmea::sentences::Epv;
use nmea_0183_rs::parse_frame;

#[test]
fn epv_pynmeagps_values() {
    let frame = parse_frame("$AIEPV,R,AI,503123450,101,38400*05").expect("valid EPV");
    assert_eq!(
        Epv::parse(&frame.fields),
        Some(Epv {
            status: Some('R'),
            equipment_type: Some("AI".into()),
            equipment_id: Some("503123450".into()),
            property_id: Some("101".into()),
            value: Some("38400".into())
        })
    );
}
