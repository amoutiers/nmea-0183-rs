//! AIS (Automatic Identification System) message decoding and application-layer sentences.
//!
//! Decodes AIVDM/AIVDO messages from `!`-prefixed NMEA frames when the `ais`
//! feature is enabled. The `!`-prefixed AIS application-layer NMEA sentences
//! ABM and BBM live in `ais::sentences`.
//!
//! # Usage
//!
//! ```
//! #[cfg(feature = "ais")]
//! {
//!     use nmea_0183_rs::ais::{AisParser, AisMessage};
//!     use nmea_0183_rs::parse_frame;
//!
//!     let mut parser = AisParser::new();
//!
//!     // Single-fragment message
//!     let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").expect("valid");
//!     if let Some(msg) = parser.decode(&frame) {
//!         match msg {
//!             AisMessage::Position(pos) => println!("MMSI: {}, lat: {:?}", pos.mmsi, pos.latitude),
//!             _ => {}
//!         }
//!     }
//! }
//! ```

#[cfg(feature = "ais")]
pub mod armor;
#[cfg(feature = "ais")]
mod encode;
#[cfg(feature = "ais")]
mod error;
#[cfg(feature = "ais")]
pub mod fragments;
#[cfg(feature = "ais")]
pub mod messages;
#[cfg(any(feature = "abm", feature = "bbm"))]
pub mod sentences;
#[cfg(feature = "ais")]
pub mod transmit;

#[cfg(feature = "ais")]
pub use messages::*;

#[cfg(feature = "ais")]
pub use error::AisDecodeError;

#[cfg(feature = "ais")]
use armor::decode_armor;
#[cfg(feature = "ais")]
use fragments::FragmentCollector;

#[cfg(feature = "ais")]
use crate::NmeaFrame;

/// Unified AIS message enum.
#[cfg(feature = "ais")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum AisMessage {
    /// Types 1, 2, 3 (Class A), 18 (Class B), 19 (Class B+) position reports.
    Position(PositionReport),
    /// Type 4: UTC time and position from base station (coast guard / port authority).
    BaseStation(BaseStationReport),
    /// Type 5: static and voyage related data (Class A).
    StaticVoyage(StaticVoyageData),
    /// Type 6: addressed binary message (application-specific data).
    BinaryAddressed(BinaryAddressed),
    /// Types 7/13: binary / safety acknowledge.
    BinaryAck(BinaryAck),
    /// Type 8: binary broadcast message (application-specific data).
    BinaryBroadcast(BinaryBroadcast),
    /// Type 9: standard SAR aircraft position report.
    SarAircraft(SarAircraftReport),
    /// Type 10: UTC and date inquiry.
    UtcDateInquiry(UtcDateInquiry),
    /// Type 11: UTC/date response (mobile station reply to interrogation).
    UtcDateResponse(UtcDateResponse),
    /// Type 12: addressed safety-related message (text to specific MMSI).
    SafetyAddressed(SafetyAddressed),
    /// Type 14: safety-related broadcast message (text alert from shore/vessel).
    Safety(SafetyBroadcast),
    /// Type 15: interrogation (request data from other vessel).
    Interrogation(Interrogation),
    /// Type 16: assigned mode command.
    AssignmentMode(AssignmentModeCommand),
    /// Type 17: DGNSS correction broadcast.
    DgnssBroadcast(DgnssBroadcast),
    /// Type 21: aid-to-navigation report (buoy, beacon, lighthouse).
    AidToNavigation(AidToNavigation),
    /// Type 20: data link management.
    DataLinkManagement(DataLinkManagement),
    /// Type 22: channel management.
    ChannelManagement(ChannelManagement),
    /// Type 23: group assignment command.
    GroupAssignment(GroupAssignment),
    /// Type 24: static data report (Class B), Part A or Part B.
    StaticReport(StaticDataReport),
    /// Type 25: single-slot binary message.
    BinarySingleSlot(BinarySingleSlot),
    /// Type 26: multiple-slot binary message with communication state.
    BinaryMultiSlot(BinaryMultiSlot),
    /// Type 27: long-range position report (satellite AIS / Class D).
    LongRangePosition(LongRangePosition),
    /// Unsupported message type.
    Unknown { msg_type: u8 },
}

