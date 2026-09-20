pub(super) fn read_u8(fields: &[&str], idx: &mut usize) -> Option<u8> {
    let val = fields.get(*idx).and_then(|f| {
        if f.is_empty() {
            None
        } else {
            f.parse::<u8>().ok()
        }
    });
    *idx += 1;
    val
}

#[cfg_attr(
    not(feature = "abm"),
    expect(dead_code, reason = "used only by AIS ABM sentence parsing")
)]
pub(super) fn read_u32(fields: &[&str], idx: &mut usize) -> Option<u32> {
    let val = fields.get(*idx).and_then(|f| {
        if f.is_empty() {
            None
        } else {
            f.parse::<u32>().ok()
        }
    });
    *idx += 1;
    val
}

#[cfg_attr(
    not(any(feature = "abm", feature = "bbm")),
    expect(dead_code, reason = "used only by AIS ABM/BBM sentence parsing")
)]
pub(super) fn read_char(fields: &[&str], idx: &mut usize) -> Option<char> {
    let val = fields.get(*idx).and_then(|field| {
        let mut chars = field.chars();
        let first = chars.next()?;
        chars.next().is_none().then_some(first)
    });
    *idx += 1;
    val
}

pub(super) fn read_string(fields: &[&str], idx: &mut usize) -> Option<String> {
    let val = fields.get(*idx).and_then(|f| {
        if f.is_empty() {
            None
        } else {
            Some((*f).to_string())
        }
    });
    *idx += 1;
    val
}

pub(super) fn encode_u8(value: Option<u8>) -> String {
    value.map(|v| v.to_string()).unwrap_or_default()
}

#[cfg_attr(
    not(feature = "abm"),
    expect(dead_code, reason = "used only by AIS ABM sentence encoding")
)]
pub(super) fn encode_u32(value: Option<u32>) -> String {
    value.map(|v| v.to_string()).unwrap_or_default()
}

#[cfg_attr(
    not(any(feature = "abm", feature = "bbm")),
    expect(dead_code, reason = "used only by AIS ABM/BBM sentence encoding")
)]
pub(super) fn encode_char(value: Option<char>) -> String {
    value.map(|v| v.to_string()).unwrap_or_default()
}
