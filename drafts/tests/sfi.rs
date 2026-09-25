use nmea_0183_rs::nmea::sentences::Sfi;
use nmea_0183_rs::parse_frame;

#[test]
fn sfi_marnav_values() {
    let frame = parse_frame("$GPSFI,1,1,156025,M*03").expect("valid SFI");
    let sfi = Sfi::parse(&frame.fields).expect("parse SFI");
    assert_eq!(sfi.total_sentences, Some(1));
    assert_eq!(sfi.sentence_number, Some(1));
    assert_eq!(sfi.frequencies, [(Some(156025), Some("M".into()))]);
}