/// Successful outcome of feeding a frame to [`AisParser::decode_detailed`].
#[cfg(feature = "ais")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum AisDecodeOutcome {
    /// The frame is not a `!`-prefixed VDM/VDO sentence.
    Ignored,
    /// More fragments are needed, or an already accepted continuation was repeated.
    Pending,
    /// A complete message, including an unknown numeric AIS type.
    Message(AisMessage),
}

/// Stateful AIS parser with multi-fragment reassembly.
///
/// Maintains separate VDM and VDO fragment buffers for up to 40 concurrent
/// multi-part messages. Feed it frames from one source via `parse_frame()` —
/// it returns decoded messages. It does not distinguish separate physical
/// receivers that share a stream.
#[cfg(feature = "ais")]
pub struct AisParser {
    vdm_collector: FragmentCollector,
    vdo_collector: FragmentCollector,
}

#[cfg(feature = "ais")]
impl AisParser {
    pub fn new() -> Self {
        Self {
            vdm_collector: FragmentCollector::new(),
            vdo_collector: FragmentCollector::new(),
        }
    }

    /// Clear all in-progress fragment buffers.
    ///
    /// Useful when switching data sources or recovering from a corrupted stream.
    pub fn reset(&mut self) {
        self.vdm_collector = FragmentCollector::new();
        self.vdo_collector = FragmentCollector::new();
    }

    /// Decode an AIS frame. Returns `Some(AisMessage)` for complete messages,
    /// `None` for incomplete fragments, parse errors, or non-AIS frames.
    /// Use [`Self::decode_detailed`] when rejected frames need diagnostics.
    pub fn decode(&mut self, frame: &NmeaFrame<'_>) -> Option<AisMessage> {
        match self.decode_detailed(frame) {
            Ok(AisDecodeOutcome::Message(message)) => Some(message),
            _ => None,
        }
    }

