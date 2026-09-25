use nmea_0183_rs::nmea::sentences::Sm3;
use nmea_0183_rs::parse_frame;

#[test]
fn sm3_pynmeagps_values() {
    let frame =
        parse_frame("$CSSM3,123456,005213,798,0,3,14,00,2012,04,05,14,30,3400,N,076,W,300*39")
            .expect("valid SM3");
    let sm3 = Sm3::parse(&frame.fields).expect("parse SM3");
    assert_eq!(sm3.message_number, Some(123456));
    assert_eq!(sm3.les_sequence, Some(5213));
    assert_eq!(sm3.latitude, Some(3400.0));
    assert_eq!(sm3.longitude, Some(76.0));
    assert_eq!(sm3.radius, Some(300));
}
