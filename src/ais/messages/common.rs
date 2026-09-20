//! Common AIS navigation, transceiver class and timestamp types.

/// Timestamp status carried by AIS position reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionTimestamp {
    /// UTC second at which the report was generated (0-59).
    Exact(u8),
    /// UTC time is not available.
    NotAvailable,
    /// Position was entered manually.
    ManualInput,
    /// Position comes from estimated or dead-reckoning navigation.
    DeadReckoning,
    /// The electronic position-fixing system is inoperative.
    Inoperative,
}

impl PositionTimestamp {
    /// Decode an already-extracted six-bit AIS timestamp.
    pub(crate) fn from_six_bits(value: u8) -> Self {
        match value {
            60 => Self::NotAvailable,
            61 => Self::ManualInput,
            62 => Self::DeadReckoning,
            63 => Self::Inoperative,
            second => Self::Exact(second),
        }
    }
}

/// AIS Navigation Status (4 bits, 0-15).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationStatus {
    UnderWayEngine,
    AtAnchor,
    NotUnderCommand,
    RestrictedManeuverability,
    ConstrainedByDraught,
    Moored,
    Aground,
    EngagedInFishing,
    UnderWaySailing,
    ReservedHsc,
    ReservedWig,
    Reserved(u8),
    AisSartActive,
    Undefined,
}

impl From<u8> for NavigationStatus {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::UnderWayEngine,
            1 => Self::AtAnchor,
            2 => Self::NotUnderCommand,
            3 => Self::RestrictedManeuverability,
            4 => Self::ConstrainedByDraught,
            5 => Self::Moored,
            6 => Self::Aground,
            7 => Self::EngagedInFishing,
            8 => Self::UnderWaySailing,
            9 => Self::ReservedHsc,
            10 => Self::ReservedWig,
            14 => Self::AisSartActive,
            15 => Self::Undefined,
            other => Self::Reserved(other),
        }
    }
}

impl From<NavigationStatus> for u8 {
    fn from(val: NavigationStatus) -> Self {
        match val {
            NavigationStatus::UnderWayEngine => 0,
            NavigationStatus::AtAnchor => 1,
            NavigationStatus::NotUnderCommand => 2,
            NavigationStatus::RestrictedManeuverability => 3,
            NavigationStatus::ConstrainedByDraught => 4,
            NavigationStatus::Moored => 5,
            NavigationStatus::Aground => 6,
            NavigationStatus::EngagedInFishing => 7,
            NavigationStatus::UnderWaySailing => 8,
            NavigationStatus::ReservedHsc => 9,
            NavigationStatus::ReservedWig => 10,
            NavigationStatus::AisSartActive => 14,
            NavigationStatus::Undefined => 15,
            NavigationStatus::Reserved(v) => v,
        }
    }
}

/// AIS transceiver class, inferred from message type.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AisClass {
    /// Class A — SOLAS vessels (Types 1/2/3/5)
    A,
    /// Class B position reports (Type 18).
    /// The `class_b_cs` flag distinguishes CS from SO equipment.
    B,
    /// Extended Class B position reports (Type 19, SO equipment).
    BPlus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_status_known_values() {
        assert_eq!(NavigationStatus::from(0), NavigationStatus::UnderWayEngine);
        assert_eq!(NavigationStatus::from(1), NavigationStatus::AtAnchor);
        assert_eq!(NavigationStatus::from(5), NavigationStatus::Moored);
        assert_eq!(NavigationStatus::from(8), NavigationStatus::UnderWaySailing);
        assert_eq!(u8::from(NavigationStatus::Moored), 5);
    }

    #[test]
    fn nav_status_roundtrip() {
        for val in 0..=15u8 {
            let status = NavigationStatus::from(val);
            let back: u8 = status.into();
            assert_eq!(val, back);
        }
    }
}

#[cfg(test)]
mod timestamp_tests {
    use super::PositionTimestamp;
    use crate::ais::messages::{
        AidToNavigation, PositionReport, SarAircraftReport, test_helpers::set_bits,
    };

    #[test]
    fn position_timestamp_states_survive_all_decoders() {
        for (raw, expected) in [
            (0, PositionTimestamp::Exact(0)),
            (59, PositionTimestamp::Exact(59)),
            (60, PositionTimestamp::NotAvailable),
            (61, PositionTimestamp::ManualInput),
            (62, PositionTimestamp::DeadReckoning),
            (63, PositionTimestamp::Inoperative),
        ] {
            for (kind, length, offset) in [
                (1, 168, 137),
                (2, 168, 137),
                (3, 168, 137),
                (9, 168, 128),
                (18, 168, 133),
                (19, 312, 133),
                (21, 272, 253),
            ] {
                let mut bits = vec![0; length];
                set_bits(&mut bits, 0, 6, kind);
                set_bits(&mut bits, offset, 6, raw);
                let actual = match kind {
                    1..=3 => {
                        PositionReport::decode_class_a(&bits)
                            .expect("class A")
                            .timestamp
                    }
                    9 => {
                        SarAircraftReport::decode(&bits)
                            .expect("aircraft")
                            .timestamp
                    }
                    18 => {
                        PositionReport::decode_class_b(&bits)
                            .expect("class B")
                            .timestamp
                    }
                    19 => {
                        PositionReport::decode_class_b_extended(&bits)
                            .expect("extended B")
                            .timestamp
                    }
                    _ => AidToNavigation::decode(&bits).expect("aid").timestamp,
                };
                assert_eq!(actual, expected, "type {kind}, wire timestamp {raw}");
            }
        }
    }
}
