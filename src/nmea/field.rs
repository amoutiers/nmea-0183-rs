//! Field parsing and formatting helpers for NMEA sentence fields.
//!
//! [`FieldReader`] reads fields sequentially from a parsed frame.
//! [`FieldWriter`] builds fields sequentially for encoding.

use crate::EncodeError;

/// Sequential field reader for NMEA sentence parsing.
///
/// Wraps a slice of `&str` fields and reads them in order,
/// advancing an internal index after each read.
pub(crate) struct FieldReader<'a> {
    fields: &'a [&'a str],
    idx: usize,
}

#[cfg_attr(
    not(feature = "nmea"),
    expect(
        dead_code,
        reason = "several field helpers are exercised only by sentence features absent from a single-feature build"
    )
)]
impl<'a> FieldReader<'a> {
    pub(crate) fn new(fields: &'a [&'a str]) -> Self {
        Self { fields, idx: 0 }
    }

    /// Read an optional f32 and advance.
    pub(crate) fn f32(&mut self) -> Option<f32> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                f.parse::<f32>().ok().filter(|v| v.is_finite())
            }
        });
        self.idx += 1;
        val
    }

    /// Read an optional f64 and advance.
    pub(crate) fn f64(&mut self) -> Option<f64> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                f.parse::<f64>().ok().filter(|v| v.is_finite())
            }
        });
        self.idx += 1;
        val
    }

    /// Read an optional u8 and advance.
    pub(crate) fn u8(&mut self) -> Option<u8> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                f.parse::<u8>().ok()
            }
        });
        self.idx += 1;
        val
    }

    /// Read an optional u32 and advance.
    pub(crate) fn u32(&mut self) -> Option<u32> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                f.parse::<u32>().ok()
            }
        });
        self.idx += 1;
        val
    }

    /// Read an optional i8 and advance.
    pub(crate) fn i8(&mut self) -> Option<i8> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                f.parse::<i8>().ok()
            }
        });
        self.idx += 1;
        val
    }

    /// Read an optional u16 and advance.
    #[expect(dead_code, reason = "reserved for future sentence types")]
    pub(crate) fn u16(&mut self) -> Option<u16> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() { None } else { f.parse::<u16>().ok() }
        });
        self.idx += 1;
        val
    }

    /// Read an optional i16 and advance.
    #[expect(dead_code, reason = "reserved for future sentence types")]
    pub(crate) fn i16(&mut self) -> Option<i16> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() { None } else { f.parse::<i16>().ok() }
        });
        self.idx += 1;
        val
    }

    /// Read an optional i32 and advance.
    #[expect(dead_code, reason = "reserved for future sentence types")]
    pub(crate) fn i32(&mut self) -> Option<i32> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() { None } else { f.parse::<i32>().ok() }
        });
        self.idx += 1;
        val
    }

    /// Read an optional single character and advance.
    pub(crate) fn char(&mut self) -> Option<char> {
        let val = self
            .fields
            .get(self.idx)
            .and_then(|f| f.chars().next().filter(|_| !f.is_empty()));
        self.idx += 1;
        val
    }

    /// Read an optional non-empty string and advance.
    pub(crate) fn string(&mut self) -> Option<String> {
        let val = self.fields.get(self.idx).and_then(|f| {
            if f.is_empty() {
                None
            } else {
                Some((*f).to_string())
            }
        });
        self.idx += 1;
        val
    }

    /// Skip one field (fixed indicator) and advance.
    pub(crate) fn skip(&mut self) {
        self.idx += 1;
    }
}

/// Sequential field writer for NMEA sentence encoding.
///
/// Builds a `Vec<String>` of field values in wire order.
pub(crate) struct FieldWriter {
    fields: Vec<String>,
    error: Option<EncodeError>,
}

