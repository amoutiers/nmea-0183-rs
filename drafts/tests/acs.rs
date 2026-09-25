use nmea_0183_rs::nmea::sentences::Acs;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn acs_ocean_signal_atb1_values_and_roundtrip() {
    // Public fixture: Ocean Signal ATB1 FCC test report,
    // https://fcc.report/FCC-ID/XYEATB1/4357980.pdf
    let fixture = "$AIACS,3,4661000,142720.00,01,04,2019*7C";
    let frame = parse_frame(fixture).expect("valid Ocean Signal ATB1 ACS frame");
    let acs = Acs::parse(&frame.fields).expect("parse ACS");

    assert_eq!(acs.sequence, Some(3));
    assert_eq!(acs.mmsi.as_deref(), Some("4661000"));
    assert_eq!(acs.time.as_deref(), Some("142720.00"));
    assert_eq!(acs.day.as_deref(), Some("01"));
    assert_eq!(acs.month.as_deref(), Some("04"));
    assert_eq!(acs.year.as_deref(), Some("2019"));

    let sentence = acs.to_sentence("AI").expect("encode");
    assert_eq!(sentence.trim(), fixture);
    let roundtrip = parse_frame(sentence.trim()).expect("re-parse");
    assert_eq!(Acs::parse(&roundtrip.fields), Some(acs));
}
