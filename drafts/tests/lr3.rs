use nmea_0183_rs::nmea::sentences::Lr3;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn lr3_ocean_signal_ata100_values_and_checksum() {
    // Public fixture: Ocean Signal ATA100 IEC 61162-1 digital-interface test report,
    // TUV SUD document 75936860-03 Issue 1, page 28,
    // https://device.report/m/99fb76110111076f2db1da73026b991c2eb919d5296ac12a27d35cb386d4c937_doc
    const FIXTURE: &str =
        "$AILR3,6,777888999,SOUTHAMPTON,111219,140000.00,8.0,52,50.0,10.0,52.0,90*40";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid Ocean Signal ATA100 LR3 frame");
    let lr3 = Lr3::parse(&frame.fields).expect("parse LR3");

    assert_eq!(lr3.sequence.as_deref(), Some("6"));
    assert_eq!(lr3.mmsi.as_deref(), Some("777888999"));
    assert_eq!(lr3.destination.as_deref(), Some("SOUTHAMPTON"));
    assert_eq!(lr3.eta_date.as_deref(), Some("111219"));
    assert_eq!(lr3.eta_time.as_deref(), Some("140000.00"));
    assert_eq!(lr3.draught, Some(8.0));
    assert_eq!(lr3.ship_cargo_type.as_deref(), Some("52"));
    assert_eq!(lr3.ship_length, Some(50.0));
    assert_eq!(lr3.ship_breadth, Some(10.0));
    assert_eq!(lr3.ship_type, Some(52.0));
    assert_eq!(lr3.persons, Some(90));
}

#[test]
fn lr3_roundtrip() {
    let frame =
        parse_frame("$AILR3,6,777888999,SOUTHAMPTON,111219,140000.00,8.0,52,50.0,10.0,52.0,90*40")
            .expect("valid LR3");
    let original = Lr3::parse(&frame.fields).expect("parse LR3");

    let sentence = original.to_sentence("AI").expect("encode LR3");
    let frame = parse_frame(sentence.trim()).expect("re-parse LR3");
    assert_eq!(Lr3::parse(&frame.fields), Some(original));
}
