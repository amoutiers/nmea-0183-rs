//! AIS Type 27 — Long Range Position Report.
//!
//! Compact 96-bit position report from satellite AIS transponders (Class D) and
//! vessels with long-range AIS. Position precision is 1/10 minute (vs 1/10000' for Type 1),
//! suited for the reduced bandwidth of satellite uplinks.

use crate::ais::armor::{extract_i32, extract_u32};

use super::common::NavigationStatus;

/// AIS Type 27 — Long Range Position Report.
///
/// ITU-R M.1371 bit layout (96 bits):
/// - bits  0–5:   message type (= 27)
/// - bits  6–7:   repeat indicator
/// - bits  8–37:  MMSI (30 bits)
/// - bit   38:    position accuracy
/// - bit   39:    RAIM flag
/// - bits 40–43:  navigational status (4 bits)
/// - bits 44–61:  longitude in 1/10 minute (18 bits, signed)
/// - bits 62–78:  latitude in 1/10 minute (17 bits, signed)
/// - bits 79–84:  SOG in knots (6 bits, integer, 63 = not available)
/// - bits 85–93:  COG in degrees (9 bits, integer, 511 = not available)
/// - bit   94:    GNSS position status
#[derive(Debug, Clone, PartialEq)]
pub struct LongRangePosition {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    /// High position accuracy (DGPS / differential fix).
    pub position_accuracy: bool,
    /// RAIM (Receiver Autonomous Integrity Monitoring) flag.
    pub raim: bool,
    /// Navigational status. `None` if not defined.
    pub nav_status: Option<NavigationStatus>,
    /// Longitude in decimal degrees WGS-84 (1/10 minute precision). `None` if sentinel (181°).
    pub longitude: Option<f64>,
    /// Latitude in decimal degrees WGS-84 (1/10 minute precision). `None` if sentinel (91°).
    pub latitude: Option<f64>,
    /// Speed over ground in integer knots. `None` if not available (raw = 63).
    pub sog: Option<u8>,
    /// Course over ground in integer degrees (0–359). `None` if not available (raw = 511).
    pub cog: Option<u16>,
    /// `false` = GNSS position less than five seconds old; `true` = more than five seconds old.
    pub gnss_position_status: bool,
}

impl LongRangePosition {
    /// Decode a Type 27 Long Range Position Report from AIS bits.
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 96 {
            return None;
        }

        let repeat_indicator = extract_u32(bits, 6, 2)? as u8;
        let mmsi = extract_u32(bits, 8, 30)?;
        let accuracy = extract_u32(bits, 38, 1)? == 1;
        let raim = extract_u32(bits, 39, 1)? == 1;
        let nav_raw = extract_u32(bits, 40, 4)? as u8;
        let lon_raw = extract_i32(bits, 44, 18)?;
        let lat_raw = extract_i32(bits, 62, 17)?;
        let sog_raw = extract_u32(bits, 79, 6)? as u8;
        let cog_raw = extract_u32(bits, 85, 9)? as u16;
        let gnss = extract_u32(bits, 94, 1)? == 1;

        // Type 27 stores position in 1/10 minute (1/600 degree).
        let longitude = {
            let deg = f64::from(lon_raw) / 600.0;
            if !(-180.0..=180.0).contains(&deg) {
                None
            } else {
                Some(deg)
            }
        };
        let latitude = {
            let deg = f64::from(lat_raw) / 600.0;
            if !(-90.0..=90.0).contains(&deg) {
                None
            } else {
                Some(deg)
            }
        };

        Some(Self {
            repeat_indicator,
            mmsi,
            position_accuracy: accuracy,
            raim,
            nav_status: Some(NavigationStatus::from(nav_raw)),
            longitude,
            latitude,
            sog: if sog_raw == 63 { None } else { Some(sog_raw) },
            cog: (cog_raw < 360).then_some(cog_raw),
            gnss_position_status: gnss,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::LongRangePosition;
    use crate::ais::messages::test_helpers::set_bits;
    use crate::ais::{AisDecodeOutcome, AisMessage, AisParser};
    use crate::parse_frame;

    #[test]
    fn long_range_gpsd() {
        let mut parser = AisParser::new();
        // Type 27 from gpsd ais.nmea fixture
        let frame = parse_frame("!AIVDM,1,1,,A,KCQ9r=hrFUnH7P00,0*41").expect("valid");
        let msg = parser.decode(&frame).expect("decoded");
        if let AisDecodeOutcome::Message(AisMessage::LongRangePosition(pos)) = msg {
            assert!(pos.mmsi > 0, "MMSI must be set");
            if let (Some(lat), Some(lon)) = (pos.latitude, pos.longitude) {
                assert!((-90.0..=90.0).contains(&lat), "lat out of range: {lat}");
                assert!((-180.0..=180.0).contains(&lon), "lon out of range: {lon}");
            }
        } else {
            panic!("expected LongRangePosition, got {msg:?}");
        }
    }

    #[test]
    fn long_range_position_sentinel() {
        let mut parser = AisParser::new();
        let frame = parse_frame("!AIVDM,1,1,,A,KCQ9r=hrFUnH7P00,0*41").expect("valid");
        let msg = parser.decode(&frame).expect("decoded");
        if let AisDecodeOutcome::Message(AisMessage::LongRangePosition(pos)) = msg {
            if let Some(lat) = pos.latitude {
                assert!(
                    (-90.0..=90.0).contains(&lat),
                    "lat sentinel not filtered: {lat}"
                );
            }
            if let Some(lon) = pos.longitude {
                assert!(
                    (-180.0..=180.0).contains(&lon),
                    "lon sentinel not filtered: {lon}"
                );
            }
        } else {
            panic!("expected LongRangePosition, got {msg:?}");
        }
    }

    #[test]
    fn long_range_course_filters_reserved_values() {
        for (raw, expected) in [
            (0, Some(0)),
            (359, Some(359)),
            (360, None),
            (510, None),
            (511, None),
        ] {
            let mut bits = vec![0; 96];
            set_bits(&mut bits, 0, 6, 27);
            set_bits(&mut bits, 85, 9, raw);
            assert_eq!(
                LongRangePosition::decode(&bits).expect("decode").cog,
                expected
            );
        }
    }
}
