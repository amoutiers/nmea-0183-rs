use crate::nmea::field::{FieldWriter, NmeaEncodable};

/// A single transducer measurement group within an XDR sentence.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct XdrGroup {
    /// Transducer type code.
    ///
    /// | Code | Type |
    /// |------|------|
    /// | `A`  | Angular displacement (pitch, roll, yaw) |
    /// | `B`  | Absolute humidity |
    /// | `C`  | Temperature |
    /// | `D`  | Depth |
    /// | `F`  | Frequency |
    /// | `G`  | Generic (e.g. magnetic field components) |
    /// | `H`  | Relative humidity |
    /// | `I`  | Current (amps) |
    /// | `L`  | Salinity |
    /// | `N`  | Force (newtons) |
    /// | `P`  | Pressure |
    /// | `R`  | Flow rate |
    /// | `S`  | Switch / valve state |
    /// | `T`  | Tachometer (RPM) |
    /// | `U`  | Voltage |
    /// | `V`  | Volume (tank levels) |
    pub sensor_type: Option<char>,
    /// Measurement value. May be negative (e.g. roll, temperature offsets).
    pub value: Option<f32>,
    /// Unit of measurement code. `None` when the field is empty (valid for some sensor types
    /// such as `G` — generic magnetic field components).
    ///
    /// | Code | Unit |
    /// |------|------|
    /// | `A`  | Amperes |
    /// | `B`  | Bars (pressure) or binary |
    /// | `C`  | Celsius |
    /// | `D`  | Degrees |
    /// | `E`  | Percentage / capacity (NMEA v4.11+) |
    /// | `H`  | Hertz |
    /// | `I`  | Litres per second |
    /// | `K`  | Kelvin or kg/m³ |
    /// | `M`  | Metres or cubic metres |
    /// | `N`  | Newton |
    /// | `P`  | Pascal or percent |
    /// | `R`  | RPM |
    /// | `S`  | Parts per thousand (salinity) |
    /// | `V`  | Volts |
    pub unit: Option<char>,
    /// Transducer name or identifier.
    ///
    /// No standardised naming before NMEA 0183 v4.11. Since v4.11, convention is
    /// `PARAMETER#INSTANCE` (e.g. `ENGINE#0`, `BATTERY#0`, `FUEL#0`).
    /// Older devices use vendor-specific names such as `WTHI`, `XDHI`, `Barometer`.
    pub name: Option<String>,
}

/// XDR — Transducer Measurement.
///
/// Wire: repeating 4-field groups: `type,value,unit,name[,type,value,unit,name,…]`
///
/// The sentence carries any number of complete 4-field groups. The NMEA 0183 standard
/// imposes no explicit limit on the group count; the 82-character sentence length is the
/// only practical constraint (typically 4–6 groups for real instruments).
/// Incomplete trailing groups (fewer than 4 fields) are silently ignored.
///
/// Empty unit fields are valid and occur in practice (e.g. generic magnetic field sensors
/// emit `G,367,,MAGX` where the unit field is intentionally blank).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Xdr {
    /// Repeating transducer measurement groups (4 fields each).
    pub groups: Vec<XdrGroup>,
}

impl Xdr {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut groups = Vec::new();
        let mut i = 0;
        while i + 4 <= fields.len() {
            let sensor_type = if fields[i].is_empty() {
                None
            } else {
                fields[i].chars().next()
            };
            let value = if fields[i + 1].is_empty() {
                None
            } else {
                fields[i + 1].parse::<f32>().ok().filter(|value| value.is_finite())
            };
            let unit = if fields[i + 2].is_empty() {
                None
            } else {
                fields[i + 2].chars().next()
            };
            let name = if fields[i + 3].is_empty() {
                None
            } else {
                Some(fields[i + 3].to_string())
            };
            groups.push(XdrGroup {
                sensor_type,
                value,
                unit,
                name,
            });
            i += 4;
        }
        Some(Self { groups })
    }

    /// Encode into one or more NMEA sentences, splitting groups across sentences as needed
    /// to stay within the 82-character NMEA 0183 limit.
    ///
    /// Uses a greedy algorithm: fills as many groups as possible into each sentence before
    /// starting the next. A single group is always included per sentence even if its fields
    /// are unusually long. All groups are preserved across the returned sentences.
    pub fn to_sentences(&self, talker: &str) -> Result<Vec<String>, crate::EncodeError> {
        if self.groups.is_empty() {
            return Ok(vec![self.to_sentence(talker)?]);
        }

        let mut sentences = Vec::new();
        let mut start = 0;

        while start < self.groups.len() {
            let mut end = start + 1; // at least 1 group per sentence
            while end < self.groups.len() {
                let probe = Xdr {
                    groups: self.groups[start..end + 1].to_vec(),
                };
                if probe.to_sentence(talker)?.len() > 82 {
                    break;
                }
                end += 1;
            }
            sentences.push(
                Xdr {
                    groups: self.groups[start..end].to_vec(),
                }
                .to_sentence(talker)?,
            );
            start = end;
        }

        Ok(sentences)
    }
}

