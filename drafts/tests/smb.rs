use nmea_0183_rs::nmea::sentences::Smb;
use nmea_0183_rs::parse_frame;

#[test]
fn smb_pynmeagps_values() {
    let frame = parse_frame(
        "$CSSMB,008,001,0,123456,FROM:Maritime Rescue Coordination Centre xxx^0D)0ATO:*48",
    )
    .expect("valid SMB");
    let smb = Smb::parse(&frame.fields).expect("parse SMB");
    assert_eq!(smb.total_sentences, Some(8));
    assert_eq!(smb.sentence_number, Some(1));
    assert_eq!(smb.message_number, Some(123456));
    assert!(
        smb.body
            .as_deref()
            .is_some_and(|body| body.starts_with("FROM:"))
    );
}
