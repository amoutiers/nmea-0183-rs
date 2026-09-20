use crate::EncodeError;
use crate::ais::messages::PositionTimestamp;

pub(crate) struct BitWriter {
    bits: Vec<u8>,
}

impl BitWriter {
    pub(crate) fn new() -> Self {
        Self { bits: Vec::new() }
    }

    pub(crate) fn push_u32(
        &mut self,
        value: u32,
        width: usize,
        field: &'static str,
    ) -> Result<(), EncodeError> {
        if width == 0 || width > 32 || (width < 32 && value >= (1u32 << width)) {
            return Err(EncodeError::InvalidAisField(field));
        }
        for shift in (0..width).rev() {
            self.bits.push(((value >> shift) & 1) as u8);
        }
        Ok(())
    }

    pub(crate) fn push_i32(
        &mut self,
        value: i32,
        width: usize,
        field: &'static str,
    ) -> Result<(), EncodeError> {
        if width == 0 || width > 32 {
            return Err(EncodeError::InvalidAisField(field));
        }
        let min = -(1i64 << (width - 1));
        let max = (1i64 << (width - 1)) - 1;
        let value_i64 = i64::from(value);
        if value_i64 < min || value_i64 > max {
            return Err(EncodeError::InvalidAisField(field));
        }
        let encoded = if value < 0 {
            ((1i64 << width) + value_i64) as u32
        } else {
            value as u32
        };
        self.push_u32(encoded, width, field)
    }

    pub(crate) fn push_bool(&mut self, value: bool) {
        self.bits.push(u8::from(value));
    }

    pub(crate) fn push_spare(&mut self, width: usize) {
        self.bits.extend(core::iter::repeat_n(0, width));
    }

    pub(crate) fn push_text(
        &mut self,
        value: &str,
        width_chars: usize,
        field: &'static str,
    ) -> Result<(), EncodeError> {
        let actual_chars = value.chars().count();
        if actual_chars > width_chars {
            return Err(EncodeError::AisTextTooLong {
                field,
                max_chars: width_chars,
                actual_chars,
            });
        }
        for ch in value.chars() {
            self.push_u32(u32::from(encode_text_char(ch, field)?), 6, field)?;
        }
        for _ in actual_chars..width_chars {
            self.push_u32(0, 6, field)?;
        }
        Ok(())
    }

    /// Finish a complete AIS message, adding zero spare bits to a byte boundary.
    pub(crate) fn finish(mut self) -> Vec<u8> {
        self.push_spare((8 - self.bits.len() % 8) % 8);
        self.bits
    }
}

fn encode_text_char(ch: char, field: &'static str) -> Result<u8, EncodeError> {
    let value = match ch {
        '@' => 0,
        'A'..='Z' => ch as u8 - b'A' + 1,
        '['..='_' => ch as u8 - b'[' + 27,
        ' ' => 32,
        '!'..='9' => ch as u8 - b'!' + 33,
        ':'..='?' => ch as u8 - b':' + 58,
        _ => return Err(EncodeError::InvalidAisField(field)),
    };
    Ok(value)
}

pub(crate) fn encode_sog(value: Option<f32>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(1023),
        Some(value) if value.is_finite() && (0.0..=102.2).contains(&value) => {
            Ok((value * 10.0).round() as u32)
        }
        Some(_) => Err(EncodeError::InvalidAisField("sog")),
    }
}

pub(crate) fn encode_cog(value: Option<f32>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(3600),
        Some(value) if value.is_finite() && (0.0..=359.9).contains(&value) => {
            Ok((value * 10.0).round() as u32)
        }
        Some(_) => Err(EncodeError::InvalidAisField("cog")),
    }
}

pub(crate) fn encode_heading(value: Option<u16>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(511),
        Some(value) if value <= 359 => Ok(u32::from(value)),
        Some(_) => Err(EncodeError::InvalidAisField("heading")),
    }
}

pub(crate) fn encode_position_timestamp(value: PositionTimestamp) -> Result<u32, EncodeError> {
    match value {
        PositionTimestamp::Exact(value) if value <= 59 => Ok(u32::from(value)),
        PositionTimestamp::Exact(_) => Err(EncodeError::InvalidAisField("timestamp")),
        PositionTimestamp::NotAvailable => Ok(60),
        PositionTimestamp::ManualInput => Ok(61),
        PositionTimestamp::DeadReckoning => Ok(62),
        PositionTimestamp::Inoperative => Ok(63),
    }
}

