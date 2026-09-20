use crate::EncodeError;
use crate::ais::encode::{
    BitWriter, encode_cog, encode_epfd, encode_heading, encode_latitude, encode_longitude,
    encode_position_timestamp, encode_rot, encode_sog,
};
use crate::ais::messages::NavigationStatus;

use super::{AisEncodable, AisTransmitOptions, PositionTimestamp, encode_payload};

/// Class A dynamic position-report type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassAPositionType {
    PositionReport,
    AssignedPositionReport,
    SpecialPositionReport,
}

impl ClassAPositionType {
    fn id(self) -> u32 {
        match self {
            Self::PositionReport => 1,
            Self::AssignedPositionReport => 2,
            Self::SpecialPositionReport => 3,
        }
    }
}

/// AIS Type 1, 2, or 3 Class A position report.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassAPosition {
    pub message_type: ClassAPositionType,
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub navigation_status: NavigationStatus,
    /// Raw AIS ROTAIS value. `None` encodes the not-available value.
    pub rate_of_turn: Option<i8>,
    pub sog: Option<f32>,
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub cog: Option<f32>,
    pub heading: Option<u16>,
    pub timestamp: PositionTimestamp,
    pub maneuver_indicator: u8,
    pub raim: bool,
    /// Raw 19-bit SOTDMA communication state.
    pub communication_state: u32,
}

/// AIS Type 5 Class A static and voyage-related data.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassAStaticVoyage {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub ais_version: u8,
    pub imo: u32,
    pub callsign: String,
    pub vessel_name: String,
    pub ship_type: u8,
    pub dimension_to_bow: u16,
    pub dimension_to_stern: u16,
    pub dimension_to_port: u8,
    pub dimension_to_starboard: u8,
    pub position_fixing_device: u8,
    pub eta_month: Option<u8>,
    pub eta_day: Option<u8>,
    pub eta_hour: Option<u8>,
    pub eta_minute: Option<u8>,
    pub draught_meters: Option<f32>,
    pub destination: String,
    /// `false` means data terminal equipment is available.
    pub dte: bool,
}

impl AisEncodable for ClassAPosition {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        let mut writer = BitWriter::new();
        writer.push_u32(self.message_type.id(), 6, "message_type")?;
        writer.push_u32(u32::from(self.repeat_indicator), 2, "repeat_indicator")?;
        writer.push_u32(self.mmsi, 30, "mmsi")?;
        writer.push_u32(
            u32::from(u8::from(self.navigation_status)),
            4,
            "navigation_status",
        )?;
        writer.push_i32(encode_rot(self.rate_of_turn)?, 8, "rate_of_turn")?;
        writer.push_u32(encode_sog(self.sog)?, 10, "sog")?;
        writer.push_bool(self.position_accuracy);
        writer.push_i32(encode_longitude(self.longitude)?, 28, "longitude")?;
        writer.push_i32(encode_latitude(self.latitude)?, 27, "latitude")?;
        writer.push_u32(encode_cog(self.cog)?, 12, "cog")?;
        writer.push_u32(encode_heading(self.heading)?, 9, "heading")?;
        writer.push_u32(encode_position_timestamp(self.timestamp)?, 6, "timestamp")?;
        writer.push_u32(u32::from(self.maneuver_indicator), 2, "maneuver_indicator")?;
        writer.push_spare(3);
        writer.push_bool(self.raim);
        writer.push_u32(self.communication_state, 19, "communication_state")?;
        encode_payload(&writer.finish(), options)
    }
}

impl AisEncodable for ClassAStaticVoyage {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        let mut writer = BitWriter::new();
        writer.push_u32(5, 6, "message_type")?;
        writer.push_u32(u32::from(self.repeat_indicator), 2, "repeat_indicator")?;
        writer.push_u32(self.mmsi, 30, "mmsi")?;
        writer.push_u32(u32::from(self.ais_version), 2, "ais_version")?;
        writer.push_u32(self.imo, 30, "imo")?;
        writer.push_text(&self.callsign, 7, "callsign")?;
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
        writer.push_u32(
            u32::from(encode_eta_part(self.eta_month, 1, 12, 0, "eta_month")?),
            4,
            "eta_month",
        )?;
        writer.push_u32(
            u32::from(encode_eta_part(self.eta_day, 1, 31, 0, "eta_day")?),
            5,
            "eta_day",
        )?;
        writer.push_u32(
            u32::from(encode_eta_part(self.eta_hour, 0, 23, 24, "eta_hour")?),
            5,
            "eta_hour",
        )?;
        writer.push_u32(
            u32::from(encode_eta_part(self.eta_minute, 0, 59, 60, "eta_minute")?),
            6,
            "eta_minute",
        )?;
        writer.push_u32(encode_draught(self.draught_meters)?, 8, "draught_meters")?;
        writer.push_text(&self.destination, 20, "destination")?;
        writer.push_bool(self.dte);
        writer.push_spare(1);
        encode_payload(&writer.finish(), options)
    }
}

fn encode_eta_part(
    value: Option<u8>,
    minimum: u8,
    maximum: u8,
    not_available: u8,
    field: &'static str,
) -> Result<u8, EncodeError> {
    match value {
        None => Ok(not_available),
        Some(value) if (minimum..=maximum).contains(&value) => Ok(value),
        Some(_) => Err(EncodeError::InvalidAisField(field)),
    }
}

fn encode_draught(value: Option<f32>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(0),
        Some(value) if value.is_finite() && (0.1..=25.5).contains(&value) => {
            Ok((value * 10.0).round() as u32)
        }
        Some(_) => Err(EncodeError::InvalidAisField("draught_meters")),
    }
}
