use crate::EncodeError;
use crate::ais::encode::{
    BitWriter, encode_cog, encode_epfd, encode_heading, encode_latitude, encode_longitude,
    encode_position_timestamp, encode_sog,
};

use super::{AisEncodable, AisTransmitOptions, PositionTimestamp, encode_payload};

/// AIS Type 18 communication-state selector and its 19-bit state value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassBCommunicationState {
    Sotdma(u32),
    Itdma(u32),
}

impl ClassBCommunicationState {
    fn selector(self) -> bool {
        matches!(self, Self::Itdma(_))
    }

    fn value(self) -> u32 {
        match self {
            Self::Sotdma(value) | Self::Itdma(value) => value,
        }
    }
}

/// AIS Type 18 standard Class B position report.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassBPosition {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub sog: Option<f32>,
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub cog: Option<f32>,
    pub heading: Option<u16>,
    pub timestamp: PositionTimestamp,
    pub transmit_power_low: bool,
    pub class_b_cs: bool,
    pub display_available: bool,
    pub dsc_capable: bool,
    pub full_band_capable: bool,
    pub message_22_capable: bool,
    pub assigned_mode: bool,
    pub raim: bool,
    pub communication_state: ClassBCommunicationState,
}

/// AIS Type 19 Class B+ extended position report.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassBExtendedPosition {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub sog: Option<f32>,
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub cog: Option<f32>,
    pub heading: Option<u16>,
    pub timestamp: PositionTimestamp,
    pub vessel_name: String,
    pub ship_type: u8,
    pub dimension_to_bow: u16,
    pub dimension_to_stern: u16,
    pub dimension_to_port: u8,
    pub dimension_to_starboard: u8,
    pub position_fixing_device: u8,
    /// `false` means the DTE is available.
    pub dte: bool,
    pub assigned_mode: bool,
    pub raim: bool,
}

/// AIS Type 24 Part A static data report.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassBStaticPartA {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub vessel_name: String,
}

/// AIS Type 24 Part B static data report.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassBStaticPartB {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub ship_type: u8,
    /// Three-character AIS manufacturer mnemonic.
    pub manufacturer_id: String,
    pub model_code: u8,
    pub serial_number: u32,
    pub callsign: String,
    pub dimension_to_bow: u16,
    pub dimension_to_stern: u16,
    pub dimension_to_port: u8,
    pub dimension_to_starboard: u8,
    pub position_fixing_device: u8,
    /// VDES capability code from ITU-R M.1371-6 Table 77.
    pub vdes_capabilities: u8,
}

impl AisEncodable for ClassBPosition {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        let mut writer = BitWriter::new();
        writer.push_u32(18, 6, "message_type")?;
        writer.push_u32(u32::from(self.repeat_indicator), 2, "repeat_indicator")?;
        writer.push_u32(self.mmsi, 30, "mmsi")?;
        writer.push_spare(8);
        writer.push_u32(encode_sog(self.sog)?, 10, "sog")?;
        writer.push_bool(self.position_accuracy);
        writer.push_i32(encode_longitude(self.longitude)?, 28, "longitude")?;
        writer.push_i32(encode_latitude(self.latitude)?, 27, "latitude")?;
        writer.push_u32(encode_cog(self.cog)?, 12, "cog")?;
        writer.push_u32(encode_heading(self.heading)?, 9, "heading")?;
        writer.push_u32(encode_position_timestamp(self.timestamp)?, 6, "timestamp")?;
        writer.push_bool(self.transmit_power_low);
        writer.push_spare(1);
        writer.push_bool(self.class_b_cs);
        writer.push_bool(self.display_available);
        writer.push_bool(self.dsc_capable);
        writer.push_bool(self.full_band_capable);
        writer.push_bool(self.message_22_capable);
        writer.push_bool(self.assigned_mode);
        writer.push_bool(self.raim);
        writer.push_bool(self.communication_state.selector());
        writer.push_u32(self.communication_state.value(), 19, "communication_state")?;
        encode_payload(&writer.finish(), options)
    }
}

