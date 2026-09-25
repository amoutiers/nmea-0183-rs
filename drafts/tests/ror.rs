use nmea_0183_rs::nmea::sentences::Ror;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn ror_pynmea2_ship_fixture_values_and_checksum() {
    // Fixture published in the pynmea2-ship README:
    // https://github.com/acousticwave/pynmea2-ship
    const FIXTURE: &str = "$AGROR,0.1,A,,V,B*1F";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid pynmea2-ship ROR frame");
    let ror = Ror::parse(&frame.fields).expect("parse ROR");

    assert_eq!(ror.starboard_order, Some(0.1));
    assert_eq!(ror.starboard_status, Some('A'));
    assert!(ror.port_order.is_none());
    assert_eq!(ror.port_status, Some('V'));
    assert_eq!(ror.source, Some('B'));
}

#[test]
fn ror_roundtrip() {
    let frame = parse_frame("$AGROR,0.1,A,,V,B*1F").expect("valid ROR");
    let original = Ror::parse(&frame.fields).expect("parse ROR");

    let sentence = original.to_sentence("AG").expect("encode ROR");
    let frame = parse_frame(sentence.trim()).expect("re-parse ROR");
    assert_eq!(Ror::parse(&frame.fields), Some(original));
}
