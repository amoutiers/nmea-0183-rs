use nmea_0183_rs::nmea::sentences::Nak;
use nmea_0183_rs::parse_frame;

#[test]
fn nak_pynmeagps_values() {
    let frame = parse_frame("$ANNAK,UP,AID,999999990,11,*1E").expect("valid NAK");
    let nak = Nak::parse(&frame.fields).expect("parse NAK");
    assert_eq!(nak.talker_id.as_deref(), Some("UP"));
    assert_eq!(nak.identifier.as_deref(), Some("999999990"));
    assert_eq!(nak.reason.as_deref(), Some("11"));
    assert!(nak.text.is_none());
}