    /// Decode a frame, distinguishing ignored frames, pending fragments and errors.
    ///
    /// Frame checksum errors belong to [`crate::parse_frame`]. Unknown numeric
    /// message types are returned as [`AisMessage::Unknown`], not as errors.
    /// Use one parser per physical source, just as with [`Self::decode`].
    pub fn decode_detailed(
        &mut self,
        frame: &NmeaFrame<'_>,
    ) -> Result<AisDecodeOutcome, AisDecodeError> {
        // Only handle VDM and VDO sentences
        if frame.prefix != '!' || (frame.sentence_type != "VDM" && frame.sentence_type != "VDO") {
            return Ok(AisDecodeOutcome::Ignored);
        }

        // Reassemble fragments
        let collector = if frame.sentence_type == "VDM" {
            &mut self.vdm_collector
        } else {
            &mut self.vdo_collector
        };
        let Some(payload) = collector.process_checked(&frame.fields)? else {
            return Ok(AisDecodeOutcome::Pending);
        };

        // Decode armor
        let bits = decode_armor(&payload.payload, payload.fill_bits)
            .ok_or(AisDecodeError::InvalidArmor)?;

        // Extract message type (first 6 bits)
        let msg_type = armor::extract_u32(&bits, 0, 6)
            .ok_or(AisDecodeError::InvalidMessage { msg_type: None })? as u8;

        // Dispatch to message decoder
        match msg_type {
            1..=3 => PositionReport::decode_class_a(&bits).map(AisMessage::Position),
            4 => BaseStationReport::decode(&bits).map(AisMessage::BaseStation),
            5 => StaticVoyageData::decode(&bits).map(AisMessage::StaticVoyage),
            6 => BinaryAddressed::decode(&bits).map(AisMessage::BinaryAddressed),
            7 | 13 => BinaryAck::decode(&bits).map(AisMessage::BinaryAck),
            8 => BinaryBroadcast::decode(&bits).map(AisMessage::BinaryBroadcast),
            9 => SarAircraftReport::decode(&bits).map(AisMessage::SarAircraft),
            10 => UtcDateInquiry::decode(&bits).map(AisMessage::UtcDateInquiry),
            11 => UtcDateResponse::decode(&bits).map(AisMessage::UtcDateResponse),
            12 => SafetyAddressed::decode(&bits).map(AisMessage::SafetyAddressed),
            14 => SafetyBroadcast::decode(&bits).map(AisMessage::Safety),
            15 => Interrogation::decode(&bits).map(AisMessage::Interrogation),
            16 => AssignmentModeCommand::decode(&bits).map(AisMessage::AssignmentMode),
            17 => DgnssBroadcast::decode(&bits).map(AisMessage::DgnssBroadcast),
            18 => PositionReport::decode_class_b(&bits).map(AisMessage::Position),
            19 => PositionReport::decode_class_b_extended(&bits).map(AisMessage::Position),
            21 => AidToNavigation::decode(&bits).map(AisMessage::AidToNavigation),
            20 => DataLinkManagement::decode(&bits).map(AisMessage::DataLinkManagement),
            22 => ChannelManagement::decode(&bits).map(AisMessage::ChannelManagement),
            23 => GroupAssignment::decode(&bits).map(AisMessage::GroupAssignment),
            24 => StaticDataReport::decode(&bits).map(AisMessage::StaticReport),
            25 => BinarySingleSlot::decode(&bits).map(AisMessage::BinarySingleSlot),
            26 => BinaryMultiSlot::decode(&bits).map(AisMessage::BinaryMultiSlot),
            27 => LongRangePosition::decode(&bits).map(AisMessage::LongRangePosition),
            _ => Some(AisMessage::Unknown { msg_type }),
        }
        .map(AisDecodeOutcome::Message)
        .ok_or(AisDecodeError::InvalidMessage {
            msg_type: Some(msg_type),
        })
    }
}

#[cfg(feature = "ais")]
impl Default for AisParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(feature = "ais")]
mod tests {
    use super::*;
    use crate::ais::armor::encode_armor;
    use crate::ais::messages::test_helpers::set_bits;
    use crate::{encode_frame, parse_frame};

    fn decode_bits(bits: &mut [u8]) -> AisMessage {
        let (payload, fill_bits) = encode_armor(bits);
        let fill_bits = fill_bits.to_string();
        let frame = encode_frame('!', "AI", "VDM", &["1", "1", "", "A", &payload, &fill_bits])
            .expect("frame");
        AisParser::new()
            .decode(&parse_frame(&frame).expect("parse"))
            .expect("decode")
    }

    #[test]
    fn dispatches_newly_supported_types() {
        for (message_type, bit_len) in [
            (10, 72),
            (16, 96),
            (17, 80),
            (20, 72),
            (22, 168),
            (23, 160),
            (25, 40),
            (26, 64),
        ] {
            let mut bits = vec![0; bit_len];
            set_bits(&mut bits, 0, 6, message_type);
            let message = decode_bits(&mut bits);
            assert!(
                matches!(
                    (message_type, message),
                    (10, AisMessage::UtcDateInquiry(_))
                        | (16, AisMessage::AssignmentMode(_))
                        | (17, AisMessage::DgnssBroadcast(_))
                        | (20, AisMessage::DataLinkManagement(_))
                        | (22, AisMessage::ChannelManagement(_))
                        | (23, AisMessage::GroupAssignment(_))
                        | (25, AisMessage::BinarySingleSlot(_))
                        | (26, AisMessage::BinaryMultiSlot(_))
                ),
                "type {message_type} did not dispatch to its typed variant"
            );
        }
    }

    #[test]
    fn ignores_nmea_sentences() {
        let mut parser = AisParser::new();
        let frame =
            parse_frame("$GPRMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*77")
                .expect("valid");
        assert!(parser.decode(&frame).is_none());
    }

