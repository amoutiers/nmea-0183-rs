//! AIS safety-related broadcast message — Type 14.
//!
//! Transmitted by vessels and shore stations (coast guard, port authority) to broadcast
//! safety-critical text messages. Long text payloads may span multiple AIVDM/AIVDO sentences.

use crate::ais::armor::{extract_string, extract_u32};

/// AIS Type 14 — Safety-Related Broadcast Message.
///
/// ITU-R M.1371 bit layout:
/// - bits  0–5:  message type (= 14)
/// - bits  6–7:  repeat indicator
/// - bits  8–37: MMSI (30 bits)
/// - bits 38–39: spare
/// - bits 40+:   safety text (6-bit ASCII, up to 161 chars / 968 bits)
#[derive(Debug, Clone, PartialEq)]
pub struct SafetyBroadcast {
    pub repeat_indicator: u8,
    /// MMSI of the transmitting station.
    pub mmsi: u32,
    /// Safety-related text message, trimmed of trailing padding.
    pub text: String,
}

impl SafetyBroadcast {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 40 {
            return None;
        }
        let repeat_indicator = extract_u32(bits, 6, 2)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let char_count = bits.len().saturating_sub(40) / 6;
        let text = if char_count > 0 {
            extract_string(bits, 40, char_count)?
        } else {
            String::new()
        };
        Some(Self {
            repeat_indicator,
            mmsi,
            text,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a Vec<u8> of individual bits: 40 zero bits (header), then 6-bit text chars.
    fn build_bits(text_6bit: &[u8]) -> Vec<u8> {
        let mut b = vec![0u8; 40];
        for &c in text_6bit {
            for i in 0..6 {
                b.push((c >> (5 - i)) & 1);
            }
        }
        b
    }

    #[test]
    fn safety_text_preserves_leading_space() {
        // 6-bit chars: 32 = ' ', 1 = 'A'  ->  " A"
        let bits = build_bits(&[32, 1]);
        let msg = SafetyBroadcast::decode(&bits).expect("decode");
        assert_eq!(
            msg.text, " A",
            "leading space must be preserved (trailing-only trim)"
        );
    }
}
