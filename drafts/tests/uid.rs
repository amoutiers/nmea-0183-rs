use nmea_0183_rs::nmea::sentences::Uid;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn uid_iec_61162_1_2024_fixture_values_checksum_and_roundtrip() {
    // Public fixture: IEC 61162-1:2024, 8.3.111, page 139,
    // https://previewnorm.com/iec/IEC%2061162-1-2024%20PDF.pdf
    const FIXTURE: &str = "$GPUID,HEPSLGN02376,DB Los 23*63";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid IEC UID frame");
    let uid = Uid::parse(&frame.fields).expect("parse UID");

    assert_eq!(uid.uid1.as_deref(), Some("HEPSLGN02376"));
    assert_eq!(uid.uid2.as_deref(), Some("DB Los 23"));

    let sentence = uid.to_sentence("GP").expect("encode UID");
    assert_eq!(sentence.trim(), FIXTURE);
    let roundtrip = parse_frame(sentence.trim()).expect("re-parse UID");
    assert_eq!(Uid::parse(&roundtrip.fields), Some(uid));
}