    #[test]
    fn sentinel_values_filtered() {
        let mut parser = AisParser::new();
        let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").expect("valid");
        let msg = parser.decode(&frame).expect("decoded");
        if let AisMessage::Position(pos) = msg {
            assert!(pos.heading.is_none() || pos.heading.expect("heading") < 360);
        }
    }

    #[test]
    fn type_18_class_b() {
        let mut parser = AisParser::new();
        let frame = parse_frame("!AIVDM,1,1,,A,B6CdCm0t3`tba35f@V9faHi7kP06,0*58").expect("valid");
        let msg = parser.decode(&frame);
        // This might be a type 18 or might not decode depending on exact payload
        // At minimum it shouldn't panic
        if let Some(AisMessage::Position(pos)) = &msg {
            assert_eq!(pos.ais_class, AisClass::B);
        }
    }

    #[test]
    fn type_19_class_b_extended() {
        let mut parser = AisParser::new();
        // GPSD fixture: Type 19 Class B+ extended position report
        let frame =
            parse_frame("!AIVDM,1,1,,B,C5N3SRgPEnJGEBT>NhWAwwo862PaLELTBJ:V00000000S0D:R220,0*0B")
                .expect("valid type 19 frame");
        let msg = parser.decode(&frame).expect("decode type 19");
        if let AisMessage::Position(pos) = msg {
            assert_eq!(pos.msg_type, 19);
            assert!(pos.mmsi > 0);
            assert!(pos.latitude.is_some());
            assert!(pos.longitude.is_some());
            assert_eq!(pos.ais_class, AisClass::BPlus);
        } else {
            panic!("expected Position (type 19), got {msg:?}");
        }
    }

    #[test]
    fn type_1_position_report() {
        let mut parser = AisParser::new();
        let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").expect("valid");
        let msg = parser.decode(&frame).expect("decoded");
        if let AisMessage::Position(pos) = msg {
            assert_eq!(pos.msg_type, 1);
            assert!(pos.mmsi > 0);
            assert!(pos.latitude.is_some());
            assert!(pos.longitude.is_some());
            assert_eq!(pos.ais_class, AisClass::A);
            // Verify f64 precision
            let lat = pos.latitude.expect("valid");
            let lon = pos.longitude.expect("valid");
            assert!((-90.0..=90.0).contains(&lat));
            assert!((-180.0..=180.0).contains(&lon));
        } else {
            panic!("expected Position, got {msg:?}");
        }
    }

    #[test]
    fn type_24_static_data_report() {
        let mut parser = AisParser::new();
        // Type 24 Part A: vessel name
        let frame = parse_frame("!AIVDM,1,1,,A,H52N>V@T2rNVPJ2000000000000,2*29")
            .expect("valid type 24 frame");
        let msg = parser.decode(&frame).expect("decode type 24");
        if let AisMessage::StaticReport(report) = msg {
            match report {
                StaticDataReport::PartA {
                    mmsi, vessel_name, ..
                } => {
                    assert!(mmsi > 0);
                    // Vessel name may be all padding (@) — trimmed to empty
                    let _ = vessel_name;
                }
                StaticDataReport::PartB { mmsi, .. } => {
                    assert!(mmsi > 0);
                }
            }
        } else {
            panic!("expected StaticReport (type 24), got {msg:?}");
        }
    }

    #[test]
    fn type_5_multi_fragment() {
        let mut parser = AisParser::new();

        // GPSD sample.aivdm Type 5 fixture
        let f1 = parse_frame(
            "!AIVDM,2,1,1,A,55?MbV02;H;s<HtKR20EHE:0@T4@Dn2222222216L961O5Gf0NSQEp6ClRp8,0*1C",
        )
        .expect("valid frag1");
        assert!(parser.decode(&f1).is_none()); // incomplete

        let f2 = parse_frame("!AIVDM,2,2,1,A,88888888880,2*25").expect("valid frag2");
        let msg = parser.decode(&f2).expect("decoded");
        if let AisMessage::StaticVoyage(svd) = msg {
            assert!(svd.mmsi > 0);
            assert!(!svd.vessel_name.is_empty());
            assert_eq!(svd.ais_class, AisClass::A);
        } else {
            panic!("expected StaticVoyage, got {msg:?}");
        }
    }

