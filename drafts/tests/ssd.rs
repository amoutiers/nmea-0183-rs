use nmea_0183_rs::nmea::sentences::Ssd;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn ssd_ocean_signal_atb1_values_checksum_and_roundtrip() {
    // Public fixture: Ocean Signal ATB1 AIS operational and performance test report,
    // https://fcc.report/FCC-ID/XYEATB1/4357980.pdf
    const FIXTURE: &str = "$AISSD,1234567,TUV TEST B,1,2,3,4,1,AI*42";

    let (body, checksum) = FIXTURE[1..].split_once('*').expect("fixture checksum");
    assert_eq!(
        body.bytes().fold(0_u8, |checksum, byte| checksum ^ byte),
        u8::from_str_radix(checksum, 16).expect("hex checksum")
    );

    let frame = parse_frame(FIXTURE).expect("valid Ocean Signal ATB1 SSD frame");
    let ssd = Ssd::parse(&frame.fields).expect("parse SSD");

    assert_eq!(ssd.call_sign.as_deref(), Some("1234567"));
    assert_eq!(ssd.ship_name.as_deref(), Some("TUV TEST B"));
    assert_eq!(ssd.dimension_a, Some(1));
    assert_eq!(ssd.dimension_b, Some(2));
    assert_eq!(ssd.dimension_c, Some(3));
    assert_eq!(ssd.dimension_d, Some(4));
    assert_eq!(ssd.dte_indicator, Some(1));
    assert_eq!(ssd.source.as_deref(), Some("AI"));

    let sentence = ssd.to_sentence("AI").expect("encode SSD");
    assert_eq!(sentence.trim(), FIXTURE);
    let roundtrip = parse_frame(sentence.trim()).expect("re-parse SSD");
    assert_eq!(Ssd::parse(&roundtrip.fields), Some(ssd));
}
