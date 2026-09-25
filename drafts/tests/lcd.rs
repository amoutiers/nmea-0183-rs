use nmea_0183_rs::nmea::sentences::Lcd;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn lcd_ion_fixture_roundtrip() {
    // Fixture published in James Carroll, "Performance Analysis of an Integrated
    // GPS/Loran-C Tracking System", ION AM 2005, Fig. 3, p. 4:
    // https://www.loran.org/proceedings/Meeting2005/Session%207%20-%20eLoran%20Applications/Carroll_ILA34.pdf
    const FIXTURE: &str = "$LCLCD,9610,129,071,,,115,071,136,030,106,-191,107,-412*61";

    let body = FIXTURE
        .strip_prefix('$')
        .and_then(|sentence| sentence.split_once('*').map(|(body, _)| body))
        .expect("NMEA fixture has body and checksum");
    let checksum = body.bytes().fold(0u8, |checksum, byte| checksum ^ byte);
    assert_eq!(checksum, 0x61);

    let frame = parse_frame(FIXTURE).expect("valid ION LCD frame");
    let lcd = Lcd::parse(&frame.fields).expect("parse LCD");

    assert_eq!(lcd.gri, Some(9610));
    assert_eq!(lcd.master_snr, Some(129));
    assert_eq!(lcd.master_ecd, Some(71));
    assert_eq!(lcd.secondary_1_snr, None);
    assert_eq!(lcd.secondary_1_ecd, None);
    assert_eq!(lcd.secondary_2_snr, Some(115));
    assert_eq!(lcd.secondary_2_ecd, Some(71));
    assert_eq!(lcd.secondary_3_snr, Some(136));
    assert_eq!(lcd.secondary_3_ecd, Some(30));
    assert_eq!(lcd.secondary_4_snr, Some(106));
    assert_eq!(lcd.secondary_4_ecd, Some(-191));
    assert_eq!(lcd.secondary_5_snr, Some(107));
    assert_eq!(lcd.secondary_5_ecd, Some(-412));

    let sentence = lcd.to_sentence("LC").expect("encode LCD");
    let frame = parse_frame(sentence.trim()).expect("re-parse LCD");
    assert_eq!(Lcd::parse(&frame.fields), Some(lcd));
}
