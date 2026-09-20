//! AIS aid-to-navigation report — Type 21.
//!
//! Transmitted by AIS-equipped buoys, lighthouses, and beacons. Provides real-time
//! position and identity for navigational aids. Always single-frame.

use super::common::PositionTimestamp;
use crate::ais::armor::{extract_i32, extract_string, extract_u32};

use super::utils::{decode_latitude, decode_longitude};

/// AIS Type 21 — Aid-to-Navigation Report.
///
/// ITU-R M.1371 bit layout (minimum 272 bits):
/// - bits   0–5:   message type (= 21)
/// - bits   6–7:   repeat indicator
/// - bits   8–37:  MMSI (30 bits)
/// - bits  38–42:  type of AID (5 bits, 0–31)
/// - bits  43–162: name (20 × 6-bit ASCII chars)
/// - bit   163:    position accuracy
/// - bits 164–191: longitude (28 bits, 1/10000 min, same encoding as Type 1)
/// - bits 192–218: latitude  (27 bits, 1/10000 min, same encoding as Type 1)
#[derive(Debug, Clone, PartialEq)]
pub struct AidToNavigation {
    pub repeat_indicator: u8,
    /// MMSI of the aid-to-navigation transponder.
    pub mmsi: u32,
    /// Type of navigational aid (ITU-R M.1371-6 Table 72).
    /// E.g. 0=unspecified, 1=reference point, 20=north cardinal mark,
    /// 30=special mark, 31=light vessel/LANBY/rig.
    pub aid_type: u8,
    /// Name of the aid (e.g. "PORTLAND BILL LT"), trimmed of padding.
    pub name: String,
    /// Latitude in decimal degrees. `None` if not available (sentinel 91°).
    pub lat: Option<f64>,
    /// Longitude in decimal degrees. `None` if not available (sentinel 181°).
    pub lon: Option<f64>,
    pub position_accuracy: bool,
    pub dimension_to_bow: u16,
    pub dimension_to_stern: u16,
    pub dimension_to_port: u8,
    pub dimension_to_starboard: u8,
    pub position_fixing_device: u8,
    pub timestamp: PositionTimestamp,
    pub off_position: bool,
    pub regional_application: u8,
    pub raim: bool,
    pub virtual_aid: bool,
    pub assigned_mode: bool,
    pub name_extension: Option<String>,
}

impl AidToNavigation {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 272 {
            return None;
        }
        let repeat_indicator = extract_u32(bits, 6, 2)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let aid_type = extract_u32(bits, 38, 5)? as u8;
        let name = extract_string(bits, 43, 20)?;
        let lon_raw = extract_i32(bits, 164, 28)?;
        let lat_raw = extract_i32(bits, 192, 27)?;
        let timestamp_raw = extract_u32(bits, 253, 6)? as u8;
        let name_extension = if bits.len() > 272 {
            let char_count = (bits.len() - 272) / 6;
            if char_count == 0 {
                None
            } else {
                Some(extract_string(bits, 272, char_count)?)
            }
        } else {
            None
        };
        Some(Self {
            repeat_indicator,
            mmsi,
            aid_type,
            name,
            lat: decode_latitude(lat_raw),
            lon: decode_longitude(lon_raw),
            position_accuracy: extract_u32(bits, 163, 1)? == 1,
            dimension_to_bow: extract_u32(bits, 219, 9)? as u16,
            dimension_to_stern: extract_u32(bits, 228, 9)? as u16,
            dimension_to_port: extract_u32(bits, 237, 6)? as u8,
            dimension_to_starboard: extract_u32(bits, 243, 6)? as u8,
            position_fixing_device: extract_u32(bits, 249, 4)? as u8,
            timestamp: PositionTimestamp::from_six_bits(timestamp_raw),
            off_position: extract_u32(bits, 259, 1)? == 1,
            regional_application: extract_u32(bits, 260, 8)? as u8,
            raim: extract_u32(bits, 268, 1)? == 1,
            virtual_aid: extract_u32(bits, 269, 1)? == 1,
            assigned_mode: extract_u32(bits, 270, 1)? == 1,
            name_extension,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    #[test]
    fn aton_name_preserves_leading_space() {
        // Type 21 needs >= 272 bits. name @43, 20 chars: char0=32 (' '), char1=1 ('A').
        let mut bits = vec![0u8; 272];
        set_bits(&mut bits, 0, 6, 21); // msg_type 21
        set_bits(&mut bits, 43, 6, 32); // ' '
        set_bits(&mut bits, 49, 6, 1); // 'A'
        let aton = AidToNavigation::decode(&bits).expect("decode");
        assert_eq!(
            aton.name, " A",
            "leading space must be preserved (trailing-only trim)"
        );
    }
}
