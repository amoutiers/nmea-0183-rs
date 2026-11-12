#[cfg(feature = "abm")]
#[path = "ais/abm.rs"]
mod abm;
#[cfg(feature = "bbm")]
#[path = "ais/bbm.rs"]
mod bbm;
#[cfg(feature = "ais")]
#[path = "ais/parser.rs"]
mod parser;
#[cfg(feature = "ais")]
#[path = "ais/transmit.rs"]
mod transmit;
#[cfg(feature = "ais")]
#[path = "ais/type1.rs"]
mod type1;
#[cfg(feature = "ais")]
#[path = "ais/type11.rs"]
mod type11;
#[cfg(feature = "ais")]
#[path = "ais/type12.rs"]
mod type12;
#[cfg(feature = "ais")]
#[path = "ais/type13.rs"]
mod type13;
#[cfg(feature = "ais")]
#[path = "ais/type14.rs"]
mod type14;
#[cfg(feature = "ais")]
#[path = "ais/type15.rs"]
mod type15;
#[cfg(feature = "ais")]
#[path = "ais/type18.rs"]
mod type18;
#[cfg(feature = "ais")]
#[path = "ais/type19.rs"]
mod type19;
#[cfg(feature = "ais")]
#[path = "ais/type21.rs"]
mod type21;
#[cfg(feature = "ais")]
#[path = "ais/type24.rs"]
mod type24;
#[cfg(feature = "ais")]
#[path = "ais/type27.rs"]
mod type27;
#[cfg(feature = "ais")]
#[path = "ais/type4.rs"]
mod type4;
#[cfg(feature = "ais")]
#[path = "ais/type5.rs"]
mod type5;
#[cfg(feature = "ais")]
#[path = "ais/type6.rs"]
mod type6;
#[cfg(feature = "ais")]
#[path = "ais/type7.rs"]
mod type7;
#[cfg(feature = "ais")]
#[path = "ais/type8.rs"]
mod type8;
#[cfg(feature = "ais")]
#[path = "ais/type9.rs"]
mod type9;

#[cfg(feature = "ais")]
use nmea_0183_rs::ais::{AisDecodeOutcome, AisMessage};

#[cfg(feature = "ais")]
fn expect_message(outcome: AisDecodeOutcome) -> AisMessage {
    match outcome {
        AisDecodeOutcome::Message(message) => message,
        other => panic!("expected complete AIS message, got {other:?}"),
    }
}

#[cfg(feature = "ais")]
#[path = "ais/type25.rs"]
mod type25;

#[cfg(feature = "ais")]
#[path = "ais/type26.rs"]
mod type26;

#[cfg(feature = "ais")]
#[path = "ais/type22.rs"]
mod type22;

// Independent bit-layout probes; these are not device captures.
#[cfg(feature = "ais")]
fn set_ais_bits(bits: &mut [u8], offset: usize, width: usize, value: u32) {
    assert!((1..=32).contains(&width));
    assert!(offset + width <= bits.len());
    assert!(width == 32 || value < (1_u32 << width));
    for (index, bit) in bits[offset..offset + width].iter_mut().enumerate() {
        *bit = ((value >> (width - index - 1)) & 1) as u8;
    }
}

#[cfg(feature = "ais")]
fn ais_frame_from_bits(bits: &[u8]) -> String {
    assert!(bits.iter().all(|&bit| bit <= 1));
    let fill = (6 - bits.len() % 6) % 6;
    let mut padded = bits.to_vec();
    padded.resize(bits.len() + fill, 0);
    let payload: String = padded
        .chunks_exact(6)
        .map(|chunk| {
            let value = chunk.iter().fold(0_u8, |value, bit| (value << 1) | bit);
            char::from(if value < 40 { value + 48 } else { value + 56 })
        })
        .collect();
    let body = format!("AIVDM,1,1,,A,{payload},{fill}");
    let checksum = body.bytes().fold(0_u8, |value, byte| value ^ byte);
    format!("!{body}*{checksum:02X}")
}

#[cfg(feature = "ais")]
#[test]
fn bit_probe_oracle_matches_fixed_frames() {
    assert_eq!(
        ais_frame_from_bits(&[0, 0, 0, 0, 0, 1]),
        "!AIVDM,1,1,,A,1,0*17"
    );
    assert_eq!(
        ais_frame_from_bits(&[0, 0, 0, 0, 1]),
        "!AIVDM,1,1,,A,2,1*15"
    );
    let mut bits = [0; 6];
    set_ais_bits(&mut bits, 0, 6, 1);
    assert_eq!(bits, [0, 0, 0, 0, 0, 1]);
}

#[cfg(feature = "ais")]
#[path = "ais/type20.rs"]
mod type20;

#[cfg(feature = "ais")]
#[path = "ais/type17.rs"]
mod type17;

#[cfg(feature = "ais")]
#[path = "ais/type23.rs"]
mod type23;

#[path = "support/bounded.rs"]
mod bounded;
