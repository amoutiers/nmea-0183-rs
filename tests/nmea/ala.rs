#![cfg(feature = "ala")]
use nmea_0183_rs::nmea::{NmeaEncodable, sentences::Ala};
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn ala_values() {
    let frame = parse_frame("$FRALA,143955,FR,OT,00,901,N,V,Syst Fault : AutroSafe comm. OK*4F")
        .expect("valid");
    let ala = Ala::parse(&frame.fields);
    assert_eq!(ala.time.as_deref(), Some("143955"));
    assert_eq!(ala.system.as_deref(), Some("FR"));
    assert_eq!(ala.subsystem.as_deref(), Some("OT"));
    assert_eq!(ala.instance.as_deref(), Some("00"));
    assert_eq!(ala.alarm_type.as_deref(), Some("901"));
    assert_eq!(ala.condition, Some('N'));
    assert_eq!(ala.ack_state, Some('V'));
    assert_eq!(
        ala.message.as_deref(),
        Some("Syst Fault : AutroSafe comm. OK")
    );
}

#[test]
fn dispatch() {
    let frame = parse_frame("$FRALA,143955,FR,OT,00,901,N,V,Syst Fault : AutroSafe comm. OK*4F")
        .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Ala(_)));
}

#[test]
fn roundtrip() {
    let original = Ala {
        time: Some("143955".into()),
        system: Some("FR".into()),
        subsystem: Some("OT".into()),
        instance: Some("00".into()),
        alarm_type: Some("901".into()),
        condition: Some('N'),
        ack_state: Some('V'),
        message: Some("Test alarm".into()),
    };
    let sentence = original.to_sentence("FR").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    assert_eq!(Ala::parse(&frame.fields), original);
}