impl NmeaEncodable for Xdr {
    const SENTENCE_TYPE: &str = "XDR";

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        for g in &self.groups {
            w.char(g.sensor_type);
            w.f32(g.value);
            w.char(g.unit);
            w.string(g.name.as_deref());
        }
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn xdr_barometer() {
        // Typical barometer output: pressure type (P), bars unit (B), long name
        let f = parse_frame("$IIXDR,P,1.01408,B,Barometer*2B").expect("valid XDR barometer");
        let xdr = Xdr::parse(&f.fields).expect("parse XDR");
        assert_eq!(xdr.groups.len(), 1);
        assert_eq!(xdr.groups[0].sensor_type, Some('P'));
        assert!((xdr.groups[0].value.expect("value") - 1.01408).abs() < 0.00001);
        assert_eq!(xdr.groups[0].unit, Some('B'));
        assert_eq!(xdr.groups[0].name, Some("Barometer".to_string()));
    }

    #[test]
    fn xdr_empty() {
        let xdr = Xdr { groups: vec![] };
        let sentence = xdr.to_sentence("II").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("valid");
        let xdr2 = Xdr::parse(&frame.fields).expect("parse");
        assert!(xdr2.groups.is_empty());
    }

    #[test]
    fn xdr_encode_roundtrip() {
        let xdr = Xdr {
            groups: vec![
                XdrGroup {
                    sensor_type: Some('P'),
                    value: Some(1013.25),
                    unit: Some('B'),
                    name: Some("Baro".to_string()),
                },
                XdrGroup {
                    sensor_type: Some('C'),
                    value: Some(25.5),
                    unit: Some('C'),
                    name: Some("Temp".to_string()),
                },
            ],
        };
        let sentence = xdr.to_sentence("WI").expect("encode");
        assert!(sentence.starts_with("$WIXDR,"));
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let xdr2 = Xdr::parse(&frame.fields).expect("re-parse XDR");
        assert_eq!(xdr2.groups.len(), 2);
        assert_eq!(xdr2.groups[0].sensor_type, Some('P'));
        assert!((xdr2.groups[0].value.expect("val") - 1013.25).abs() < 0.01);
        assert_eq!(xdr2.groups[0].name, Some("Baro".to_string()));
        assert_eq!(xdr2.groups[1].sensor_type, Some('C'));
        assert_eq!(xdr2.groups[1].name, Some("Temp".to_string()));
    }

    #[test]
    fn xdr_four_measurements_gonmea() {
        // Real vessel NMEA bus: temperature + 3× voltage sensors
        let f = parse_frame("$WIXDR,C,9.7,C,2,U,24.1,N,0,U,24.4,V,1,U,3.510,V,2*46")
            .expect("valid XDR from go-nmea");
        let xdr = Xdr::parse(&f.fields).expect("parse XDR");
        assert_eq!(xdr.groups.len(), 4);
        assert_eq!(xdr.groups[0].sensor_type, Some('C'));
        assert!((xdr.groups[0].value.expect("temp") - 9.7).abs() < 0.01);
        assert_eq!(xdr.groups[0].unit, Some('C'));
        assert_eq!(xdr.groups[0].name, Some("2".to_string()));
        assert_eq!(xdr.groups[1].sensor_type, Some('U'));
        assert!((xdr.groups[1].value.expect("volt") - 24.1).abs() < 0.01);
        assert_eq!(xdr.groups[3].sensor_type, Some('U'));
        assert!((xdr.groups[3].value.expect("volt") - 3.51).abs() < 0.01);
    }

    #[test]
    fn xdr_incomplete_trailing() {
        let xdr = Xdr {
            groups: vec![XdrGroup {
                sensor_type: Some('P'),
                value: Some(1.013),
                unit: Some('B'),
                name: Some("Baro".to_string()),
            }],
        };
        let mut fields = xdr.encode().expect("encode");
        fields.push("C".to_string()); // trailing incomplete field
        let field_refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
        let parsed = Xdr::parse(&field_refs).expect("parse XDR with trailing");
        assert_eq!(parsed.groups.len(), 1);
        assert_eq!(parsed.groups[0].sensor_type, Some('P'));
    }