impl AisEncodable for ClassBExtendedPosition {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        let mut writer = BitWriter::new();
        writer.push_u32(19, 6, "message_type")?;
        writer.push_u32(u32::from(self.repeat_indicator), 2, "repeat_indicator")?;
        writer.push_u32(self.mmsi, 30, "mmsi")?;
        writer.push_spare(8);
        writer.push_u32(encode_sog(self.sog)?, 10, "sog")?;
        writer.push_bool(self.position_accuracy);
        writer.push_i32(encode_longitude(self.longitude)?, 28, "longitude")?;
        writer.push_i32(encode_latitude(self.latitude)?, 27, "latitude")?;
        writer.push_u32(encode_cog(self.cog)?, 12, "cog")?;
        writer.push_u32(encode_heading(self.heading)?, 9, "heading")?;
        writer.push_u32(encode_position_timestamp(self.timestamp)?, 6, "timestamp")?;
        writer.push_spare(4);
        writer.push_text(&self.vessel_name, 20, "vessel_name")?;
        writer.push_u32(u32::from(self.ship_type), 8, "ship_type")?;
        writer.push_u32(u32::from(self.dimension_to_bow), 9, "dimension_to_bow")?;
        writer.push_u32(u32::from(self.dimension_to_stern), 9, "dimension_to_stern")?;
        writer.push_u32(u32::from(self.dimension_to_port), 6, "dimension_to_port")?;
        writer.push_u32(
            u32::from(self.dimension_to_starboard),
            6,
            "dimension_to_starboard",
        )?;
        writer.push_u32(
            encode_epfd(self.position_fixing_device)?,
            4,
            "position_fixing_device",
        )?;
        writer.push_bool(self.raim);
        writer.push_bool(self.dte);
        writer.push_bool(self.assigned_mode);
        writer.push_spare(4);
        encode_payload(&writer.finish(), options)
    }
}

impl AisEncodable for ClassBStaticPartA {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        let mut writer = BitWriter::new();
        writer.push_u32(24, 6, "message_type")?;
        writer.push_u32(u32::from(self.repeat_indicator), 2, "repeat_indicator")?;
        writer.push_u32(self.mmsi, 30, "mmsi")?;
        writer.push_u32(0, 2, "part_number")?;
        writer.push_text(&self.vessel_name, 20, "vessel_name")?;
        encode_payload(&writer.finish(), options)
    }
}

impl AisEncodable for ClassBStaticPartB {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        let mut writer = BitWriter::new();
        writer.push_u32(24, 6, "message_type")?;
        writer.push_u32(u32::from(self.repeat_indicator), 2, "repeat_indicator")?;
        writer.push_u32(self.mmsi, 30, "mmsi")?;
        writer.push_u32(1, 2, "part_number")?;
        writer.push_u32(u32::from(self.ship_type), 8, "ship_type")?;
        writer.push_text(&self.manufacturer_id, 3, "manufacturer_id")?;
        writer.push_u32(u32::from(self.model_code), 4, "model_code")?;
        writer.push_u32(self.serial_number, 20, "serial_number")?;
        writer.push_text(&self.callsign, 7, "callsign")?;
        writer.push_u32(u32::from(self.dimension_to_bow), 9, "dimension_to_bow")?;
        writer.push_u32(u32::from(self.dimension_to_stern), 9, "dimension_to_stern")?;
        writer.push_u32(u32::from(self.dimension_to_port), 6, "dimension_to_port")?;
        writer.push_u32(
            u32::from(self.dimension_to_starboard),
            6,
            "dimension_to_starboard",
        )?;
        writer.push_u32(
            encode_epfd(self.position_fixing_device)?,
            4,
            "position_fixing_device",
        )?;
        writer.push_u32(u32::from(self.vdes_capabilities), 2, "vdes_capabilities")?;
        encode_payload(&writer.finish(), options)
    }
}
