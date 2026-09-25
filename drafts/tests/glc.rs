use nmea_0183_rs::nmea::sentences::Glc;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn glc_ion_fixture_roundtrip() {
    // Fixture published in James Carroll, "Performance Analysis of an Integrated
    // GPS/Loran-C Tracking System", ION AM 2005, Fig. 3, p. 4:
    // https://www.loran.org/proceedings/Meeting2005/Session%207%20-%20eLoran%20Applications/Carroll_ILA34.pdf
    const FIXTURE: &str =
        "$LCGLC,9610,0.0,A,,,29658.0232,A,41180.0079,A,58421.9709,A,72894.2738,A*08";

    let body = FIXTURE
        .strip_prefix('$')
        .and_then(|sentence| sentence.split_once('*').map(|(body, _)| body))
        .expect("NMEA fixture has body and checksum");
    let checksum = body.bytes().fold(0u8, |checksum, byte| checksum ^ byte);
    assert_eq!(checksum, 0x08);

    let frame = parse_frame(FIXTURE).expect("valid ION GLC frame");
    let glc = Glc::parse(&frame.fields).expect("parse GLC");

    assert_eq!(glc.gri, Some(9610));
    assert!((glc.master_toa.expect("master TOA") - 0.0).abs() < 0.000_1);
    assert_eq!(glc.master_status, Some('A'));
    assert_eq!(glc.td1, None);
    assert_eq!(glc.td1_status, None);
    assert!((glc.td2.expect("TD2") - 29658.0232).abs() < 0.001);
    assert_eq!(glc.td2_status, Some('A'));
    assert!((glc.td3.expect("TD3") - 41180.0079).abs() < 0.001);
    assert_eq!(glc.td3_status, Some('A'));
    assert!((glc.td4.expect("TD4") - 58421.9709).abs() < 0.001);
    assert_eq!(glc.td4_status, Some('A'));
    assert!((glc.td5.expect("TD5") - 72894.2738).abs() < 0.001);
    assert_eq!(glc.td5_status, Some('A'));

    let sentence = glc.to_sentence("LC").expect("encode GLC");
    let frame = parse_frame(sentence.trim()).expect("re-parse GLC");
    assert_eq!(Glc::parse(&frame.fields), Some(glc));
}
