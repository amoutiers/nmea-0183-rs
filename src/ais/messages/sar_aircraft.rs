//! AIS Type 9 — Standard SAR Aircraft Position Report.

use super::common::PositionTimestamp;
use super::utils::{decode_cog, decode_latitude, decode_longitude};
use crate::ais::armor::{extract_i32, extract_u32};

/// AIS Type 9 — Standard SAR Aircraft Position Report.
///
/// 168-bit fixed-length message from search and rescue aircraft.
/// Unlike vessel position reports, altitude is in meters and SOG is in whole knots (not 1/10).
#[derive(Debug, Clone, PartialEq)]
pub struct SarAircraftReport {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    /// Altitude in meters. None if not available (4095).
    pub altitude: Option<u16>,
    /// Speed over ground in knots (integer, NOT 1/10 knot). None if not available (1023).
    pub sog: Option<f32>,
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    /// Course over ground in degrees, with 0.1-degree resolution.
    /// None if unavailable or reserved.
    pub cog: Option<f32>,
    /// UTC second or explicit position timestamp status.
    pub timestamp: PositionTimestamp,
    /// Eight regional-application bits reserved for a regional use.
    pub regional_application: u8,
    /// DTE flag.
    pub dte: bool,
    /// Assigned mode flag.
    pub assigned: bool,
    /// RAIM flag.
    pub raim: bool,
    /// `true` selects the ITDMA communication-state format; otherwise SOTDMA.
    pub communication_state_selector: bool,
    /// Raw 19-bit communication state.
    pub communication_state: u32,
}

impl SarAircraftReport {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 168 {
            return None;
        }
        let repeat_indicator = extract_u32(bits, 6, 2)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let alt_raw = extract_u32(bits, 38, 12)?;
        let sog_raw = extract_u32(bits, 50, 10)?;
        let accuracy = extract_u32(bits, 60, 1)? == 1;
        let lon_raw = extract_i32(bits, 61, 28)?;
        let lat_raw = extract_i32(bits, 89, 27)?;
        let cog_raw = extract_u32(bits, 116, 12)?;
        let ts_raw = extract_u32(bits, 128, 6)? as u8;
        let regional_application = extract_u32(bits, 134, 8)? as u8;
        // ITU-R M.1371 Type 9: regional/reserved 134-141, DTE@142, spare 143-145,
        // assigned@146, RAIM@147.
        let dte = extract_u32(bits, 142, 1)? == 1; // Note: DTE=0 means "DTE ready"
        let assigned = extract_u32(bits, 146, 1)? == 1;
        let raim = extract_u32(bits, 147, 1)? == 1;
        let communication_state_selector = extract_u32(bits, 148, 1)? == 1;
        let communication_state = extract_u32(bits, 149, 19)?;
        Some(Self {
            repeat_indicator,
            mmsi,
            altitude: if alt_raw == 4095 {
                None
            } else {
                Some(alt_raw as u16)
            },
            sog: if sog_raw == 1023 {
                None
            } else {
                Some(sog_raw as f32)
            },
            position_accuracy: accuracy,
            longitude: decode_longitude(lon_raw),
            latitude: decode_latitude(lat_raw),
            cog: decode_cog(cog_raw),
            timestamp: PositionTimestamp::from_six_bits(ts_raw),
            regional_application,
            dte,
            assigned,
            raim,
            communication_state_selector,
            communication_state,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ais::messages::test_helpers::set_bits;

    #[test]
    fn type9_retains_radio_metadata() {
        let mut bits = vec![0u8; 168];
        set_bits(&mut bits, 0, 6, 9);
        set_bits(&mut bits, 6, 2, 2);
        set_bits(&mut bits, 134, 8, 0xA5);
        set_bits(&mut bits, 148, 1, 1);
        set_bits(&mut bits, 149, 19, 0x5_4321);

        let report = SarAircraftReport::decode(&bits).expect("decode");
        assert_eq!(report.repeat_indicator, 2);
        assert_eq!(report.regional_application, 0xA5);
        assert!(report.communication_state_selector);
        assert_eq!(report.communication_state, 0x5_4321);
    }
}