// Shared logic for f32/f64 writers: non-finite → empty, -0.0 → "0".
// Expands at the call site so type inference resolves `0.0` and `is_finite()`
// to the concrete float type.
macro_rules! push_optional_float {
    ($fields:expr, $value:expr) => {
        $fields.push(match $value {
            Some(v) if !v.is_finite() => String::new(),
            Some(v) => format!("{}", if v == 0.0 { 0.0 } else { v }),
            None => String::new(),
        });
    };
}

#[cfg_attr(
    not(feature = "nmea"),
    expect(
        dead_code,
        reason = "several field helpers are exercised only by sentence features absent from a single-feature build"
    )
)]
impl FieldWriter {
    pub(crate) fn new() -> Self {
        Self {
            fields: Vec::new(),
            error: None,
        }
    }

    /// Write an optional f32. `None` → empty field. Non-finite → empty field. `-0.0` → `"0"`.
    pub(crate) fn f32(&mut self, value: Option<f32>) {
        push_optional_float!(self.fields, value);
    }

    /// Write an optional f64. `None` → empty field. Non-finite → empty field. `-0.0` → `"0"`.
    #[cfg_attr(not(test), expect(dead_code, reason = "reserved for future sentence types"))]
    pub(crate) fn f64(&mut self, value: Option<f64>) {
        push_optional_float!(self.fields, value);
    }

