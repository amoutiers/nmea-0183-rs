//! AIS 6-bit ASCII armor encoding/decoding.
//!
//! AIS payloads are encoded as 6-bit ASCII: each character in the payload
//! represents 6 bits of data. Characters are in the range 0x30-0x77
//! (ASCII '0' to 'w'), with a gap at 0x58-0x5F (skipped).

/// Decode a 6-bit ASCII armored AIS payload into a bit vector.
///
/// Each character in the input maps to 6 bits. Returns a `Vec<u8>` where
/// each element is a single bit (0 or 1), MSB first within each 6-bit group.
///
/// The `fill_bits` parameter indicates how many of the rightmost bits in
/// the last 6-bit group should be ignored (0-5).
pub fn decode_armor(payload: &str, fill_bits: u8) -> Option<Vec<u8>> {
    if fill_bits > 5 {
        return None;
    }
    let mut bits = Vec::with_capacity(payload.len() * 6);

    for ch in payload.bytes() {
        // Valid 6-bit armor alphabet: 0x30-0x57 ('0'-'W') and 0x60-0x77 ('`'-'w').
        // The gap 0x58-0x5F and anything outside the range are invalid.
        if !(0x30..=0x57).contains(&ch) && !(0x60..=0x77).contains(&ch) {
            return None;
        }
        let mut val = ch - 48;
        if val > 40 {
            val -= 8; // skip the gap 0x58-0x5F
        }

        // Extract 6 bits, MSB first
        for i in (0..6).rev() {
            bits.push((val >> i) & 1);
        }
    }

    // Remove fill bits from the end
    let fill = fill_bits as usize;
    if fill > bits.len() {
        return None;
    }
    if fill > 0 {
        bits.truncate(bits.len() - fill);
    }

    Some(bits)
}

/// Encode AIS bits as 6-bit ASCII armor.
///
/// Returns the armored payload and the number of zero fill bits appended to its final
/// 6-bit character. Callers provide bits MSB first.
pub(crate) fn encode_armor(bits: &[u8]) -> (String, u8) {
    let fill_bits = ((6 - bits.len() % 6) % 6) as u8;
    let char_count = (bits.len() + usize::from(fill_bits)) / 6;
    let mut payload = String::with_capacity(char_count);

    for char_index in 0..char_count {
        let mut value = 0u8;
        for bit_index in 0..6 {
            let offset = char_index * 6 + bit_index;
            let bit = bits.get(offset).copied().unwrap_or(0) & 1;
            value = (value << 1) | bit;
        }
        let byte = if value < 40 { value + 48 } else { value + 56 };
        payload.push(char::from(byte));
    }

    (payload, fill_bits)
}

/// Extract an unsigned integer from a bit slice at the given offset and length.
///
/// Each consumed element must be a canonical bit (0 or 1), MSB first.
/// Values outside this input domain are unspecified; this precondition is not validated.
pub fn extract_u32(bits: &[u8], offset: usize, len: usize) -> Option<u32> {
    if len > 32 {
        return None;
    }
    let end = offset.checked_add(len)?;
    if end > bits.len() {
        return None;
    }
    let mut val: u32 = 0;
    for &bit in &bits[offset..end] {
        val = (val << 1) | u32::from(bit);
    }
    Some(val)
}

/// Extract a signed integer from a bit slice (two's complement).
///
/// Each consumed element must be a canonical bit (0 or 1), MSB first.
/// Values outside this input domain are unspecified; this precondition is not validated.
pub fn extract_i32(bits: &[u8], offset: usize, len: usize) -> Option<i32> {
    let raw = extract_u32(bits, offset, len)?;
    // Check sign bit
    if len > 0 && (raw >> (len - 1)) & 1 == 1 {
        // Negative: sign-extend
        // For len == 32 the value already fills all bits, so the mask is empty
        // (a 32-bit shift would overflow).
        let mask = if len >= 32 { 0 } else { u32::MAX << len };
        Some((raw | mask) as i32)
    } else {
        Some(raw as i32)
    }
}

