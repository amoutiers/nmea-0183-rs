use nmea_0183_rs::nmea::sentences::Smv;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn smv_iec_61162_1_2024_multipart_fixture_values_and_checksum() {
    // Public fixture: IEC 61162-1:2024, 8.3.97, page 128,
    // https://previewnorm.com/iec/IEC%2061162-1-2024%20PDF.pdf
    // The published Example 3 has `1234.5678.N`; 8.3.97 defines N/S as its
    // own field, so the separator is corrected to `,`. Consequently, its XOR
    // checksum is `37`, rather than the published `35` for the typo.
    const FIXTURE: &str =
        "$CSSMV,2,1,5,12,123456789,,1234.5678,N,12345.6789,W,2018,01,23,12,34,D*37";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid IEC SMV frame");
    let smv = Smv::parse(&frame.fields).expect("parse SMV");

    assert_eq!(smv.total_sentences, Some(2));
    assert_eq!(smv.sentence_number, Some(1));
    assert_eq!(smv.sequential_message_id, Some(5));
    assert_eq!(smv.message_number, Some(12));
    assert_eq!(smv.mmsi.as_deref(), Some("123456789"));
    assert!(smv.vessel_name.is_none());
    assert_eq!(smv.latitude, Some(1234.5678));
    assert_eq!(smv.ns, Some('N'));
    assert_eq!(smv.longitude, Some(12345.6789));
    assert_eq!(smv.ew, Some('W'));
    assert_eq!(smv.year.as_deref(), Some("2018"));
    assert_eq!(smv.month.as_deref(), Some("01"));
    assert_eq!(smv.day.as_deref(), Some("23"));
    assert_eq!(smv.hour.as_deref(), Some("12"));
    assert_eq!(smv.minute.as_deref(), Some("34"));
    assert_eq!(smv.status, Some('D'));

    let cancelled = parse_frame("$CSSMV,1,,5,12,123456789,,,,,,,,,,,C*2D")
        .expect("valid IEC SMV cancellation frame");
    assert_eq!(
        Smv::parse(&cancelled.fields)
            .expect("parse cancelled SMV")
            .status,
        Some('C')
    );
}

#[test]
fn smv_roundtrip() {
    let frame =
        parse_frame("$CSSMV,2,1,5,12,123456789,,1234.5678,N,12345.6789,W,2018,01,23,12,34,D*37")
            .expect("valid SMV");
    let original = Smv::parse(&frame.fields).expect("parse SMV");

    let sentence = original.to_sentence("CS").expect("encode SMV");
    let frame = parse_frame(sentence.trim()).expect("re-parse SMV");
    assert_eq!(Smv::parse(&frame.fields), Some(original));
}