    #[test]
    fn xdr_multi_group_gonmea() {
        let f = parse_frame(
            "$HCXDR,A,171,D,PITCH,A,-37,D,ROLL,G,367,,MAGX,G,2420,,MAGY,G,-8984,,MAGZ*41",
        )
        .expect("valid XDR from go-nmea");
        let xdr = Xdr::parse(&f.fields).expect("parse XDR");
        assert_eq!(xdr.groups.len(), 5);
        assert_eq!(xdr.groups[0].sensor_type, Some('A'));
        assert!((xdr.groups[0].value.expect("pitch value") - 171.0).abs() < 0.01);
        assert_eq!(xdr.groups[0].unit, Some('D'));
        assert_eq!(xdr.groups[0].name, Some("PITCH".to_string()));
        // negative value (ROLL)
        assert!((xdr.groups[1].value.expect("roll value") - (-37.0)).abs() < 0.01);
        // empty unit (MAGX, MAGY, MAGZ)
        assert!(xdr.groups[2].unit.is_none());
        assert_eq!(xdr.groups[2].name, Some("MAGX".to_string()));
        assert!((xdr.groups[4].value.expect("magz value") - (-8984.0)).abs() < 0.1);
    }

    #[test]
    fn xdr_partial_value() {
        let xdr = Xdr {
            groups: vec![XdrGroup {
                sensor_type: Some('P'),
                value: None,
                unit: Some('B'),
                name: Some("Baro".to_string()),
            }],
        };
        let sentence = xdr.to_sentence("WI").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let xdr2 = Xdr::parse(&frame.fields).expect("re-parse XDR");
        assert_eq!(xdr2.groups.len(), 1);
        assert!(xdr2.groups[0].value.is_none());
        assert_eq!(xdr2.groups[0].unit, Some('B'));
    }

    #[test]
    fn xdr_rejects_non_finite_values() {
        let xdr = Xdr::parse(&["P", "NaN", "B", "Sensor"]).expect("parse");
        assert_eq!(xdr.groups[0].value, None);
        let xdr = Xdr::parse(&["P", "inf", "B", "Sensor"]).expect("parse");
        assert_eq!(xdr.groups[0].value, None);
        let xdr = Xdr::parse(&["P", "1.01408", "B", "Barometer"]).expect("parse");
        assert_eq!(xdr.groups[0].value, Some(1.01408));
    }

    #[test]
    fn xdr_single_gonmea() {
        let f = parse_frame("$SDXDR,C,23.15,C,WTHI*70").expect("valid XDR from go-nmea");
        let xdr = Xdr::parse(&f.fields).expect("parse XDR");
        assert_eq!(xdr.groups.len(), 1);
        assert_eq!(xdr.groups[0].sensor_type, Some('C'));
        assert!((xdr.groups[0].value.expect("value") - 23.15).abs() < 0.01);
        assert_eq!(xdr.groups[0].unit, Some('C'));
        assert_eq!(xdr.groups[0].name, Some("WTHI".to_string()));
    }

    #[test]
    fn xdr_to_sentences_empty() {
        let xdr = Xdr { groups: vec![] };
        let sentences = xdr.to_sentences("II").expect("encode");
        assert_eq!(sentences.len(), 1);
        assert!(sentences[0].starts_with("$IIXDR"));
    }

    #[test]
    fn xdr_to_sentences_fits() {
        // 2 groups — well within 82 chars, should produce a single sentence
        let xdr = Xdr {
            groups: vec![
                XdrGroup {
                    sensor_type: Some('P'),
                    value: Some(1013.25),
                    unit: Some('B'),
                    name: Some("Baro".to_string()),
                },
                XdrGroup {
                    sensor_type: Some('C'),
                    value: Some(22.5),
                    unit: Some('C'),
                    name: Some("Temp".to_string()),
                },
            ],
        };
        let sentences = xdr.to_sentences("WI").expect("encode");
        assert_eq!(sentences.len(), 1);
        assert!(sentences[0].len() <= 82);
    }

    #[test]
    fn xdr_to_sentences_split() {
        // 12 groups with typical names — forces a split into multiple sentences
        let groups: Vec<XdrGroup> = (0..12)
            .map(|i| XdrGroup {
                sensor_type: Some('C'),
                value: Some(20.0 + i as f32),
                unit: Some('C'),
                name: Some(format!("SENSOR{i:02}")),
            })
            .collect();
        let total = groups.len();
        let xdr = Xdr { groups };
        let sentences = xdr.to_sentences("WI").expect("encode");

        // Must have produced more than one sentence
        assert!(
            sentences.len() > 1,
            "expected split, got {} sentence(s)",
            sentences.len()
        );

        // Every sentence must be within 82 chars
        for s in &sentences {
            assert!(
                s.len() <= 82,
                "sentence too long ({} chars): {}",
                s.len(),
                s.trim()
            );
        }

        // All groups must be recoverable in order
        let recovered: Vec<XdrGroup> = sentences
            .iter()
            .flat_map(|s| {
                let frame = crate::parse_frame(s.trim()).expect("re-parse");
                Xdr::parse(&frame.fields).expect("parse").groups
            })
            .collect();
        assert_eq!(recovered.len(), total);
        for (i, g) in recovered.iter().enumerate() {
            assert_eq!(g.name, Some(format!("SENSOR{i:02}")));
        }
    }
}