/// Extract a 6-bit ASCII string from a bit slice.
///
/// Each consumed element must be a canonical bit (0 or 1), MSB first.
/// Values outside this input domain are unspecified; this precondition is not validated.
/// AIS text uses a custom 6-bit encoding: 0=@ (space), 1-26=A-Z, etc.
pub fn extract_string(bits: &[u8], offset: usize, num_chars: usize) -> Option<String> {
    let bit_len = num_chars.checked_mul(6)?;
    let end = offset.checked_add(bit_len)?;
    let chars = bits.get(offset..end)?;
    let mut s = String::with_capacity(num_chars);
    for chunk in chars.chunks_exact(6) {
        let val = extract_u32(chunk, 0, 6)? as u8;
        let ch = match val {
            0 => '@',
            1..=26 => (b'A' + val - 1) as char,
            27..=31 => (b'[' + val - 27) as char, // [\]^_
            32 => ' ',
            33..=57 => (b'!' + val - 33) as char, // !"#$%&'()*+,-./0123456789
            58..=63 => (b':' + val - 58) as char, // :;<=>?
            _ => '?',
        };
        s.push(ch);
    }
    // Trim trailing @ and spaces
    let trimmed = s.trim_end_matches(['@', ' ']);
    Some(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_bit_extraction_matches_integer_oracle() {
        for width in 1..=8 {
            for value in 0_u32..(1 << width) {
                let mut bits = vec![0, 1];
                bits.extend((0..width).rev().map(|shift| ((value >> shift) & 1) as u8));
                bits.push(1);
                assert_eq!(extract_u32(&bits, 2, width), Some(value));
                let signed = if value < (1 << (width - 1)) {
                    value as i32
                } else {
                    value as i32 - (1 << width)
                };
                assert_eq!(extract_i32(&bits, 2, width), Some(signed));
            }
        }
        for (width, value, signed) in [
            (31, 0, 0),
            (31, 1, 1),
            (31, 0x3fff_ffff, 1_073_741_823),
            (31, 0x4000_0000, -1_073_741_824),
            (31, 0x7fff_ffff, -1),
            (32, 0, 0),
            (32, 1, 1),
            (32, 0x7fff_ffff, 2_147_483_647),
            (32, 0x8000_0000, -2_147_483_648),
            (32, 0xffff_ffff, -1),
        ] {
            let bits: Vec<u8> = (0..width)
                .rev()
                .map(|shift| ((value >> shift) & 1) as u8)
                .collect();
            assert_eq!(extract_u32(&bits, 0, width), Some(value));
            assert_eq!(extract_i32(&bits, 0, width), Some(signed));
        }
        for bits in [&[][..], &[1][..]] {
            assert_eq!(extract_u32(bits, bits.len(), 0), Some(0));
            assert_eq!(extract_i32(bits, bits.len(), 0), Some(0));
            assert_eq!(extract_u32(bits, bits.len(), 1), None);
            assert_eq!(extract_i32(bits, bits.len(), 1), None);
        }
        assert_eq!(extract_u32(&[1, 0], 0, 2), Some(2));
        assert_eq!(extract_i32(&[1, 0], 0, 2), Some(-2));
        assert_eq!(extract_u32(&[0; 33], 0, 33), None);
        assert_eq!(extract_i32(&[0; 33], 0, 33), None);
        assert_eq!(extract_u32(&[1], usize::MAX, 1), None);
        assert_eq!(extract_i32(&[1], usize::MAX, 1), None);
    }

    #[test]
    fn decode_simple() {
        // '0' = ASCII 48, val = 0 → 000000
        // '1' = ASCII 49, val = 1 → 000001
        let bits = decode_armor("01", 0).expect("valid");
        assert_eq!(bits.len(), 12);
        assert_eq!(&bits[..6], &[0, 0, 0, 0, 0, 0]);
        assert_eq!(&bits[6..12], &[0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn encode_roundtrip_preserves_non_byte_aligned_bits() {
        let bits = vec![1, 0, 1, 0, 1, 0, 1];
        let (payload, fill_bits) = encode_armor(&bits);
        assert_eq!(decode_armor(&payload, fill_bits), Some(bits));
    }

    #[test]
    fn decode_with_fill_bits() {
        let bits = decode_armor("0", 2).expect("valid");
        assert_eq!(bits.len(), 4); // 6 - 2 = 4
    }

    #[test]
    fn decode_rejects_invalid_fill_bits() {
        assert_eq!(decode_armor("0", 6), None);
        assert_eq!(decode_armor("", 1), None);
    }

    #[test]
    fn extract_signed_negative() {
        // All ones = -1 in two's complement
        let bits = vec![1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(extract_i32(&bits, 0, 8), Some(-1));
    }

    #[test]
    fn extract_signed_positive() {
        // All zeros except last bit = 1 → value = 1
        let bits = vec![0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(extract_i32(&bits, 0, 8), Some(1));
    }

    #[test]
    fn extract_text() {
        // "@@@@" in AIS 6-bit = all zeros = "@" trimmed to empty
        let bits = vec![0; 24];
        assert_eq!(extract_string(&bits, 0, 4), Some(String::new()));
    }

    #[test]
    fn extract_unsigned() {
        let bits = decode_armor("15RTgt0PAso;90TKcjM8h6g208CQ", 0).expect("valid");
        // Message type is first 6 bits
        let msg_type = extract_u32(&bits, 0, 6).expect("valid");
        assert_eq!(msg_type, 1); // Type 1 position report

        // MMSI is bits 8-37 (30 bits)
        let mmsi = extract_u32(&bits, 8, 30).expect("valid");
        assert!(mmsi > 0);
        assert_eq!(mmsi, 371798000); // exact MMSI (was only asserted > 0)
    }

    #[test]
    fn extract_unsigned_out_of_bounds() {
        let bits = vec![0, 1, 0];
        assert_eq!(extract_u32(&bits, 0, 4), None); // only 3 bits available
    }

    #[test]
    fn extract_unsigned_large_offset_returns_none() {
        assert_eq!(extract_u32(&[0], usize::MAX, 1), None);
    }

    #[test]
    fn extract_text_large_offset_returns_none() {
        assert_eq!(extract_string(&[0; 6], usize::MAX, 1), None);
    }

    #[test]
    fn decode_empty_payload() {
        let bits = decode_armor("", 0).expect("valid");
        assert!(bits.is_empty());
    }

    #[test]
    fn decode_invalid_character() {
        // '~' (0x7E) is outside the valid AIS character range (0x30-0x77)
        assert!(decode_armor("~", 0).is_none());
    }

    #[test]
    fn decode_low_byte_rejected() {
        // Byte 0x20 (space) is below valid AIS range — must not panic
        assert!(decode_armor("\x20", 0).is_none());
    }

    #[test]
    fn decode_rejects_gap_characters() {
        // 0x58 'X' .. 0x5F '_' are the reserved gap and must be rejected.
        assert!(
            decode_armor("X", 0).is_none(),
            "'X' (0x58) must be rejected"
        );
        assert!(
            decode_armor("_", 0).is_none(),
            "'_' (0x5F) must be rejected"
        );
        // Sanity: legitimate characters on each side of the gap still decode.
        assert!(decode_armor("W", 0).is_some()); // 0x57, last before gap
        assert!(decode_armor("`", 0).is_some()); // 0x60, first after gap
    }

    #[test]
    fn decode_single_character() {
        let bits = decode_armor("0", 0).expect("valid");
        assert_eq!(bits.len(), 6);
        assert_eq!(&bits, &[0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn extract_i32_len_32_no_overflow() {
        // 32 one-bits = -1 in two's complement; must not panic on the mask shift.
        let bits = [1u8; 32];
        assert_eq!(extract_i32(&bits, 0, 32), Some(-1));
    }
}
