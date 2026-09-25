use nmea_0183_rs::nmea::sentences::Abk;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn abk_ocean_signal_atb1_values_and_roundtrip() {
    // Public fixture: Ocean Signal ATB1 FCC test report,
    // https://fcc.report/FCC-ID/XYEATB1/4357980.pdf
    let fixture = "$AIABK,200300900,A,6,2,0*21";
    let frame = parse_frame(fixture).expect("valid Ocean Signal ATB1 ABK frame");
    let abk = Abk::parse(&frame.fields).expect("parse ABK");

    assert_eq!(abk.mmsi.as_deref(), Some("200300900"));
    assert_eq!(abk.channel, Some('A'));
    assert_eq!(abk.message_id, Some(6));
    assert_eq!(abk.sequence.as_deref(), Some("2"));
    assert_eq!(abk.ack_type, Some(0));

    let sentence = abk.to_sentence("AI").expect("encode");
    assert_eq!(sentence.trim(), fixture);
    let roundtrip = parse_frame(sentence.trim()).expect("re-parse");
    assert_eq!(Abk::parse(&roundtrip.fields), Some(abk));
}