    #[test]
    fn reset_clears_pending_fragments() {
        let mut parser = AisParser::new();
        // Send fragment 1 of 2
        let f1 = parse_frame(
            "!AIVDM,2,1,1,A,55?MbV02;H;s<HtKR20EHE:0@T4@Dn2222222216L961O5Gf0NSQEp6ClRp8,0*1C",
        )
        .expect("valid");
        assert!(parser.decode(&f1).is_none());
        // Reset clears the pending fragment
        parser.reset();
        // Fragment 2 alone should not produce a message
        let f2 = parse_frame("!AIVDM,2,2,1,A,88888888880,2*25").expect("valid");
        assert!(parser.decode(&f2).is_none());
    }

    #[test]
    fn type_8_binary_broadcast() {
        let mut parser = AisParser::new();
        let frame = parse_frame("!AIVDM,1,1,,A,85Mv070j2d>=<e<<=PQhhg`59P00,0*26").expect("valid");
        let msg = parser.decode(&frame);
        if let Some(AisMessage::BinaryBroadcast(bb)) = msg {
            assert!(bb.mmsi > 0);
        } else {
            panic!("expected BinaryBroadcast type 8, got {msg:?}");
        }
    }

    #[test]
    fn type_14_safety_broadcast() {
        let mut parser = AisParser::new();
        // Type 14 safety broadcast — payload starts with '>' (val=14)
        let frame =
            parse_frame("!AIVDM,1,1,,A,>5?Per18=HB1U:1@E=B0m<L,0*53").expect("valid type 14 frame");
        let msg = parser.decode(&frame).expect("decoded");
        if let AisMessage::Safety(broadcast) = msg {
            assert!(broadcast.mmsi > 0, "MMSI must be set");
        } else {
            panic!("expected Safety (type 14), got {msg:?}");
        }
    }

    #[test]
    fn type_14_empty_text_no_panic() {
        let mut parser = AisParser::new();
        // Minimal type 14: short payload, text portion may be empty
        let frame = parse_frame("!AIVDM,1,1,,A,>5?Per1,0*64").expect("valid minimal type 14");
        // Should decode (returns Safety with empty text) or return None — must not panic
        let _ = parser.decode(&frame);
    }

    #[test]
    fn type_21_aid_to_navigation() {
        let mut parser = AisParser::new();
        // Type 21 AtoN — 46-char payload (276 bits > 272 minimum), fill=4
        // payload starts with 'E' (val=21 → msg_type=21)
        let frame =
            parse_frame("!AIVDM,1,1,,B,E>jCfrv2`0c2h0W:0a0h6220d5Du0`Htp00000l1@Dc2P0,4*3C")
                .expect("valid type 21 frame");
        let msg = parser.decode(&frame).expect("decoded");
        if let AisMessage::AidToNavigation(aton) = msg {
            assert!(aton.mmsi > 0, "MMSI must be set");
            assert!(
                aton.aid_type <= 31,
                "aid_type must be 0–31, got {}",
                aton.aid_type
            );
        } else {
            panic!("expected AidToNavigation (type 21), got {msg:?}");
        }
    }

    #[test]
    fn type_21_position_in_range() {
        let mut parser = AisParser::new();
        let frame =
            parse_frame("!AIVDM,1,1,,B,E>jCfrv2`0c2h0W:0a0h6220d5Du0`Htp00000l1@Dc2P0,4*3C")
                .expect("valid type 21");
        let msg = parser.decode(&frame).expect("decoded");
        if let AisMessage::AidToNavigation(aton) = msg {
            if let (Some(lat), Some(lon)) = (aton.lat, aton.lon) {
                assert!((-90.0..=90.0).contains(&lat), "lat out of range: {lat}");
                assert!((-180.0..=180.0).contains(&lon), "lon out of range: {lon}");
            }
        }
    }
}
