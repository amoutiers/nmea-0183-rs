use nmea_0183_rs::nmea::sentences::Mss;
use nmea_0183_rs::parse_frame;

#[test]
fn mss_pynmeagps_values() {
    let frame = parse_frame("$GPMSS,55,27,318.0,100,*66").expect("valid MSS");
    assert_eq!(
        Mss::parse(&frame.fields),
        Some(Mss {
            strength: Some(55),
            snr: Some(27),
            frequency: Some(318.0),
            beacon_bps: Some(100),
            channel: None
        })
    );
}
