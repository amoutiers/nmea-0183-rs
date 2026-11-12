//! AIS Type 20 — Data link management.

use crate::ais::armor::extract_u32;

/// A FATDMA reservation block from a Type 20 message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotReservation {
    pub offset: u16,
    pub slots: u8,
    pub timeout: u8,
    pub increment: u16,
}

/// AIS Type 20 data link management message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataLinkManagement {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    /// At most four complete reservations, in wire order.
    /// Additional reservation blocks and incomplete trailing blocks are ignored.
    pub reservations: Vec<SlotReservation>,
}

impl DataLinkManagement {
    pub(crate) fn decode(bits: &[u8]) -> Option<Self> {
        if bits.len() < 72 {
            return None;
        }
        let mut reservations = Vec::new();
        let mut offset = 40;
        while offset + 30 <= bits.len() && reservations.len() < 4 {
            reservations.push(SlotReservation {
                offset: extract_u32(bits, offset, 12)? as u16,
                slots: extract_u32(bits, offset + 12, 4)? as u8,
                timeout: extract_u32(bits, offset + 16, 3)? as u8,
                increment: extract_u32(bits, offset + 19, 11)? as u16,
            });
            offset += 30;
        }
        Some(Self {
            repeat_indicator: extract_u32(bits, 6, 2)? as u8,
            mmsi: extract_u32(bits, 8, 30)?,
            reservations,
        })
    }
}
