use nmea_0183_rs::nmea::sentences::Air;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

fn checksum(body: &str) -> u8 {
    body.bytes().fold(0, |value, byte| value ^ byte)
}

#[test]
fn air_bsh_fixture_values_checksum_and_roundtrip() {
    // Public fixture: BSH/FCC AIS test report 734.2/0048-1/2003, p. 22,
    // https://fcc.report/FCC-ID/Q8Z13EAQ8Z/346124.pdf
    // The report's payload ends with one extra empty field; *4C is its XOR checksum.
    const FIXTURE: &str = "$AIAIR,000005002,3,,5,,000007001,5,,*4C";

    let (body, reported_checksum) = FIXTURE[1..].split_once('*').expect("checksum separator");
    assert_eq!(
        checksum(body),
        u8::from_str_radix(reported_checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid BSH AIR frame");
    let air = Air::parse(&frame.fields).expect("parse AIR");

    assert_eq!(air.station_1_mmsi.as_deref(), Some("000005002"));
    assert_eq!(air.station_1_message_1.as_deref(), Some("3"));
    assert_eq!(air.station_1_message_1_subsection, None);
    assert_eq!(air.station_1_message_2.as_deref(), Some("5"));
    assert_eq!(air.station_1_message_2_subsection, None);
    assert_eq!(air.station_2_mmsi.as_deref(), Some("000007001"));
    assert_eq!(air.station_2_message_1.as_deref(), Some("5"));
    assert_eq!(air.station_2_message_1_subsection, None);

    let sentence = air.to_sentence("AI").expect("encode AIR");
    assert_eq!(sentence.trim(), "$AIAIR,000005002,3,,5,,000007001,5,*60");
    let roundtrip = parse_frame(sentence.trim()).expect("re-parse AIR");
    assert_eq!(Air::parse(&roundtrip.fields), Some(air));
}
