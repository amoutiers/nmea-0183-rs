use crate::EncodeError;
use crate::ais::encode::{
    BitWriter, encode_cog, encode_epfd, encode_integer_sog, encode_latitude, encode_long_range_cog,
    encode_long_range_latitude, encode_long_range_longitude, encode_long_range_sog,
    encode_longitude, encode_position_timestamp, encode_sar_altitude, encode_utc_day,
    encode_utc_hour, encode_utc_minute_or_second, encode_utc_month, encode_utc_year,
};
use crate::ais::messages::NavigationStatus;

use super::{
    AisEncodable, AisTransmitOptions, ClassBCommunicationState, PositionTimestamp, encode_payload,
};

/// AIS Type 4 base-station report.
#[derive(Debug, Clone, PartialEq)]
pub struct BaseStation {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub position_fixing_device: u8,
    pub transmission_control: bool,
    pub raim: bool,
    pub communication_state: u32,
}

/// AIS Type 11 UTC/date response.
#[derive(Debug, Clone, PartialEq)]
pub struct UtcDateResponse {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub position_fixing_device: u8,
    pub transmission_control: bool,
    pub raim: bool,
    pub communication_state: u32,
}

/// AIS Type 9 standard SAR aircraft position report.
#[derive(Debug, Clone, PartialEq)]
pub struct SarAircraft {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub altitude: Option<u16>,
    pub sog: Option<u16>,
    pub position_accuracy: bool,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub cog: Option<f32>,
    pub timestamp: PositionTimestamp,
    pub regional_application: u8,
    /// `false` means the DTE is available.
    pub dte: bool,
    pub assigned_mode: bool,
    pub raim: bool,
    pub communication_state: ClassBCommunicationState,
}

/// AIS Type 21 aid-to-navigation report.
#[derive(Debug, Clone, PartialEq)]
pub struct AidToNavigation {
    pub repeat_indicator: u8,
    pub mmsi: u32,
    pub aid_type: u8,
    pub name: String,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
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

/// AIS Type 27 long-range position report.
#[derive(Debug, Clone, PartialEq)]
pub struct LongRangePosition {
    pub mmsi: u32,
    pub position_accuracy: bool,
    pub raim: bool,
    pub navigation_status: NavigationStatus,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub sog: Option<u8>,
    pub cog: Option<u16>,
    /// `false` = GNSS position less than five seconds old; `true` = more than five seconds old.
    pub gnss_position_status: bool,
}

struct UtcPositionInput {
    repeat_indicator: u8,
    mmsi: u32,
    year: Option<u16>,
    month: Option<u8>,
    day: Option<u8>,
    hour: Option<u8>,
    minute: Option<u8>,
    second: Option<u8>,
    position_accuracy: bool,
    longitude: Option<f64>,
    latitude: Option<f64>,
    position_fixing_device: u8,
    transmission_control: bool,
    raim: bool,
    communication_state: u32,
}

impl AisEncodable for BaseStation {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        encode_utc_position(4, self.utc_position_input(), options)
    }
}

impl AisEncodable for UtcDateResponse {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        encode_utc_position(11, self.utc_position_input(), options)
    }
}

impl AisEncodable for SarAircraft {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        let mut writer = BitWriter::new();
        let (selector, communication_state) = communication_state_parts(self.communication_state);
        writer.push_u32(9, 6, "message_type")?;
        writer.push_u32(u32::from(self.repeat_indicator), 2, "repeat_indicator")?;
        writer.push_u32(self.mmsi, 30, "mmsi")?;
        writer.push_u32(encode_sar_altitude(self.altitude)?, 12, "altitude")?;
        writer.push_u32(encode_integer_sog(self.sog)?, 10, "sog")?;
        writer.push_bool(self.position_accuracy);
        writer.push_i32(encode_longitude(self.longitude)?, 28, "longitude")?;
        writer.push_i32(encode_latitude(self.latitude)?, 27, "latitude")?;
        writer.push_u32(encode_cog(self.cog)?, 12, "cog")?;
        writer.push_u32(encode_position_timestamp(self.timestamp)?, 6, "timestamp")?;
        writer.push_u32(
            u32::from(self.regional_application),
            8,
            "regional_application",
        )?;
        writer.push_bool(self.dte);
        writer.push_spare(3);
        writer.push_bool(self.assigned_mode);
        writer.push_bool(self.raim);
        writer.push_bool(selector);
        writer.push_u32(communication_state, 19, "communication_state")?;
        encode_payload(&writer.finish(), options)
    }
}

