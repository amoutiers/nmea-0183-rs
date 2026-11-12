#![cfg(feature = "ais")]
use super::expect_message;
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::parse_frame;

#[test]
fn type12_safety_addressed_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,<5?SIj1;GbD07??4,0*38").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::SafetyAddressed(sa) = msg {
        assert!(sa.mmsi > 0);
        assert!(sa.dest_mmsi > 0);
    } else {
        panic!("expected SafetyAddressed (type 12), got {msg:?}");
    }
}

#[test]
fn type12_with_text_gpsd() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,<42Lati0W:Ov=C7P6B?=Pjoihhjhqq0,2*2B").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    if let AisMessage::SafetyAddressed(sa) = msg {
        assert!(!sa.text.is_empty(), "should have safety text");
    } else {
        panic!("expected SafetyAddressed, got {msg:?}");
    }
}

#[test]
fn type12_values() {
    let mut parser = AisParser::new();
    let frame = parse_frame("!AIVDM,1,1,,A,<42Lati0W:Ov=C7P6B?=Pjoihhjhqq0,2*2B").expect("valid");
    let msg = expect_message(parser.decode(&frame).expect("decoded"));
    match msg {
        AisMessage::SafetyAddressed(sa) => {
            assert_eq!(sa.mmsi, 271002099);
            assert_eq!(sa.text, "MSG FROM 271002099");
        }
        other => panic!("expected SafetyAddressed, got {other:?}"),
    }
}

// Synthetic one-hot probes for the public flags at their documented bit offsets.
#[test]
fn type12_flags_are_exact() {
    let kind = 12;
    for (set_bit, expected) in [(None, [false]), (Some(70), [true])] {
        let mut bits = vec![0; 72];
        super::set_ais_bits(&mut bits, 0, 6, kind);
        if let Some(bit) = set_bit {
            super::set_ais_bits(&mut bits, bit, 1, 1);
        }
        let wire = super::ais_frame_from_bits(&bits);
        match expect_message(
            AisParser::new()
                .decode(&parse_frame(&wire).expect("frame"))
                .expect("decode"),
        ) {
            AisMessage::SafetyAddressed(message) => assert_eq!(
                [message.retransmit],
                expected,
                "type {kind}, bit {set_bit:?}"
            ),
            other => panic!("unexpected {other:?}"),
        }
    }
}
