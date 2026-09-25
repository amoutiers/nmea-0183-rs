use nmea_0183_rs::nmea::sentences::Rlm;
use nmea_0183_rs::parse_frame;

#[test]
fn rlm_pynmeagps_values() {
    let frame = parse_frame("$GNRLM,00000078A9FBAD5,083559.00,3,C45B*06").expect("valid RLM");
    assert_eq!(
        Rlm::parse(&frame.fields),
        Some(Rlm {
            beacon: Some("00000078A9FBAD5".into()),
            time: Some("083559.00".into()),
            code: Some('3'),
            body: Some("C45B".into())
        })
    );
}
