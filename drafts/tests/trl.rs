use nmea_0183_rs::nmea::sentences::Trl;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn trl_tuv_ocean_signal_fixture_values_and_checksum() {
    // Fixture published in TÜV SÜD's Ocean Signal Class A report, page 28:
    // https://device.report/m/99fb76110111076f2db1da73026b991c2eb919d5296ac12a27d35cb386d4c937_doc
    const FIXTURE: &str = "$AITRL,10,2,2,12122019,153000,12122019,155100,1*75";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid TÜV/Ocean Signal TRL frame");
    let trl = Trl::parse(&frame.fields).expect("parse TRL");

    assert_eq!(trl.total_log_entries, Some(10));
    assert_eq!(trl.log_entry_number, Some(2));
    assert_eq!(trl.sequence_id.as_deref(), Some("2"));
    assert_eq!(trl.switch_off_date.as_deref(), Some("12122019"));
    assert_eq!(trl.switch_off_time.as_deref(), Some("153000"));
    assert_eq!(trl.switch_on_date.as_deref(), Some("12122019"));
    assert_eq!(trl.switch_on_time.as_deref(), Some("155100"));
    assert_eq!(trl.reason_code, Some(1));
}

#[test]
fn trl_roundtrip() {
    let frame =
        parse_frame("$AITRL,10,2,2,12122019,153000,12122019,155100,1*75").expect("valid TRL");
    let original = Trl::parse(&frame.fields).expect("parse TRL");

    let sentence = original.to_sentence("AI").expect("encode TRL");
    let frame = parse_frame(sentence.trim()).expect("re-parse TRL");
    assert_eq!(Trl::parse(&frame.fields), Some(original));
}