pub(crate) fn encode_epfd(value: u8) -> Result<u32, EncodeError> {
    match value {
        0..=9 | 12..=15 => Ok(u32::from(value)),
        _ => Err(EncodeError::InvalidAisField("position_fixing_device")),
    }
}

pub(crate) fn encode_utc_year(value: Option<u16>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(0),
        Some(value) if (1..=9_999).contains(&value) => Ok(u32::from(value)),
        Some(_) => Err(EncodeError::InvalidAisField("year")),
    }
}

pub(crate) fn encode_utc_month(value: Option<u8>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(0),
        Some(value) if (1..=12).contains(&value) => Ok(u32::from(value)),
        Some(_) => Err(EncodeError::InvalidAisField("month")),
    }
}

pub(crate) fn encode_utc_day(value: Option<u8>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(0),
        Some(value) if (1..=31).contains(&value) => Ok(u32::from(value)),
        Some(_) => Err(EncodeError::InvalidAisField("day")),
    }
}

pub(crate) fn encode_utc_hour(value: Option<u8>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(24),
        Some(value) if value <= 23 => Ok(u32::from(value)),
        Some(_) => Err(EncodeError::InvalidAisField("hour")),
    }
}

pub(crate) fn encode_utc_minute_or_second(
    value: Option<u8>,
    field: &'static str,
) -> Result<u32, EncodeError> {
    match value {
        None => Ok(60),
        Some(value) if value <= 59 => Ok(u32::from(value)),
        Some(_) => Err(EncodeError::InvalidAisField(field)),
    }
}

pub(crate) fn encode_sar_altitude(value: Option<u16>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(4_095),
        Some(value) if value <= 4_094 => Ok(u32::from(value)),
        Some(_) => Err(EncodeError::InvalidAisField("altitude")),
    }
}

pub(crate) fn encode_integer_sog(value: Option<u16>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(1_023),
        Some(value) if value <= 1_022 => Ok(u32::from(value)),
        Some(_) => Err(EncodeError::InvalidAisField("sog")),
    }
}

pub(crate) fn encode_long_range_sog(value: Option<u8>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(63),
        Some(value) if value <= 62 => Ok(u32::from(value)),
        Some(_) => Err(EncodeError::InvalidAisField("sog")),
    }
}

pub(crate) fn encode_long_range_cog(value: Option<u16>) -> Result<u32, EncodeError> {
    match value {
        None => Ok(511),
        Some(value) if value <= 359 => Ok(u32::from(value)),
        Some(_) => Err(EncodeError::InvalidAisField("cog")),
    }
}

pub(crate) fn encode_long_range_longitude(value: Option<f64>) -> Result<i32, EncodeError> {
    match value {
        None => Ok(108_600),
        Some(value) if value.is_finite() && (-180.0..=180.0).contains(&value) => {
            Ok((value * 600.0).round() as i32)
        }
        Some(_) => Err(EncodeError::InvalidAisField("longitude")),
    }
}

pub(crate) fn encode_long_range_latitude(value: Option<f64>) -> Result<i32, EncodeError> {
    match value {
        None => Ok(54_600),
        Some(value) if value.is_finite() && (-90.0..=90.0).contains(&value) => {
            Ok((value * 600.0).round() as i32)
        }
        Some(_) => Err(EncodeError::InvalidAisField("latitude")),
    }
}

pub(crate) fn encode_rot(value: Option<i8>) -> Result<i32, EncodeError> {
    match value {
        None => Ok(-128),
        Some(-128) => Err(EncodeError::InvalidAisField("rate_of_turn")),
        Some(value) => Ok(i32::from(value)),
    }
}

pub(crate) fn encode_longitude(value: Option<f64>) -> Result<i32, EncodeError> {
    match value {
        None => Ok(108_600_000),
        Some(value) if value.is_finite() && (-180.0..=180.0).contains(&value) => {
            Ok((value * 600_000.0).round() as i32)
        }
        Some(_) => Err(EncodeError::InvalidAisField("longitude")),
    }
}

pub(crate) fn encode_latitude(value: Option<f64>) -> Result<i32, EncodeError> {
    match value {
        None => Ok(54_600_000),
        Some(value) if value.is_finite() && (-90.0..=90.0).contains(&value) => {
            Ok((value * 600_000.0).round() as i32)
        }
        Some(_) => Err(EncodeError::InvalidAisField("latitude")),
    }
}
