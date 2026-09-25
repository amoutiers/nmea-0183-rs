use nmea_0183_rs::nmea::sentences::Aca;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn aca_jrc_jhs_183_fixture_roundtrip() {
    // Fixture published in JRC JHS-183 test report 75918046, p. 36:
    // https://fcc.report/FCC-ID/CKEJHS-183/1846406.pdf
    const FIXTURE: &str =
        "$AIACA,8,5200.0,N,00200.0,W,5000.0,N,00300.0,W,1,2088,0,2060,0,0,1,B,1,103300.00*36";

    let frame = parse_frame(FIXTURE).expect("valid JRC JHS-183 ACA frame");
    let aca = Aca::parse(&frame.fields).expect("parse ACA");

    assert_eq!(aca.sequence, Some(8));
    assert_eq!(aca.ne_lat, Some(5200.0));
    assert_eq!(aca.ne_ns, Some('N'));
    assert_eq!(aca.ne_lon, Some(200.0));
    assert_eq!(aca.ne_ew, Some('W'));
    assert_eq!(aca.sw_lat, Some(5000.0));
    assert_eq!(aca.sw_ns, Some('N'));
    assert_eq!(aca.sw_lon, Some(300.0));
    assert_eq!(aca.sw_ew, Some('W'));
    assert_eq!(aca.transition_zone_size, Some(1));
    assert_eq!(aca.channel_a, Some(2088));
    assert_eq!(aca.channel_a_bandwidth, Some(0));
    assert_eq!(aca.channel_b, Some(2060));
    assert_eq!(aca.channel_b_bandwidth, Some(0));
    assert_eq!(aca.tx_rx_mode, Some(0));
    assert_eq!(aca.power_level, Some(1));
    assert_eq!(aca.information_source, Some('B'));
    assert_eq!(aca.in_use_flag, Some(1));
    assert_eq!(aca.in_use_change_time.as_deref(), Some("103300.00"));

    let sentence = aca.to_sentence("AI").expect("encode ACA");
    assert_eq!(sentence.trim(), FIXTURE);
    let frame = parse_frame(sentence.trim()).expect("re-parse ACA");
    assert_eq!(Aca::parse(&frame.fields), Some(aca));
}