    /// Write an optional u8. `None` → empty field.
    pub(crate) fn u8(&mut self, value: Option<u8>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional i8. `None` → empty field.
    pub(crate) fn i8(&mut self, value: Option<i8>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional u32. `None` → empty field.
    pub(crate) fn u32(&mut self, value: Option<u32>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional u16. `None` → empty field.
    #[expect(dead_code, reason = "reserved for future sentence types")]
    pub(crate) fn u16(&mut self, value: Option<u16>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional i16. `None` → empty field.
    #[expect(dead_code, reason = "reserved for future sentence types")]
    pub(crate) fn i16(&mut self, value: Option<i16>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional i32. `None` → empty field.
    #[expect(dead_code, reason = "reserved for future sentence types")]
    pub(crate) fn i32(&mut self, value: Option<i32>) {
        self.fields.push(match value {
            Some(v) => v.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional char. `None` → empty field.
    pub(crate) fn char(&mut self, value: Option<char>) {
        self.fields.push(match value {
            Some(c) => c.to_string(),
            None => String::new(),
        });
    }

    /// Write an optional latitude in NMEA `DDMM.MMMM` form (integer part zero-padded
    /// to 4 digits). `None` → empty field.
    pub(crate) fn lat(&mut self, value: Option<f64>) {
        self.push_coord(value, 4);
    }

    /// Write an optional longitude in NMEA `DDDMM.MMMM` form (integer part zero-padded
    /// to 5 digits). `None` → empty field.
    pub(crate) fn lon(&mut self, value: Option<f64>) {
        self.push_coord(value, 5);
    }

    /// Format a coordinate by zero-padding the integer part to `int_width` digits and
    /// keeping the fractional part as the shortest `f64` representation. The value must
    /// be a non-negative magnitude — the N/S·E/W sign lives in a separate indicator
    /// field. Non-finite or negative values record `EncodeError::InvalidCoordinate`
    /// (surfaced by `finish()`) and emit an empty field. `-0.0` is normalized to `0.0`.
    fn push_coord(&mut self, value: Option<f64>, int_width: usize) {
        let v = match value {
            Some(v) if v.is_finite() && v >= 0.0 => v,
            Some(_) => {
                if self.error.is_none() {
                    self.error = Some(EncodeError::InvalidCoordinate);
                }
                self.fields.push(String::new());
                return;
            }
            None => {
                self.fields.push(String::new());
                return;
            }
        };
        let v = if v == 0.0 { 0.0 } else { v }; // normalize -0.0
        let s = format!("{v}");
        self.fields.push(match s.split_once('.') {
            Some((int, frac)) => format!("{int:0>int_width$}.{frac}"),
            None => format!("{s:0>int_width$}.0"),
        });
    }

    /// Write a fixed indicator character (always emitted).
    pub(crate) fn fixed(&mut self, c: char) {
        self.fields.push(c.to_string());
    }

    /// Write an optional string. `None` → empty field. A value containing `,`, `*`,
    /// `\r`, `\n`, or a non-ASCII character records `EncodeError::InvalidFieldCharacter`
    /// (surfaced by `finish()`) and emits an empty field.
    pub(crate) fn string(&mut self, value: Option<&str>) {
        match value {
            Some(s) => {
                if let Some(c) = s
                    .chars()
                    .find(|&c| !c.is_ascii() || matches!(c, ',' | '*' | '\r' | '\n'))
                {
                    if self.error.is_none() {
                        self.error = Some(EncodeError::InvalidFieldCharacter(c));
                    }
                    self.fields.push(String::new());
                } else {
                    self.fields.push(s.to_string());
                }
            }
            None => self.fields.push(String::new()),
        }
    }

    /// Consume and return the built field list, or the first recorded encode error.
    pub(crate) fn finish(self) -> Result<Vec<String>, EncodeError> {
        match self.error {
            Some(e) => Err(e),
            None => Ok(self.fields),
        }
    }
}

impl Default for FieldWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for NMEA sentence types that can be encoded to wire format.
///
/// Provides `SENTENCE_TYPE` and `encode()` — the `to_sentence()` default
/// method combines them with [`encode_frame()`](crate::encode_frame) to
/// produce a complete NMEA sentence with checksum.
///
/// # Standard sentences
///
/// ```
/// #[cfg(feature = "dpt")]
/// {
///     use nmea_kit::nmea::{NmeaEncodable, sentences::Dpt};
///
///     let dpt = Dpt { depth: Some(4.1), offset: Some(0.0), rangescale: None };
///     let sentence = dpt.to_sentence("II").expect("encode");
///     assert!(sentence.starts_with("$IIDPT,"));
/// }
/// ```
///
/// # Proprietary sentences
///
/// Proprietary types (`$P...`) set [`SENTENCE_TYPE`](Self::SENTENCE_TYPE) to the full
/// address (e.g. `"PASHR"`, `"PSKPDPT"`) and [`PROPRIETARY`](Self::PROPRIETARY) to `true`.
/// `to_sentence` then ignores the `talker` argument and emits the full address.
pub trait NmeaEncodable {
    /// The NMEA sentence prefix. Standard sentences use `$`; encapsulation
    /// sentences may override this with `!`.
    const PREFIX: char = '$';

    /// The wire address: the 3-character type for standard sentences (e.g. `"RMC"`),
    /// or the full address for proprietary sentences (e.g. `"PSKPDPT"`).
    const SENTENCE_TYPE: &'static str;

    /// `true` when [`SENTENCE_TYPE`](Self::SENTENCE_TYPE) is a full proprietary address.
    /// [`to_sentence`](Self::to_sentence) then ignores the `talker` argument.
    const PROPRIETARY: bool = false;

    /// Encode fields into a `Vec` of strings in wire order.
    fn encode(&self) -> Result<Vec<String>, EncodeError>;

    /// Encode into a complete NMEA 0183 sentence with checksum and `\r\n`.
    ///
    /// For standard sentences the `talker` is prepended to `SENTENCE_TYPE`. For
    /// proprietary sentences (`PROPRIETARY == true`) the `talker` is ignored — a
    /// proprietary address carries no talker — and `SENTENCE_TYPE` is the full address.
    fn to_sentence(&self, talker: &str) -> Result<String, EncodeError> {
        let fields = self.encode()?;
        let field_refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        let talker = if Self::PROPRIETARY { "" } else { talker };
        crate::encode_frame(Self::PREFIX, talker, Self::SENTENCE_TYPE, &field_refs)
    }
}

/// Convert an NMEA `DDMM.MMMM` coordinate to decimal degrees.
///
/// NMEA sentences encode latitude as `DDMM.MMMM` (degrees + minutes) and
/// longitude as `DDDMM.MMMM`. AIS and most application code use decimal degrees.
/// The sign (N/S, E/W) is not part of `ddmm` — apply it after conversion.
///
/// # Example
///
/// ```
/// use nmea_kit::nmea::ddmm_to_decimal;
///
/// // 4807.038 → 48°07.038′ → 48.1173°
/// let lat = ddmm_to_decimal(4807.038);
/// assert!((lat - 48.1173).abs() < 0.0001);
/// ```
pub fn ddmm_to_decimal(ddmm: f64) -> f64 {
    let degrees = (ddmm / 100.0).floor();
    let minutes = ddmm - degrees * 100.0;
    degrees + minutes / 60.0
}

/// Convert decimal degrees to an NMEA `DDMM.MMMM` coordinate.
///
/// This is the inverse of [`ddmm_to_decimal`]. The sign is not encoded —
/// strip it before calling and re-apply the N/S or E/W indicator separately.
///
/// # Example
///
/// ```
/// use nmea_kit::nmea::decimal_to_ddmm;
///
/// // 48.1173° → 48°07.038′ → 4807.038
/// let ddmm = decimal_to_ddmm(48.1173);
/// assert!((ddmm - 4807.038).abs() < 0.001);
/// ```
pub fn decimal_to_ddmm(decimal: f64) -> f64 {
    let degrees = decimal.floor();
    let minutes = (decimal - degrees) * 60.0;
    degrees * 100.0 + minutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_char() {
        let fields = &["T", "", "AB"];
        let mut r = FieldReader::new(fields);
        assert_eq!(r.char(), Some('T'));
        assert_eq!(r.char(), None);
        assert_eq!(r.char(), Some('A')); // takes first char
    }

    #[test]
    fn reader_f32() {
        let fields = &["270.0", "", "abc"];
        let mut r = FieldReader::new(fields);
        assert_eq!(r.f32(), Some(270.0));
        assert_eq!(r.f32(), None);
        assert_eq!(r.f32(), None); // invalid
    }

    #[test]
    fn reader_past_end() {
        let fields: &[&str] = &[];
        let mut r = FieldReader::new(fields);
        assert_eq!(r.f32(), None);
        assert_eq!(r.char(), None);
    }

    #[test]
    fn reader_skip() {
        let fields = &["10.0", "T", "20.0"];
        let mut r = FieldReader::new(fields);
        assert_eq!(r.f32(), Some(10.0));
        r.skip();
        assert_eq!(r.f32(), Some(20.0));
    }

    #[test]
    fn reader_string() {
        let fields = &["DEST", ""];
        let mut r = FieldReader::new(fields);
        assert_eq!(r.string(), Some("DEST".to_string()));
        assert_eq!(r.string(), None);
    }

    #[test]
    fn writer_roundtrip() {
        let mut w = FieldWriter::new();
        w.f32(Some(270.0));
        w.fixed('T');
        w.f32(None);
        w.fixed('M');
        let fields = w.finish().expect("encode");
        assert_eq!(fields, vec!["270", "T", "", "M"]);
    }

    #[test]
    fn ddmm_to_decimal_lat() {
        // 4807.038 → 48°07.038′ → 48.1173°
        let result = ddmm_to_decimal(4807.038);
        assert!((result - 48.1173).abs() < 0.0001);
    }

    #[test]
    fn ddmm_to_decimal_lon() {
        // 01131.000 → 11°31.000′ → 11.5167°
        let result = ddmm_to_decimal(1131.0);
        assert!((result - 11.5167).abs() < 0.0001);
    }

    #[test]
    fn decimal_to_ddmm_lat() {
        // 48.1173° → 4807.038
        let result = decimal_to_ddmm(48.1173);
        assert!((result - 4807.038).abs() < 0.001);
    }

    #[test]
    fn decimal_to_ddmm_roundtrip() {
        let original = 5132.5200_f64;
        let roundtrip = decimal_to_ddmm(ddmm_to_decimal(original));
        assert!((roundtrip - original).abs() < 0.0001);
    }

    #[test]
    fn writer_non_finite_and_neg_zero() {
        let mut w = FieldWriter::new();
        w.f32(Some(f32::NAN));        // -> "" (not "NaN")
        w.f64(Some(f64::INFINITY));   // -> "" (not "inf")
        w.f64(Some(-0.0));            // -> "0" (not "-0")
        assert_eq!(w.finish().expect("encode"), vec!["", "", "0"]);
    }

    #[test]
    fn reader_rejects_non_finite() {
        let fields = &["NaN", "inf", "-inf", "12.5"];
        let mut r = FieldReader::new(fields);
        assert_eq!(r.f32(), None); // "NaN"
        assert_eq!(r.f64(), None); // "inf"
        assert_eq!(r.f64(), None); // "-inf"
        assert_eq!(r.f32(), Some(12.5)); // finite still parses
    }

    #[test]
    fn writer_lat_lon_zero_pads_degrees() {
        let mut w = FieldWriter::new();
        w.lon(Some(454.5784));   // 004°54.5784' → 5-digit integer part
        w.lat(Some(837.038));    // 08°37.038'   → 4-digit integer part
        w.lon(Some(1131.0));     // 011°31'      → no fractional part
        w.lat(Some(4807.038));   // already 4 digits, unchanged
        w.lat(None);             // empty field
        let fields = w.finish().expect("encode");
        assert_eq!(fields, vec!["00454.5784", "0837.038", "01131.0", "4807.038", ""]);
    }

    #[test]
    fn writer_coord_integer_emits_decimal_point() {
        let mut w = FieldWriter::new();
        w.lat(Some(0.0));       // equator  → "0000.0"
        w.lon(Some(0.0));       // meridian → "00000.0"
        w.lat(Some(1131.0));    // 11°31.0' → "1131.0"
        w.lon(Some(1131.0));    // → "01131.0"
        assert_eq!(
            w.finish().expect("encode"),
            vec!["0000.0", "00000.0", "1131.0", "01131.0"]
        );
    }

    #[test]
    fn writer_coord_rejects_nan_and_negative() {
        let mut w = FieldWriter::new();
        w.lat(Some(f64::NAN));
        assert_eq!(w.finish(), Err(EncodeError::InvalidCoordinate));

        let mut w = FieldWriter::new();
        w.lon(Some(f64::INFINITY));
        assert_eq!(w.finish(), Err(EncodeError::InvalidCoordinate));

        let mut w = FieldWriter::new();
        w.lat(Some(-4807.038));
        assert_eq!(w.finish(), Err(EncodeError::InvalidCoordinate));
    }

    #[test]
    fn writer_coord_normalizes_neg_zero() {
        let mut w = FieldWriter::new();
        w.lat(Some(-0.0));
        assert_eq!(w.finish().expect("encode"), vec!["0000.0"]);
    }

    #[test]
    fn writer_string_rejects_delimiters() {
        let mut w = FieldWriter::new();
        w.string(Some("a,b"));
        assert_eq!(w.finish(), Err(EncodeError::InvalidFieldCharacter(',')));

        let mut w = FieldWriter::new();
        w.string(Some("line\r\nbreak"));
        assert_eq!(w.finish(), Err(EncodeError::InvalidFieldCharacter('\r')));

        let mut w = FieldWriter::new();
        w.string(Some("caf\u{e9}")); // non-ASCII
        assert!(matches!(
            w.finish(),
            Err(EncodeError::InvalidFieldCharacter(_))
        ));
    }
}
