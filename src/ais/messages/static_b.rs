//! AIS static data report — Type 24 (Class B).

use crate::ais::armor::{extract_string, extract_u32};

/// AIS Static Data Report — Type 24 (Class B).
///
/// Type 24 comes in two parts:
/// - Part A: vessel name
/// - Part B: callsign + ship type
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum StaticDataReport {
    PartA {
        repeat_indicator: u8,
        mmsi: u32,
        vessel_name: String,
    },
    PartB {
        repeat_indicator: u8,
        mmsi: u32,
        manufacturer_id: String,
        model_code: u8,
        serial_number: u32,
        callsign: String,
        ship_type: u8,
        dimension_to_bow: u16,
        dimension_to_stern: u16,
        dimension_to_port: u8,
        dimension_to_starboard: u8,
        position_fixing_device: u8,
        vdes_capabilities: u8,
    },
}

impl StaticDataReport {
    /// Decode a Type 24 static data report.
    ///
    /// Part B requires at least 168 bits and ignores trailing bits; part A requires at least 160 bits.
    /// Consumed elements must be canonical bits (0 or 1), MSB first.
    pub fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 160 {
            return None;
        }

        let repeat_indicator = extract_u32(bits, 6, 2)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let part_number = extract_u32(bits, 38, 2)?;

        match part_number {
            0 => {
                // Part A: vessel name
                let vessel_name = extract_string(bits, 40, 20)?;
                Some(Self::PartA {
                    repeat_indicator,
                    mmsi,
                    vessel_name,
                })
            }
            1 => {
                // Part B: callsign + ship type
                if bits.len() < 168 {
                    return None;
                }
                let ship_type = extract_u32(bits, 40, 8)? as u8;
                let manufacturer_id = extract_string(bits, 48, 3)?;
                let model_code = extract_u32(bits, 66, 4)? as u8;
                let serial_number = extract_u32(bits, 70, 20)?;
                let callsign = extract_string(bits, 90, 7)?;
                let dimension_to_bow = extract_u32(bits, 132, 9)? as u16;
                let dimension_to_stern = extract_u32(bits, 141, 9)? as u16;
                let dimension_to_port = extract_u32(bits, 150, 6)? as u8;
                let dimension_to_starboard = extract_u32(bits, 156, 6)? as u8;
                let position_fixing_device = extract_u32(bits, 162, 4)? as u8;
                let vdes_capabilities = extract_u32(bits, 166, 2)? as u8;
                Some(Self::PartB {
                    repeat_indicator,
                    mmsi,
                    manufacturer_id,
                    model_code,
                    serial_number,
                    callsign,
                    ship_type,
                    dimension_to_bow,
                    dimension_to_stern,
                    dimension_to_port,
                    dimension_to_starboard,
                    position_fixing_device,
                    vdes_capabilities,
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    #[test]
    fn type24_retains_static_part_metadata() {
        let mut bits = vec![0u8; 168];
        set_bits(&mut bits, 0, 6, 24);
        set_bits(&mut bits, 6, 2, 3);
        set_bits(&mut bits, 38, 2, 1);
        set_bits(&mut bits, 40, 8, 70);
        set_bits(&mut bits, 48, 6, 19);
        set_bits(&mut bits, 54, 6, 9);
        set_bits(&mut bits, 60, 6, 13);
        set_bits(&mut bits, 66, 4, 7);
        set_bits(&mut bits, 70, 20, 42);
        set_bits(&mut bits, 132, 9, 8);
        set_bits(&mut bits, 141, 9, 4);
        set_bits(&mut bits, 150, 6, 2);
        set_bits(&mut bits, 156, 6, 3);
        set_bits(&mut bits, 162, 4, 1);
        set_bits(&mut bits, 166, 2, 2);

        assert_eq!(
            StaticDataReport::decode(&bits),
            Some(StaticDataReport::PartB {
                repeat_indicator: 3,
                mmsi: 0,
                manufacturer_id: "SIM".to_string(),
                model_code: 7,
                serial_number: 42,
                callsign: String::new(),
                ship_type: 70,
                dimension_to_bow: 8,
                dimension_to_stern: 4,
                dimension_to_port: 2,
                dimension_to_starboard: 3,
                position_fixing_device: 1,
                vdes_capabilities: 2,
            })
        );
    }
}
