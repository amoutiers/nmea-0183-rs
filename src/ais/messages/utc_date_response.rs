//! AIS Type 11 — UTC/Date Response.

use super::utils::{decode_latitude, decode_longitude};
use crate::ais::armor::{extract_i32, extract_u32};

/// AIS Type 11 — UTC/Date Response.
///
/// Decoding through [`crate::ais::AisParser`] requires at least 168 bits;
/// trailing bits are ignored.
///
/// Same bit layout as Type 4 (168 bits). Sent by mobile stations in response to Type 10 interrogation.
#[derive(Debug, Clone, PartialEq)]
pub struct UtcDateResponse {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub year: Option<u16>,  // 0 = not available
    pub month: Option<u8>,  // 0 = not available; 13-15 reserved
    pub day: Option<u8>,    // 0 = not available
    pub hour: Option<u8>,   // 24 = not available; 25-31 reserved
    pub minute: Option<u8>, // 60 = not available; 61-63 reserved
    pub second: Option<u8>, // 60 = not available; 61-63 reserved
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub type_of_epfd: u8,
    pub transmission_control: bool,
    pub raim: bool,
    pub communication_state: u32,
}

impl UtcDateResponse {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 168 {
            return None;
        }
        let repeat_indicator = extract_u32(bits, 6, 2)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let year_raw = extract_u32(bits, 38, 14)?;
        let month_raw = extract_u32(bits, 52, 4)? as u8;
        let day_raw = extract_u32(bits, 56, 5)? as u8;
        let hour_raw = extract_u32(bits, 61, 5)? as u8;
        let minute_raw = extract_u32(bits, 66, 6)? as u8;
        let second_raw = extract_u32(bits, 72, 6)? as u8;
        let accuracy = extract_u32(bits, 78, 1)? == 1;
        let lon_raw = extract_i32(bits, 79, 28)?;
        let lat_raw = extract_i32(bits, 107, 27)?;
        let epfd = extract_u32(bits, 134, 4)? as u8;
        let transmission_control = extract_u32(bits, 138, 1)? == 1;
        let raim = extract_u32(bits, 148, 1)? == 1;
        let communication_state = extract_u32(bits, 149, 19)?;
        Some(Self {
            repeat_indicator,
            mmsi,
            year: if year_raw == 0 {
                None
            } else {
                Some(year_raw as u16)
            },
            month: if month_raw == 0 || month_raw > 12 {
                None
            } else {
                Some(month_raw)
            },
            day: if day_raw == 0 { None } else { Some(day_raw) },
            hour: if hour_raw >= 24 { None } else { Some(hour_raw) },
            minute: if minute_raw >= 60 {
                None
            } else {
                Some(minute_raw)
            },
            second: if second_raw >= 60 {
                None
            } else {
                Some(second_raw)
            },
            position_accuracy: accuracy,
            longitude: decode_longitude(lon_raw),
            latitude: decode_latitude(lat_raw),
            type_of_epfd: epfd,
            transmission_control,
            raim,
            communication_state,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    fn decode_type11(offset: usize, len: usize, value: u32) -> UtcDateResponse {
        let mut bits = vec![0u8; 168];
        set_bits(&mut bits, 0, 6, 11);
        set_bits(&mut bits, offset, len, value);
        UtcDateResponse::decode(&bits).expect("decode")
    }

    #[test]
    fn reserved_time_values_are_none() {
        assert_eq!(decode_type11(52, 4, 13).month, None);
        assert_eq!(decode_type11(61, 5, 24).hour, None);
        assert_eq!(decode_type11(61, 5, 25).hour, None);
        assert_eq!(decode_type11(66, 6, 61).minute, None);
        assert_eq!(decode_type11(72, 6, 62).second, None);
        assert_eq!(decode_type11(52, 4, 12).month, Some(12));
        assert_eq!(decode_type11(61, 5, 23).hour, Some(23));
        assert_eq!(decode_type11(66, 6, 59).minute, Some(59));
    }
}
