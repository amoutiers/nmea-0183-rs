//! AIS position report — Type 19 (Class B+ extended).

use super::common::PositionTimestamp;
use crate::ais::armor::{extract_i32, extract_string, extract_u32};

use super::common::AisClass;
use super::position_a::{ClassBExtendedData, PositionReport};
use super::utils::{decode_cog, decode_heading, decode_latitude, decode_longitude, decode_sog};

impl PositionReport {
    /// Decode a Type 19 Class B+ extended position report.
    pub fn decode_class_b_extended(bits: &[u8]) -> Option<Self> {
        if bits.len() < 312 {
            return None;
        }

        let msg_type = extract_u32(bits, 0, 6)? as u8;
        let repeat_indicator = extract_u32(bits, 6, 2)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let sog_raw = extract_u32(bits, 46, 10)?;
        let accuracy = extract_u32(bits, 56, 1)? == 1;
        let lon_raw = extract_i32(bits, 57, 28)?;
        let lat_raw = extract_i32(bits, 85, 27)?;
        let cog_raw = extract_u32(bits, 112, 12)?;
        let hdg_raw = extract_u32(bits, 124, 9)?;
        let ts_raw = extract_u32(bits, 133, 6)? as u8;
        let raim = extract_u32(bits, 305, 1)? == 1;
        let class_b_extended = ClassBExtendedData {
            vessel_name: extract_string(bits, 143, 20)?,
            ship_type: extract_u32(bits, 263, 8)? as u8,
            dimension_to_bow: extract_u32(bits, 271, 9)? as u16,
            dimension_to_stern: extract_u32(bits, 280, 9)? as u16,
            dimension_to_port: extract_u32(bits, 289, 6)? as u8,
            dimension_to_starboard: extract_u32(bits, 295, 6)? as u8,
            position_fixing_device: extract_u32(bits, 301, 4)? as u8,
            dte: extract_u32(bits, 306, 1)? == 1,
            assigned_mode: extract_u32(bits, 307, 1)? == 1,
        };

        Some(Self {
            msg_type,
            repeat_indicator,
            mmsi,
            nav_status: None,
            rate_of_turn: None,
            sog: decode_sog(sog_raw),
            position_accuracy: accuracy,
            longitude: decode_longitude(lon_raw),
            latitude: decode_latitude(lat_raw),
            cog: decode_cog(cog_raw),
            heading: decode_heading(hdg_raw),
            timestamp: PositionTimestamp::from_six_bits(ts_raw),
            maneuver_indicator: None,
            raim,
            communication_state: None,
            class_b: None,
            class_b_extended: Some(class_b_extended),
            ais_class: AisClass::BPlus,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    #[test]
    fn type19_retains_static_extensions() {
        let mut bits = vec![0u8; 312];
        set_bits(&mut bits, 0, 6, 19);
        set_bits(&mut bits, 6, 2, 1);
        set_bits(&mut bits, 143, 6, 20); // T
        set_bits(&mut bits, 149, 6, 5); // E
        set_bits(&mut bits, 155, 6, 19); // S
        set_bits(&mut bits, 161, 6, 20); // T
        set_bits(&mut bits, 263, 8, 70);
        set_bits(&mut bits, 271, 9, 10);
        set_bits(&mut bits, 280, 9, 5);
        set_bits(&mut bits, 289, 6, 2);
        set_bits(&mut bits, 295, 6, 3);
        set_bits(&mut bits, 301, 4, 1);
        set_bits(&mut bits, 305, 1, 1);
        set_bits(&mut bits, 306, 1, 1);
        set_bits(&mut bits, 307, 1, 1);

        let pos = PositionReport::decode_class_b_extended(&bits).expect("decodes");
        assert_eq!(pos.repeat_indicator, 1);
        assert!(pos.raim);
        assert_eq!(
            pos.class_b_extended,
            Some(ClassBExtendedData {
                vessel_name: "TEST".to_string(),
                ship_type: 70,
                dimension_to_bow: 10,
                dimension_to_stern: 5,
                dimension_to_port: 2,
                dimension_to_starboard: 3,
                position_fixing_device: 1,
                dte: true,
                assigned_mode: true,
            })
        );
    }
}
