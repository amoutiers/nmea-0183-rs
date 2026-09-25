use nmea_0183_rs::nmea::sentences::Msk;
use nmea_0183_rs::parse_frame;

#[test]
fn msk_pynmeagps_values() {
    let frame = parse_frame("$GPMSK,318.0,A,100,M,2*45").expect("valid MSK");
    let msk = Msk::parse(&frame.fields).expect("parse MSK");
    assert_eq!(msk.frequency, Some(318.0));
    assert_eq!(msk.frequency_mode, Some('A'));
    assert_eq!(msk.beacon_bps, Some(100));
    assert_eq!(msk.mms_frequency, Some(2.0));
    assert!(msk.channel.is_none() && msk.status.is_none());
}
