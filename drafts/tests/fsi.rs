use nmea_0183_rs::nmea::sentences::Fsi;
use nmea_0183_rs::parse_frame;

#[test]
fn fsi_marnav_values() {
    let frame = parse_frame("$GPFSI,156000,156025,,,*60").expect("valid FSI");
    let fsi = Fsi::parse(&frame.fields).expect("parse FSI");
    assert_eq!(fsi.transmit_frequency.as_deref(), Some("156000"));
    assert_eq!(fsi.receive_frequency.as_deref(), Some("156025"));
    assert!(fsi.operating_mode.is_none() && fsi.power.is_none() && fsi.status.is_none());
}
