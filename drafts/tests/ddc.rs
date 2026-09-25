use nmea_0183_rs::nmea::sentences::Ddc;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

fn checksum(body: &str) -> u8 {
    body.bytes().fold(0, |value, byte| value ^ byte)
}

#[test]
fn ddc_iec_pas_fixture_values_checksum_and_roundtrip() {
    // Public fixture: IEC PAS 61162-103:2021, 2.2.1, example 2,
    // https://cdn.standards.iteh.ai/samples/104987/3a9990f3b9264960b6c0ed2637345655/IEC-PAS-61162-103-2021.pdf
    const FIXTURE: &str = "$IIDDC,D,75,D,C,P*7E";

    let (body, reported_checksum) = FIXTURE[1..].split_once('*').expect("checksum separator");
    assert_eq!(
        checksum(body),
        u8::from_str_radix(reported_checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid IEC DDC frame");
    let ddc = Ddc::parse(&frame.fields).expect("parse DDC");

    assert_eq!(ddc.preset, Some('D'));
    assert_eq!(ddc.brightness, Some(75));
    assert_eq!(ddc.palette, Some('D'));
    assert_eq!(ddc.sentence_status, Some('C'));
    assert_eq!(ddc.command_mode, Some('P'));

    let sentence = ddc.to_sentence("II").expect("encode DDC");
    assert_eq!(sentence.trim(), FIXTURE);
    let roundtrip = parse_frame(sentence.trim()).expect("re-parse DDC");
    assert_eq!(Ddc::parse(&roundtrip.fields), Some(ddc));
}

#[test]
fn ddc_legacy_encoding_omits_command_mode() {
    let ddc = Ddc {
        preset: Some('N'),
        brightness: Some(5),
        palette: Some('N'),
        sentence_status: Some('C'),
        command_mode: None,
    };

    let sentence = ddc.to_sentence("II").expect("encode legacy DDC");
    let frame = parse_frame(sentence.trim()).expect("parse legacy DDC");
    assert_eq!(frame.fields.as_slice(), ["N", "05", "N", "C"]);
    assert_eq!(Ddc::parse(&frame.fields), Some(ddc));
}
