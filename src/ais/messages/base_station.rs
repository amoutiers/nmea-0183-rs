//! AIS Type 4 — Base Station Report.
//!
//! Transmitted by coast guard stations, port authorities, and AIS base stations.
//! Provides UTC time synchronization and precise position for fixed reference stations.
//! Uses the same 168-bit layout as Type 1 but with UTC fields instead of voyage data.

use crate::ais::armor::{extract_i32, extract_u32};

use super::utils::{decode_latitude, decode_longitude};

/// AIS Type 4 — Base Station Report.
///
/// ITU-R M.1371 bit layout (168 bits):
/// - bits   0–5:   message type (= 4)
/// - bits   6–7:   repeat indicator
/// - bits   8–37:  MMSI (30 bits)
/// - bits  38–51:  UTC year (14 bits, 0 = not available)
/// - bits  52–55:  UTC month (4 bits, 0 = not available)
/// - bits  56–60:  UTC day (5 bits, 0 = not available)
/// - bits  61–65:  UTC hour (5 bits, 24 = not available)
/// - bits  66–71:  UTC minute (6 bits, 60 = not available)
/// - bits  72–77:  UTC second (6 bits, 60 = not available)
/// - bits  78:     position accuracy (1 bit)
/// - bits  79–106: longitude in 1/10000 min (28 bits, signed)
/// - bits 107–133: latitude in 1/10000 min (27 bits, signed)
/// - bits 134–137: type of EPFD (4 bits)
#[derive(Debug, Clone, PartialEq)]
pub struct BaseStationReport {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    /// UTC year (e.g. 2024). `None` if not available (raw = 0).
    pub year: Option<u16>,
    /// UTC month (1–12). `None` if not available or reserved (raw = 0 or 13–15).
    pub month: Option<u8>,
    /// UTC day (1–31). `None` if not available (raw = 0).
    pub day: Option<u8>,
    /// UTC hour (0–23). `None` if not available or reserved (raw = 24–31).
    pub hour: Option<u8>,
    /// UTC minute (0–59). `None` if not available or reserved (raw = 60–63).
    pub minute: Option<u8>,
    /// UTC second (0–59). `None` if not available or reserved (raw = 60–63).
    pub second: Option<u8>,
    /// High position accuracy (DGPS / differential fix).
    pub position_accuracy: bool,
    /// Longitude in decimal degrees WGS-84. `None` if sentinel (181°).
    pub longitude: Option<f64>,
    /// Latitude in decimal degrees WGS-84. `None` if sentinel (91°).
    pub latitude: Option<f64>,
    /// Type of EPFD (electronic position fixing device). 0 = undefined.
    pub type_of_epfd: u8,
    /// UTC transmission-control flag.
    pub transmission_control: bool,
    pub raim: bool,
    /// Raw 19-bit SOTDMA radio status.
    pub communication_state: u32,
}

impl BaseStationReport {
    /// Decode a Type 4 Base Station Report from AIS bits.
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
    use crate::ais::{AisDecodeOutcome, AisMessage, AisParser};
    use crate::parse_frame;

    #[test]
    fn base_station_gpsd() {
        let mut parser = AisParser::new();
        // Type 4 from gpsd ais.nmea fixture
        let frame = parse_frame("!AIVDM,1,1,,A,403OviQuMGCqWrRO9>E6fE700@GO,0*4D").expect("valid");
        let msg = parser.decode(&frame).expect("decoded");
        if let AisDecodeOutcome::Message(AisMessage::BaseStation(report)) = msg {
            assert!(report.mmsi > 0, "MMSI must be set");
            if let (Some(lat), Some(lon)) = (report.latitude, report.longitude) {
                assert!((-90.0..=90.0).contains(&lat), "lat out of range: {lat}");
                assert!((-180.0..=180.0).contains(&lon), "lon out of range: {lon}");
            }
        } else {
            panic!("expected BaseStation, got {msg:?}");
        }
    }

    #[test]
    fn base_station_utc_sentinel() {
        let mut parser = AisParser::new();
        let frame = parse_frame("!AIVDM,1,1,,A,403OviQuMGCqWrRO9>E6fE700@GO,0*4D").expect("valid");
        let msg = parser.decode(&frame).expect("decoded");
        if let AisDecodeOutcome::Message(AisMessage::BaseStation(report)) = msg {
            if let Some(h) = report.hour {
                assert!(h < 24, "hour sentinel not filtered: {h}");
            }
            if let Some(m) = report.minute {
                assert!(m < 60, "minute sentinel not filtered: {m}");
            }
        } else {
            panic!("expected BaseStation, got {msg:?}");
        }
    }

    #[test]
    fn reserved_time_values_are_none() {
        fn decode_type4(offset: usize, len: usize, value: u32) -> BaseStationReport {
            let mut bits = vec![0u8; 168];
            set_bits(&mut bits, 0, 6, 4);
            set_bits(&mut bits, offset, len, value);
            BaseStationReport::decode(&bits).expect("decode")
        }

        assert_eq!(decode_type4(52, 4, 13).month, None);
        assert_eq!(decode_type4(61, 5, 25).hour, None);
        assert_eq!(decode_type4(66, 6, 61).minute, None);
        assert_eq!(decode_type4(72, 6, 62).second, None);
        assert_eq!(decode_type4(52, 4, 12).month, Some(12));
    }
}
