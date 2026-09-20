//! AIS position report — Type 18 (Class B standard).

use super::common::PositionTimestamp;
use crate::ais::armor::{extract_i32, extract_u32};

use super::common::AisClass;
use super::position_a::{ClassBPositionMetadata, PositionReport};
use super::utils::{decode_cog, decode_heading, decode_latitude, decode_longitude, decode_sog};

impl PositionReport {
    /// Decode a Type 18 Class B standard position report.
    pub fn decode_class_b(bits: &[u8]) -> Option<Self> {
        if bits.len() < 168 {
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
        let class_b = ClassBPositionMetadata {
            transmit_power_low: extract_u32(bits, 139, 1)? == 1,
            class_b_cs: extract_u32(bits, 141, 1)? == 1,
            display_available: extract_u32(bits, 142, 1)? == 1,
            dsc_capable: extract_u32(bits, 143, 1)? == 1,
            full_band_capable: extract_u32(bits, 144, 1)? == 1,
            message_22_capable: extract_u32(bits, 145, 1)? == 1,
            assigned_mode: extract_u32(bits, 146, 1)? == 1,
            communication_state_selector: extract_u32(bits, 148, 1)? == 1,
        };
        let raim = extract_u32(bits, 147, 1)? == 1;
        let communication_state = extract_u32(bits, 149, 19)?;

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
            communication_state: Some(communication_state),
            class_b: Some(class_b),
            class_b_extended: None,
            ais_class: AisClass::B,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    #[test]
    fn type18_retains_capability_and_radio_metadata() {
        let mut bits = vec![0u8; 168];
        set_bits(&mut bits, 0, 6, 18);
        set_bits(&mut bits, 6, 2, 2);
        set_bits(&mut bits, 139, 1, 1);
        set_bits(&mut bits, 141, 1, 1);
        set_bits(&mut bits, 142, 1, 1);
        set_bits(&mut bits, 144, 1, 1);
        set_bits(&mut bits, 146, 1, 1);
        set_bits(&mut bits, 147, 1, 1);
        set_bits(&mut bits, 148, 1, 1);
        set_bits(&mut bits, 149, 19, 0x5_4321);

        let pos = PositionReport::decode_class_b(&bits).expect("decodes");
        assert_eq!(pos.repeat_indicator, 2);
        assert!(pos.raim);
        assert_eq!(pos.communication_state, Some(0x5_4321));
        assert_eq!(
            pos.class_b,
            Some(ClassBPositionMetadata {
                transmit_power_low: true,
                class_b_cs: true,
                display_available: true,
                dsc_capable: false,
                full_band_capable: true,
                message_22_capable: false,
                assigned_mode: true,
                communication_state_selector: true,
            })
        );
    }
}
