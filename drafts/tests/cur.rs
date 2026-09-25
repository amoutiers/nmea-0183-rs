use nmea_0183_rs::nmea::sentences::Cur;
use nmea_0183_rs::{NmeaEncodable, parse_frame};

#[test]
fn cur_aanderaa_seaguardii_fixture_roundtrip() {
    // Fixture published in Aanderaa TD 303 SeaGuardII Platform Operating Manual, p. 49:
    // https://www.xylem.com/siteassets/brand/aanderaa/resources/manual/td303-seaguard-ii-platform.pdf
    const FIXTURE: &str = "$WICUR,A,0,0,0.000000,295.695587,T,5.222386,0.000000,0.000000,T,B*6F";

    let body = FIXTURE
        .strip_prefix('$')
        .and_then(|sentence| sentence.split_once('*').map(|(body, _)| body))
        .expect("NMEA fixture has body and checksum");
    let checksum = body.bytes().fold(0u8, |checksum, byte| checksum ^ byte);
    assert_eq!(checksum, 0x6F);

    let frame = parse_frame(FIXTURE).expect("valid SeaGuardII CUR frame");
    let cur = Cur::parse(&frame.fields).expect("parse CUR");

    assert_eq!(cur.validity, Some('A'));
    assert_eq!(cur.data_set_number, Some(0));
    assert_eq!(cur.layer_number, Some(0));
    assert!((cur.depth.expect("depth") - 0.0).abs() < 0.000_1);
    assert!((cur.direction.expect("direction") - 295.695587).abs() < 0.000_1);
    assert_eq!(cur.direction_reference, Some('T'));
    assert!((cur.speed.expect("speed") - 5.222386).abs() < 0.000_1);
    assert!((cur.reference_depth.expect("reference depth") - 0.0).abs() < 0.000_1);
    assert!((cur.heading.expect("heading") - 0.0).abs() < 0.000_1);
    assert_eq!(cur.heading_reference, Some('T'));
    assert_eq!(cur.speed_reference, Some('B'));

    let sentence = cur.to_sentence("WI").expect("encode CUR");
    let frame = parse_frame(sentence.trim()).expect("re-parse CUR");
    let reparsed = Cur::parse(&frame.fields).expect("parse re-encoded CUR");

    assert_eq!(reparsed.validity, cur.validity);
    assert_eq!(reparsed.data_set_number, cur.data_set_number);
    assert_eq!(reparsed.layer_number, cur.layer_number);
    assert!(
        (reparsed.depth.expect("re-encoded depth") - cur.depth.expect("depth")).abs() < 0.000_1
    );
    assert!(
        (reparsed.direction.expect("re-encoded direction") - cur.direction.expect("direction"))
            .abs()
            < 0.000_1
    );
    assert_eq!(reparsed.direction_reference, cur.direction_reference);
    assert!(
        (reparsed.speed.expect("re-encoded speed") - cur.speed.expect("speed")).abs() < 0.000_1
    );
    assert!(
        (reparsed
            .reference_depth
            .expect("re-encoded reference depth")
            - cur.reference_depth.expect("reference depth"))
        .abs()
            < 0.000_1
    );
    assert!(
        (reparsed.heading.expect("re-encoded heading") - cur.heading.expect("heading")).abs()
            < 0.000_1
    );
    assert_eq!(reparsed.heading_reference, cur.heading_reference);
    assert_eq!(reparsed.speed_reference, cur.speed_reference);
}
