#![cfg(feature = "eve")]
use nmea_0183_rs::nmea::sentences::Eve;
use nmea_0183_rs::nmea::{NmeaEncodable, NmeaSentence};
use nmea_0183_rs::parse_frame;

#[test]
fn decode_encode() {
    let frame =
        parse_frame("$FREVE,000001,DZ00513,Fire Alarm On: TEST DZ201 Name*0A").expect("valid");
    let eve = Eve::parse(&frame.fields);
    let sentence = eve.to_sentence("FR").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let eve2 = Eve::parse(&frame2.fields);
    assert_eq!(eve, eve2);
}

#[test]
fn roundtrip() {
    let original = Eve {
        time: Some("000001".to_string()),
        tag_code: Some("DZ00513".to_string()),
        message: Some("Fire Alarm On: TEST DZ201 Name".to_string()),
    };
    let sentence = original.to_sentence("FR").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Eve::parse(&frame.fields);
    assert_eq!(original, parsed);
}

#[test]
fn dispatch() {
    let frame =
        parse_frame("$FREVE,000001,DZ00513,Fire Alarm On: TEST DZ201 Name*0A").expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Eve(_)));
}

#[test]
fn eve_values() {
    let frame =
        parse_frame("$FREVE,000001,DZ00513,Fire Alarm On: TEST DZ201 Name*0A").expect("valid");
    let eve = Eve::parse(&frame.fields);
    assert_eq!(eve.time.as_deref(), Some("000001"));
    assert_eq!(eve.tag_code.as_deref(), Some("DZ00513"));
    assert_eq!(
        eve.message.as_deref(),
        Some("Fire Alarm On: TEST DZ201 Name")
    );
}
