use nmea_0183_rs::nmea::sentences::Nrx;
use nmea_0183_rs::parse_frame;

#[test]
fn nrx_pynmeagps_values() {
    let frame = parse_frame(
        "$CRNRX,007,001,00,IE69,1,135600,27,06,2001,241,3,A,============================*09",
    )
    .expect("valid NRX");
    let nrx = Nrx::parse(&frame.fields).expect("parse NRX");
    assert_eq!(nrx.total_sentences, Some(7));
    assert_eq!(nrx.sentence_number, Some(1));
    assert_eq!(nrx.message_code.as_deref(), Some("IE69"));
    assert_eq!(nrx.year, Some(2001));
    assert_eq!(nrx.body.as_deref(), Some("============================"));
}