impl AisEncodable for AidToNavigation {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        let mut writer = BitWriter::new();
        writer.push_u32(21, 6, "message_type")?;
        writer.push_u32(u32::from(self.repeat_indicator), 2, "repeat_indicator")?;
        writer.push_u32(self.mmsi, 30, "mmsi")?;
        writer.push_u32(u32::from(self.aid_type), 5, "aid_type")?;
        writer.push_text(&self.name, 20, "name")?;
        writer.push_bool(self.position_accuracy);
        writer.push_i32(encode_longitude(self.longitude)?, 28, "longitude")?;
        writer.push_i32(encode_latitude(self.latitude)?, 27, "latitude")?;
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
        writer.push_u32(encode_position_timestamp(self.timestamp)?, 6, "timestamp")?;
        writer.push_bool(self.off_position);
        writer.push_u32(
            u32::from(self.regional_application),
            8,
            "regional_application",
        )?;
        writer.push_bool(self.raim);
        writer.push_bool(self.virtual_aid);
        writer.push_bool(self.assigned_mode);
        writer.push_spare(1);
        if let Some(extension) = &self.name_extension {
            let actual_chars = extension.chars().count();
            if actual_chars > 14 {
                return Err(EncodeError::AisTextTooLong {
                    field: "name_extension",
                    max_chars: 14,
                    actual_chars,
                });
            }
            writer.push_text(extension, actual_chars, "name_extension")?;
        }
        encode_payload(&writer.finish(), options)
    }
}

impl AisEncodable for LongRangePosition {
    fn to_sentences(&self, options: AisTransmitOptions) -> Result<Vec<String>, EncodeError> {
        let mut writer = BitWriter::new();
        writer.push_u32(27, 6, "message_type")?;
        writer.push_u32(3, 2, "repeat_indicator")?;
        writer.push_u32(self.mmsi, 30, "mmsi")?;
        writer.push_bool(self.position_accuracy);
        writer.push_bool(self.raim);
        writer.push_u32(
            u32::from(u8::from(self.navigation_status)),
            4,
            "navigation_status",
        )?;
        writer.push_i32(
            encode_long_range_longitude(self.longitude)?,
            18,
            "longitude",
        )?;
        writer.push_i32(encode_long_range_latitude(self.latitude)?, 17, "latitude")?;
        writer.push_u32(encode_long_range_sog(self.sog)?, 6, "sog")?;
        writer.push_u32(encode_long_range_cog(self.cog)?, 9, "cog")?;
        writer.push_bool(self.gnss_position_status);
        writer.push_spare(1);
        encode_payload(&writer.finish(), options)
    }
}

fn encode_utc_position(
    message_type: u32,
    input: UtcPositionInput,
    options: AisTransmitOptions,
) -> Result<Vec<String>, EncodeError> {
    let mut writer = BitWriter::new();
    writer.push_u32(message_type, 6, "message_type")?;
    writer.push_u32(u32::from(input.repeat_indicator), 2, "repeat_indicator")?;
    writer.push_u32(input.mmsi, 30, "mmsi")?;
    writer.push_u32(encode_utc_year(input.year)?, 14, "year")?;
    writer.push_u32(encode_utc_month(input.month)?, 4, "month")?;
    writer.push_u32(encode_utc_day(input.day)?, 5, "day")?;
    writer.push_u32(encode_utc_hour(input.hour)?, 5, "hour")?;
    writer.push_u32(
        encode_utc_minute_or_second(input.minute, "minute")?,
        6,
        "minute",
    )?;
    writer.push_u32(
        encode_utc_minute_or_second(input.second, "second")?,
        6,
        "second",
    )?;
    writer.push_bool(input.position_accuracy);
    writer.push_i32(encode_longitude(input.longitude)?, 28, "longitude")?;
    writer.push_i32(encode_latitude(input.latitude)?, 27, "latitude")?;
    writer.push_u32(
        encode_epfd(input.position_fixing_device)?,
        4,
        "position_fixing_device",
    )?;
    writer.push_bool(input.transmission_control);
    writer.push_spare(9);
    writer.push_bool(input.raim);
    writer.push_u32(input.communication_state, 19, "communication_state")?;
    encode_payload(&writer.finish(), options)
}

trait UtcPositionSource {
    fn utc_position_input(&self) -> UtcPositionInput;
}

impl UtcPositionSource for BaseStation {
    fn utc_position_input(&self) -> UtcPositionInput {
        UtcPositionInput {
            repeat_indicator: self.repeat_indicator,
            mmsi: self.mmsi,
            year: self.year,
            month: self.month,
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
            position_accuracy: self.position_accuracy,
            longitude: self.longitude,
            latitude: self.latitude,
            position_fixing_device: self.position_fixing_device,
            transmission_control: self.transmission_control,
            raim: self.raim,
            communication_state: self.communication_state,
        }
    }
}

impl UtcPositionSource for UtcDateResponse {
    fn utc_position_input(&self) -> UtcPositionInput {
        UtcPositionInput {
            repeat_indicator: self.repeat_indicator,
            mmsi: self.mmsi,
            year: self.year,
            month: self.month,
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
            position_accuracy: self.position_accuracy,
            longitude: self.longitude,
            latitude: self.latitude,
            position_fixing_device: self.position_fixing_device,
            transmission_control: self.transmission_control,
            raim: self.raim,
            communication_state: self.communication_state,
        }
    }
}

fn communication_state_parts(value: ClassBCommunicationState) -> (bool, u32) {
    match value {
        ClassBCommunicationState::Sotdma(value) => (false, value),
        ClassBCommunicationState::Itdma(value) => (true, value),
    }
}
