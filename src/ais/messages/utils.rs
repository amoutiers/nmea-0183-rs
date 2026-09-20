//! Shared AIS decode helpers used across multiple message types.

/// Decode latitude from 1/10000 minute to degrees. 91° = not available.
pub(crate) fn decode_latitude(raw: i32) -> Option<f64> {
    let deg = f64::from(raw) / 600_000.0;
    if !(-90.0..=90.0).contains(&deg) {
        None
    } else {
        Some(deg)
    }
}

/// Decode longitude from 1/10000 minute to degrees. 181° = not available.
pub(crate) fn decode_longitude(raw: i32) -> Option<f64> {
    let deg = f64::from(raw) / 600_000.0;
    if !(-180.0..=180.0).contains(&deg) {
        None
    } else {
        Some(deg)
    }
}

/// Decode SOG from 1/10 knot. 1023 = not available.
pub(crate) fn decode_sog(raw: u32) -> Option<f32> {
    if raw == 1023 {
        None
    } else {
        Some(raw as f32 / 10.0)
    }
}

/// Decode COG from 1/10 degree. Values 3600 and above are reserved.
pub(crate) fn decode_cog(raw: u32) -> Option<f32> {
    (raw < 3600).then_some(raw as f32 / 10.0)
}

/// Decode true heading in degrees. 511 = not available; 360-510 reserved.
pub(crate) fn decode_heading(raw: u32) -> Option<u16> {
    if raw > 359 { None } else { Some(raw as u16) }
}

/// Decode rate of turn. Returns the raw ITU-R M.1371 ROT_AIS indicator value (not deg/min). -128 = not available.
pub(crate) fn decode_rot(raw: i32) -> Option<f32> {
    if raw == -128 { None } else { Some(raw as f32) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_filters_reserved_values() {
        assert_eq!(decode_heading(0), Some(0));
        assert_eq!(decode_heading(359), Some(359));
        assert_eq!(decode_heading(360), None);
        assert_eq!(decode_heading(400), None);
        assert_eq!(decode_heading(511), None);
    }

    #[test]
    fn course_filters_reserved_values() {
        assert_eq!(decode_cog(0), Some(0.0));
        assert_eq!(decode_cog(3599), Some(359.9));
        assert_eq!(decode_cog(3600), None);
        assert_eq!(decode_cog(3601), None);
        assert_eq!(decode_cog(4095), None);
    }
}
