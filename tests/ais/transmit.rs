use super::expect_message;
use nmea_0183_rs::ais::AisDecodeOutcome;
use nmea_0183_rs::ais::armor::extract_u32;
use nmea_0183_rs::ais::messages::NavigationStatus;
use nmea_0183_rs::ais::messages::PositionTimestamp;
use nmea_0183_rs::ais::transmit::{
    AidToNavigation, AisChannel, AisEncodable, AisTransmitOptions, BaseStation, ClassAPosition,
    ClassAPositionType, ClassAStaticVoyage, ClassBCommunicationState, ClassBExtendedPosition,
    ClassBPosition, ClassBStaticPartA, ClassBStaticPartB, LongRangePosition, SafetyAddressed,
    SafetyBroadcast, SarAircraft, UtcDateResponse,
};
use nmea_0183_rs::ais::{AisMessage, AisParser};
use nmea_0183_rs::{EncodeError, parse_frame};

fn decode_lines(lines: &[String]) -> AisMessage {
    let mut parser = AisParser::new();
    let mut decoded = None;
    for line in lines {
        match parser
            .decode(&parse_frame(line).expect("parse encoded AIS sentence"))
            .expect("valid AIS content")
        {
            AisDecodeOutcome::Pending => {}
            AisDecodeOutcome::Message(message) => decoded = Some(message),
            other => panic!("unexpected AIS outcome: {other:?}"),
        }
    }
    decoded.expect("decode encoded AIS message")
}

fn payload_bits(line: &str) -> Vec<u8> {
    let frame = parse_frame(line).expect("parse encoded AIS sentence");
    let payload = frame.fields[4];
    let fill_bits = frame.fields[5].parse::<u8>().expect("parse fill bits");
    // Independent AIS six-bit alphabet oracle, MSB first.
    let mut bits: Vec<u8> = payload
        .bytes()
        .flat_map(|byte| {
            let value = if byte < 88 { byte - 48 } else { byte - 56 };
            (0..6).rev().map(move |shift| (value >> shift) & 1)
        })
        .collect();
    bits.truncate(bits.len() - usize::from(fill_bits));
    bits
}

fn check_timestamp_roundtrip(
    report: &impl AisEncodable,
    raw: u32,
    expected: PositionTimestamp,
    offset: usize,
) {
    let lines = report
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1))
        .expect("timestamp encode");
    assert_eq!(extract_u32(&payload_bits(&lines[0]), offset, 6), Some(raw));
    let actual = match decode_lines(&lines) {
        AisMessage::Position(value) => value.timestamp,
        AisMessage::SarAircraft(value) => value.timestamp,
        AisMessage::AidToNavigation(value) => value.timestamp,
        other => panic!("unexpected timestamp message: {other:?}"),
    };
    let expected: nmea_0183_rs::ais::messages::common::PositionTimestamp = expected;
    assert_eq!(actual, expected);
}

const TIMESTAMP_CASES: [(u32, PositionTimestamp); 6] = [
    (0, PositionTimestamp::Exact(0)),
    (59, PositionTimestamp::Exact(59)),
    (60, PositionTimestamp::NotAvailable),
    (61, PositionTimestamp::ManualInput),
    (62, PositionTimestamp::DeadReckoning),
    (63, PositionTimestamp::Inoperative),
];

#[test]
fn safety_messages_encode_to_decodable_sentences() {
    let addressed = SafetyAddressed {
        repeat_indicator: 0,
        mmsi: 244_670_316,
        sequence: 2,
        destination_mmsi: 235_009_217,
        retransmit: false,
        text: "KEEP CLEAR".to_string(),
    };
    let broadcast = SafetyBroadcast {
        repeat_indicator: 0,
        mmsi: addressed.mmsi,
        text: "MAYDAY TEST".to_string(),
    };

    let addressed_lines = addressed
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
        .expect("encode addressed safety");
    let broadcast_lines = broadcast
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
        .expect("encode broadcast safety");

    assert!(matches!(
        decode_lines(&addressed_lines),
        AisMessage::SafetyAddressed(value)
            if value.mmsi == addressed.mmsi
                && value.dest_mmsi == addressed.destination_mmsi
                && value.sequence == addressed.sequence
                && value.text == addressed.text
    ));
    assert!(matches!(
        decode_lines(&broadcast_lines),
        AisMessage::Safety(value) if value.mmsi == broadcast.mmsi && value.text == broadcast.text
    ));
}

#[test]
fn safety_broadcast_fragments_and_preserves_terminal_at_bit() {
    let broadcast = SafetyBroadcast {
        repeat_indicator: 0,
        mmsi: 244_670_316,
        text: format!("{}@", "A".repeat(160)),
    };
    assert_eq!(
        broadcast.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        Err(EncodeError::MissingAisSequenceId)
    );
    let lines = broadcast
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(6))
        .expect("encode fragmented safety broadcast");
    assert_eq!(lines.len(), 3);
    let last_bits = payload_bits(lines.last().expect("last fragment"));
    assert_eq!(extract_u32(&last_bits, last_bits.len() - 6, 6), Some(0));
}

#[test]
fn specialized_station_reports_encode_to_decodable_sentences() {
    let base = BaseStation {
        repeat_indicator: 0,
        mmsi: 111_222_333,
        year: Some(2026),
        month: Some(7),
        day: Some(28),
        hour: Some(12),
        minute: Some(30),
        second: Some(15),
        position_accuracy: true,
        longitude: Some(2.352_2),
        latitude: Some(48.856_6),
        position_fixing_device: 1,
        transmission_control: false,
        raim: true,
        communication_state: 42,
    };
    let utc = UtcDateResponse {
        repeat_indicator: base.repeat_indicator,
        mmsi: base.mmsi,
        year: base.year,
        month: base.month,
        day: base.day,
        hour: base.hour,
        minute: base.minute,
        second: base.second,
        position_accuracy: base.position_accuracy,
        longitude: base.longitude,
        latitude: base.latitude,
        position_fixing_device: base.position_fixing_device,
        transmission_control: base.transmission_control,
        raim: base.raim,
        communication_state: base.communication_state,
    };
    let sar = SarAircraft {
        repeat_indicator: 0,
        mmsi: 111_222_334,
        altitude: Some(300),
        sog: Some(120),
        position_accuracy: true,
        longitude: Some(2.352_2),
        latitude: Some(48.856_6),
        cog: Some(91.2),
        timestamp: PositionTimestamp::DeadReckoning,
        regional_application: 0,
        dte: false,
        assigned_mode: false,
        raim: false,
        communication_state: ClassBCommunicationState::Sotdma(0),
    };
    for (raw, timestamp) in TIMESTAMP_CASES {
        let mut candidate = sar.clone();
        candidate.timestamp = timestamp;
        check_timestamp_roundtrip(&candidate, raw, timestamp, 128);
    }
    for invalid in [60, 255] {
        let mut candidate = sar.clone();
        candidate.timestamp = PositionTimestamp::Exact(invalid);
        assert_eq!(
            candidate.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
            Err(EncodeError::InvalidAisField("timestamp"))
        );
    }

    let aton = AidToNavigation {
        repeat_indicator: 0,
        mmsi: 992_001_001,
        aid_type: 1,
        name: "TEST BUOY".to_string(),
        longitude: Some(2.352_2),
        latitude: Some(48.856_6),
        position_accuracy: true,
        dimension_to_bow: 2,
        dimension_to_stern: 2,
        dimension_to_port: 1,
        dimension_to_starboard: 1,
        position_fixing_device: 1,
        timestamp: PositionTimestamp::Inoperative,
        off_position: false,
        regional_application: 0,
        raim: false,
        virtual_aid: false,
        assigned_mode: false,
        name_extension: Some("WEST".to_string()),
    };
    for (raw, timestamp) in TIMESTAMP_CASES {
        let mut candidate = aton.clone();
        candidate.timestamp = timestamp;
        check_timestamp_roundtrip(&candidate, raw, timestamp, 253);
    }
    for invalid in [60, 255] {
        let mut candidate = aton.clone();
        candidate.timestamp = PositionTimestamp::Exact(invalid);
        assert_eq!(
            candidate.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
            Err(EncodeError::InvalidAisField("timestamp"))
        );
    }

    for report in [
        base.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        utc.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        sar.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        aton.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
    ] {
        let lines = report.expect("encode specialized station report");
        assert_eq!(lines.len(), 1);
        assert!(parse_frame(&lines[0]).is_ok());
    }

    assert!(matches!(
        decode_lines(
            &base
                .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
                .expect("base")
        ),
        AisMessage::BaseStation(_)
    ));
    assert!(matches!(
        decode_lines(
            &utc.to_sentences(AisTransmitOptions::vdm(AisChannel::A))
                .expect("utc")
        ),
        AisMessage::UtcDateResponse(_)
    ));
    let sar_lines = sar
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
        .expect("sar");
    assert_eq!(extract_u32(&payload_bits(&sar_lines[0]), 128, 6), Some(62));
    let aton_lines = aton
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
        .expect("aton");
    assert_eq!(extract_u32(&payload_bits(&aton_lines[0]), 253, 6), Some(63));
    assert!(matches!(
        decode_lines(&aton_lines),
        AisMessage::AidToNavigation(_)
    ));
}

#[test]
fn specialized_station_reports_reject_reserved_epfd() {
    let report = BaseStation {
        repeat_indicator: 0,
        mmsi: 111_222_333,
        year: None,
        month: None,
        day: None,
        hour: None,
        minute: None,
        second: None,
        position_accuracy: false,
        longitude: None,
        latitude: None,
        position_fixing_device: 10,
        transmission_control: false,
        raim: false,
        communication_state: 0,
    };
    assert_eq!(
        report.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        Err(EncodeError::InvalidAisField("position_fixing_device"))
    );
}

#[test]
fn class_b_extended_and_long_range_reports_encode_to_decodable_sentences() {
    let extended = ClassBExtendedPosition {
        repeat_indicator: 0,
        mmsi: 235_009_217,
        sog: Some(5.4),
        position_accuracy: true,
        longitude: Some(2.352_2),
        latitude: Some(48.856_6),
        cog: Some(91.2),
        heading: Some(91),
        timestamp: PositionTimestamp::ManualInput,
        vessel_name: "SIM BOAT".to_string(),
        ship_type: 37,
        dimension_to_bow: 8,
        dimension_to_stern: 4,
        dimension_to_port: 2,
        dimension_to_starboard: 2,
        position_fixing_device: 1,
        dte: false,
        assigned_mode: false,
        raim: true,
    };
    for (raw, timestamp) in TIMESTAMP_CASES {
        let mut candidate = extended.clone();
        candidate.timestamp = timestamp;
        check_timestamp_roundtrip(&candidate, raw, timestamp, 133);
    }
    for invalid in [60, 255] {
        let mut candidate = extended.clone();
        candidate.timestamp = PositionTimestamp::Exact(invalid);
        assert_eq!(
            candidate.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
            Err(EncodeError::InvalidAisField("timestamp"))
        );
    }

    let long_range = LongRangePosition {
        mmsi: extended.mmsi,
        position_accuracy: true,
        raim: false,
        navigation_status: NavigationStatus::UnderWayEngine,
        longitude: Some(2.35),
        latitude: Some(48.85),
        sog: Some(5),
        cog: Some(91),
        gnss_position_status: true,
    };

    let extended_lines = extended
        .to_sentences(AisTransmitOptions::vdm(AisChannel::B))
        .expect("encode type 19");
    let long_range_lines = long_range
        .to_sentences(AisTransmitOptions::vdm(AisChannel::B))
        .expect("encode type 27");

    assert_eq!(
        extract_u32(&payload_bits(&extended_lines[0]), 133, 6),
        Some(61)
    );
    assert_eq!(
        extract_u32(&payload_bits(&long_range_lines[0]), 6, 2),
        Some(3)
    );
    assert!(
        matches!(decode_lines(&extended_lines), AisMessage::Position(value) if value.msg_type == 19 && value.mmsi == extended.mmsi)
    );
    assert!(
        matches!(decode_lines(&long_range_lines), AisMessage::LongRangePosition(value) if value.mmsi == long_range.mmsi && value.nav_status == Some(long_range.navigation_status))
    );
}

#[test]
fn class_a_position_encodes_to_a_decodable_vdm_sentence() {
    let report = ClassAPosition {
        message_type: ClassAPositionType::PositionReport,
        repeat_indicator: 0,
        mmsi: 244_670_316,
        navigation_status: NavigationStatus::UnderWayEngine,
        rate_of_turn: None,
        sog: Some(10.0),
        position_accuracy: true,
        longitude: Some(4.379_285),
        latitude: Some(51.894_75),
        cog: Some(70.6),
        heading: Some(71),
        timestamp: PositionTimestamp::Exact(5),
        maneuver_indicator: 1,
        raim: false,
        communication_state: 0,
    };
    for message_type in [
        ClassAPositionType::PositionReport,
        ClassAPositionType::AssignedPositionReport,
        ClassAPositionType::SpecialPositionReport,
    ] {
        for (raw, timestamp) in TIMESTAMP_CASES {
            let mut candidate = report.clone();
            candidate.message_type = message_type;
            let type_bits = match message_type {
                ClassAPositionType::PositionReport => [0, 0, 0, 0, 0, 1],
                ClassAPositionType::AssignedPositionReport => [0, 0, 0, 0, 1, 0],
                ClassAPositionType::SpecialPositionReport => [0, 0, 0, 0, 1, 1],
            };
            let wire = candidate
                .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
                .expect("class A type");
            assert_eq!(payload_bits(&wire[0])[..6], type_bits);

            candidate.timestamp = timestamp;
            check_timestamp_roundtrip(&candidate, raw, timestamp, 137);
        }
    }
    for invalid in [60, 255] {
        let mut candidate = report.clone();
        candidate.timestamp = PositionTimestamp::Exact(invalid);
        assert_eq!(
            candidate.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
            Err(EncodeError::InvalidAisField("timestamp"))
        );
    }

    let lines = report
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
        .expect("encode type 1");
    assert_eq!(lines.len(), 1);
    assert!(lines[0].starts_with("!AIVDM,1,1,,A,"));
    assert!(lines[0].len() <= 82);

    let frame = parse_frame(&lines[0]).expect("parse encoded type 1");
    let decoded = expect_message(AisParser::new().decode(&frame).expect("decode type 1"));
    match decoded {
        AisMessage::Position(position) => {
            assert_eq!(position.msg_type, 1);
            assert_eq!(position.mmsi, report.mmsi);
            assert_eq!(position.repeat_indicator, report.repeat_indicator);
            assert_eq!(position.latitude, report.latitude);
            assert_eq!(position.longitude, report.longitude);
            assert_eq!(position.cog, report.cog);
            assert_eq!(position.heading, report.heading);
            assert_eq!(position.maneuver_indicator, Some(report.maneuver_indicator));
            assert_eq!(position.raim, report.raim);
            assert_eq!(
                position.communication_state,
                Some(report.communication_state)
            );
        }
        other => panic!("expected position report, got {other:?}"),
    }
}

#[test]
fn long_range_gnss_status_preserves_its_wire_bit() {
    for status in [false, true] {
        let report = LongRangePosition {
            mmsi: 244_670_316,
            position_accuracy: true,
            raim: false,
            navigation_status: NavigationStatus::UnderWayEngine,
            longitude: Some(2.35),
            latitude: Some(48.85),
            sog: Some(5),
            cog: Some(91),
            gnss_position_status: status,
        };
        let lines = report
            .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
            .expect("encode type 27");
        assert_eq!(
            extract_u32(&payload_bits(&lines[0]), 94, 1),
            Some(u32::from(status))
        );
        assert!(matches!(
            decode_lines(&lines),
            AisMessage::LongRangePosition(value) if value.gnss_position_status == status
        ));
    }
}

#[test]
fn class_a_static_voyage_encodes_to_two_decodable_vdm_fragments() {
    let report = ClassAStaticVoyage {
        repeat_indicator: 0,
        mmsi: 366_123_456,
        ais_version: 3,
        imo: 9_876_543,
        callsign: "WDC1234".to_string(),
        vessel_name: "TEST VESSEL".to_string(),
        ship_type: 70,
        dimension_to_bow: 100,
        dimension_to_stern: 30,
        dimension_to_port: 10,
        dimension_to_starboard: 10,
        position_fixing_device: 1,
        eta_month: Some(7),
        eta_day: Some(28),
        eta_hour: Some(12),
        eta_minute: Some(30),
        draught_meters: Some(6.5),
        destination: "LE HAVRE".to_string(),
        dte: false,
    };

    let mut reserved_epfd = report.clone();
    reserved_epfd.position_fixing_device = 10;
    assert_eq!(
        reserved_epfd.to_sentences(AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(3)),
        Err(EncodeError::InvalidAisField("position_fixing_device"))
    );

    let lines = report
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(3))
        .expect("encode type 5");
    assert_eq!(lines.len(), 2);
    assert!(lines.iter().all(|line| line.len() <= 82));

    let mut parser = AisParser::new();
    assert_eq!(
        parser.decode(&parse_frame(&lines[0]).expect("parse first type 5 fragment")),
        Ok(AisDecodeOutcome::Pending)
    );
    let decoded = expect_message(
        parser
            .decode(&parse_frame(&lines[1]).expect("parse second type 5 fragment"))
            .expect("decode type 5"),
    );
    match decoded {
        AisMessage::StaticVoyage(data) => {
            assert_eq!(data.mmsi, report.mmsi);
            assert_eq!(data.imo, report.imo);
            assert_eq!(data.callsign, report.callsign);
            assert_eq!(data.vessel_name, report.vessel_name);
            assert_eq!(data.ship_type, report.ship_type);
            assert_eq!(data.repeat_indicator, report.repeat_indicator);
            assert_eq!(data.ais_version, report.ais_version);
            assert_eq!(data.dimension_to_bow, report.dimension_to_bow);
            assert_eq!(data.dimension_to_stern, report.dimension_to_stern);
            assert_eq!(data.dimension_to_port, report.dimension_to_port);
            assert_eq!(data.dimension_to_starboard, report.dimension_to_starboard);
            assert_eq!(data.position_fixing_device, report.position_fixing_device);
            assert_eq!(data.eta_month, report.eta_month);
            assert_eq!(data.eta_day, report.eta_day);
            assert_eq!(data.eta_hour, report.eta_hour);
            assert_eq!(data.eta_minute, report.eta_minute);
            assert_eq!(data.draught_meters, report.draught_meters);
            assert_eq!(data.destination, report.destination);
            assert_eq!(data.dte, report.dte);
        }
        other => panic!("expected static voyage data, got {other:?}"),
    }
}

#[test]
fn class_b_position_encodes_to_a_decodable_vdm_sentence() {
    let report = ClassBPosition {
        repeat_indicator: 0,
        mmsi: 235_009_217,
        sog: Some(5.4),
        position_accuracy: true,
        longitude: Some(2.352_2),
        latitude: Some(48.856_6),
        cog: Some(91.2),
        heading: Some(91),
        timestamp: PositionTimestamp::Exact(15),
        transmit_power_low: false,
        class_b_cs: true,
        display_available: true,
        dsc_capable: false,
        full_band_capable: true,
        message_22_capable: true,
        assigned_mode: false,
        raim: false,
        communication_state: ClassBCommunicationState::Itdma(0),
    };
    for (raw, timestamp) in TIMESTAMP_CASES {
        let mut candidate = report.clone();
        candidate.timestamp = timestamp;
        check_timestamp_roundtrip(&candidate, raw, timestamp, 133);
    }
    for invalid in [60, 255] {
        let mut candidate = report.clone();
        candidate.timestamp = PositionTimestamp::Exact(invalid);
        assert_eq!(
            candidate.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
            Err(EncodeError::InvalidAisField("timestamp"))
        );
    }

    let mut invalid_heading = report.clone();
    invalid_heading.heading = Some(360);
    assert_eq!(
        invalid_heading.to_sentences(AisTransmitOptions::vdm(AisChannel::B)),
        Err(EncodeError::InvalidAisField("heading"))
    );

    let mut invalid_timestamp = report.clone();
    invalid_timestamp.timestamp = PositionTimestamp::Exact(61);
    assert_eq!(
        invalid_timestamp.to_sentences(AisTransmitOptions::vdm(AisChannel::B)),
        Err(EncodeError::InvalidAisField("timestamp"))
    );

    let lines = report
        .to_sentences(AisTransmitOptions::vdm(AisChannel::B))
        .expect("encode type 18");
    assert_eq!(lines.len(), 1);
    assert!(lines[0].starts_with("!AIVDM,1,1,,B,"));
    assert!(lines[0].len() <= 82);

    let decoded = expect_message(
        AisParser::new()
            .decode(&parse_frame(&lines[0]).expect("parse encoded type 18"))
            .expect("decode type 18"),
    );
    match decoded {
        AisMessage::Position(position) => {
            assert_eq!(position.msg_type, 18);
            assert_eq!(position.mmsi, report.mmsi);
            assert_eq!(position.repeat_indicator, report.repeat_indicator);
            assert_eq!(position.latitude, report.latitude);
            assert_eq!(position.longitude, report.longitude);
            assert_eq!(position.cog, report.cog);
            assert_eq!(position.heading, report.heading);
            assert_eq!(position.raim, report.raim);
            assert_eq!(position.communication_state, Some(0));
            assert_eq!(
                position.class_b,
                Some(nmea_0183_rs::ais::ClassBPositionMetadata {
                    transmit_power_low: report.transmit_power_low,
                    class_b_cs: report.class_b_cs,
                    display_available: report.display_available,
                    dsc_capable: report.dsc_capable,
                    full_band_capable: report.full_band_capable,
                    message_22_capable: report.message_22_capable,
                    assigned_mode: report.assigned_mode,
                    communication_state_selector: true,
                })
            );
        }
        other => panic!("expected Class B position report, got {other:?}"),
    }
}

#[test]
fn class_b_static_parts_encode_to_decodable_vdm_sentences() {
    let part_a = ClassBStaticPartA {
        repeat_indicator: 0,
        mmsi: 235_009_217,
        vessel_name: "SIM BOAT".to_string(),
    };
    let part_b = ClassBStaticPartB {
        repeat_indicator: 0,
        mmsi: part_a.mmsi,
        ship_type: 37,
        manufacturer_id: "SIM".to_string(),
        model_code: 1,
        serial_number: 42,
        callsign: "FTEST01".to_string(),
        dimension_to_bow: 8,
        dimension_to_stern: 4,
        dimension_to_port: 2,
        dimension_to_starboard: 2,
        position_fixing_device: 1,
        vdes_capabilities: 0,
    };

    let mut reserved_epfd = part_b.clone();
    reserved_epfd.position_fixing_device = 11;
    assert_eq!(
        reserved_epfd.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        Err(EncodeError::InvalidAisField("position_fixing_device"))
    );

    let part_a_line = part_a
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
        .expect("encode type 24A");
    let part_b_line = part_b
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
        .expect("encode type 24B");
    assert_eq!(part_a_line.len(), 1);
    assert_eq!(part_b_line.len(), 1);
    assert!(part_a_line[0].len() <= 82);
    assert!(part_b_line[0].len() <= 82);

    let mut parser = AisParser::new();
    let decoded_a = expect_message(
        parser
            .decode(&parse_frame(&part_a_line[0]).expect("parse type 24A"))
            .expect("decode type 24A"),
    );
    let decoded_b = expect_message(
        parser
            .decode(&parse_frame(&part_b_line[0]).expect("parse type 24B"))
            .expect("decode type 24B"),
    );
    assert!(matches!(
        decoded_a,
        AisMessage::StaticReport(nmea_0183_rs::ais::StaticDataReport::PartA {
            repeat_indicator,
            mmsi,
            vessel_name,
        }) if repeat_indicator == part_a.repeat_indicator && mmsi == part_a.mmsi && vessel_name == part_a.vessel_name
    ));
    assert!(matches!(
        decoded_b,
        AisMessage::StaticReport(nmea_0183_rs::ais::StaticDataReport::PartB {
            repeat_indicator,
            mmsi,
            manufacturer_id,
            model_code,
            serial_number,
            callsign,
            ship_type,
            dimension_to_bow,
            dimension_to_stern,
            dimension_to_port,
            dimension_to_starboard,
            position_fixing_device,
            vdes_capabilities,
        }) if repeat_indicator == part_b.repeat_indicator
            && mmsi == part_b.mmsi
            && manufacturer_id == part_b.manufacturer_id
            && model_code == part_b.model_code
            && serial_number == part_b.serial_number
            && callsign == part_b.callsign
            && ship_type == part_b.ship_type
            && dimension_to_bow == part_b.dimension_to_bow
            && dimension_to_stern == part_b.dimension_to_stern
            && dimension_to_port == part_b.dimension_to_port
            && dimension_to_starboard == part_b.dimension_to_starboard
            && position_fixing_device == part_b.position_fixing_device
            && vdes_capabilities == part_b.vdes_capabilities
    ));
}

#[test]
fn class_a_static_voyage_requires_sequence_id_for_fragments() {
    let report = ClassAStaticVoyage {
        repeat_indicator: 0,
        mmsi: 366_123_456,
        ais_version: 0,
        imo: 0,
        callsign: String::new(),
        vessel_name: "SIMULATOR".to_string(),
        ship_type: 0,
        dimension_to_bow: 0,
        dimension_to_stern: 0,
        dimension_to_port: 0,
        dimension_to_starboard: 0,
        position_fixing_device: 0,
        eta_month: None,
        eta_day: None,
        eta_hour: None,
        eta_minute: None,
        draught_meters: None,
        destination: String::new(),
        dte: false,
    };

    assert_eq!(
        report.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        Err(EncodeError::MissingAisSequenceId)
    );
}

#[test]
fn class_a_static_voyage_encodes_unavailable_eta_with_ais_sentinels() {
    let report = ClassAStaticVoyage {
        repeat_indicator: 0,
        mmsi: 366_123_456,
        ais_version: 0,
        imo: 0,
        callsign: String::new(),
        vessel_name: "SIMULATOR".to_string(),
        ship_type: 0,
        dimension_to_bow: 0,
        dimension_to_stern: 0,
        dimension_to_port: 0,
        dimension_to_starboard: 0,
        position_fixing_device: 0,
        eta_month: None,
        eta_day: None,
        eta_hour: None,
        eta_minute: None,
        draught_meters: None,
        destination: String::new(),
        dte: false,
    };
    let lines = report
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(0))
        .expect("encode type 5");
    let mut parser = AisParser::new();
    assert!(matches!(
        parser.decode(&parse_frame(&lines[0]).expect("parse first fragment")),
        Ok(AisDecodeOutcome::Pending)
    ));
    let decoded = expect_message(
        parser
            .decode(&parse_frame(&lines[1]).expect("parse second fragment"))
            .expect("decode type 5"),
    );
    let AisMessage::StaticVoyage(data) = decoded else {
        panic!("expected static voyage data");
    };
    assert_eq!(data.eta_month, None);
    assert_eq!(data.eta_day, None);
    assert_eq!(data.eta_hour, None);
    assert_eq!(data.eta_minute, None);
}

#[test]
fn ais_transmitter_rejects_invalid_field_values() {
    let invalid_text = ClassBStaticPartA {
        repeat_indicator: 0,
        mmsi: 235_009_217,
        vessel_name: "navire_école".to_string(),
    };
    assert_eq!(
        invalid_text.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        Err(EncodeError::InvalidAisField("vessel_name"))
    );

    let too_long = ClassBStaticPartA {
        repeat_indicator: 0,
        mmsi: 235_009_217,
        vessel_name: "A".repeat(21),
    };
    assert_eq!(
        too_long.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        Err(EncodeError::AisTextTooLong {
            field: "vessel_name",
            max_chars: 20,
            actual_chars: 21,
        })
    );

    let invalid_range = ClassBStaticPartA {
        repeat_indicator: 4,
        mmsi: 235_009_217,
        vessel_name: "SIM BOAT".to_string(),
    };
    assert_eq!(
        invalid_range.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
        Err(EncodeError::InvalidAisField("repeat_indicator"))
    );
}

fn assert_byte_aligned_message(report: &impl AisEncodable, unpadded_len: usize) {
    let lines = report
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1))
        .expect("encode aligned message");
    let bits: Vec<u8> = lines.iter().flat_map(|line| payload_bits(line)).collect();
    assert_eq!(bits.len(), unpadded_len.div_ceil(8) * 8);
    assert!(bits[unpadded_len..].iter().all(|&bit| bit == 0));
    for (index, line) in lines.iter().enumerate() {
        nmea_0183_rs::validate_sentence(line).expect("strict AIS envelope");
        let frame = parse_frame(line).expect("frame");
        if index + 1 < lines.len() {
            assert_eq!(frame.fields[5], "0");
        }
    }
}

#[test]
fn safety_text_is_byte_aligned_at_all_lengths() {
    super::bounded::run_bounded(
        "transmit::safety_text_is_byte_aligned_at_all_lengths",
        || {
            for count in 0..=161 {
                let text = "A".repeat(count);
                let broadcast = SafetyBroadcast {
                    repeat_indicator: 0,
                    mmsi: 244_670_316,
                    text: text.clone(),
                };
                assert_byte_aligned_message(&broadcast, 40 + 6 * count);
                let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
                assert!(matches!(
                    decode_lines(&broadcast.to_sentences(options).expect("encode")),
                    AisMessage::Safety(value) if value.text == text
                ));
                if count <= 156 {
                    let addressed = SafetyAddressed {
                        repeat_indicator: 0,
                        mmsi: 244_670_316,
                        sequence: 0,
                        destination_mmsi: 235_009_217,
                        retransmit: false,
                        text: text.clone(),
                    };
                    assert_byte_aligned_message(&addressed, 72 + 6 * count);
                    assert!(matches!(
                        decode_lines(&addressed.to_sentences(options).expect("encode")),
                        AisMessage::SafetyAddressed(value) if value.text == text
                    ));
                }
            }
        },
    );
}

#[test]
fn aton_extension_is_byte_aligned_at_all_lengths() {
    super::bounded::run_bounded(
        "transmit::aton_extension_is_byte_aligned_at_all_lengths",
        || {
            let mut report = AidToNavigation {
                repeat_indicator: 0,
                mmsi: 992_001_001,
                aid_type: 1,
                name: "TEST".to_string(),
                longitude: None,
                latitude: None,
                position_accuracy: false,
                dimension_to_bow: 0,
                dimension_to_stern: 0,
                dimension_to_port: 0,
                dimension_to_starboard: 0,
                position_fixing_device: 0,
                timestamp: PositionTimestamp::NotAvailable,
                off_position: false,
                regional_application: 0,
                raim: false,
                virtual_aid: false,
                assigned_mode: false,
                name_extension: None,
            };
            assert_byte_aligned_message(&report, 272);
            for count in 1..=14 {
                let extension = "A".repeat(count);
                report.name_extension = Some(extension.clone());
                assert_byte_aligned_message(&report, 272 + 6 * count);
                let lines = report
                    .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
                    .expect("encode AtoN");
                assert!(matches!(
                    decode_lines(&lines),
                    AisMessage::AidToNavigation(value)
                        if value.name_extension.as_deref() == Some(extension.as_str())
                ));
            }
        },
    );
}

fn boundary_classaposition() -> ClassAPosition {
    ClassAPosition {
        message_type: ClassAPositionType::PositionReport,
        repeat_indicator: 0,
        mmsi: 244_670_316,
        navigation_status: NavigationStatus::UnderWayEngine,
        rate_of_turn: None,
        sog: Some(10.0),
        position_accuracy: true,
        longitude: Some(4.379_285),
        latitude: Some(51.894_75),
        cog: Some(70.6),
        heading: Some(71),
        timestamp: PositionTimestamp::Exact(5),
        maneuver_indicator: 1,
        raim: false,
        communication_state: 0,
    }
}

fn boundary_basestation() -> BaseStation {
    BaseStation {
        repeat_indicator: 0,
        mmsi: 111_222_333,
        year: Some(2026),
        month: Some(7),
        day: Some(28),
        hour: Some(12),
        minute: Some(30),
        second: Some(15),
        position_accuracy: true,
        longitude: Some(2.352_2),
        latitude: Some(48.856_6),
        position_fixing_device: 1,
        transmission_control: false,
        raim: true,
        communication_state: 42,
    }
}

fn boundary_saraircraft() -> SarAircraft {
    SarAircraft {
        repeat_indicator: 0,
        mmsi: 111_222_334,
        altitude: Some(300),
        sog: Some(120),
        position_accuracy: true,
        longitude: Some(2.352_2),
        latitude: Some(48.856_6),
        cog: Some(91.2),
        timestamp: PositionTimestamp::DeadReckoning,
        regional_application: 0,
        dte: false,
        assigned_mode: false,
        raim: false,
        communication_state: ClassBCommunicationState::Sotdma(0),
    }
}

fn boundary_longrangeposition() -> LongRangePosition {
    LongRangePosition {
        mmsi: 244_670_316,
        position_accuracy: true,
        raim: false,
        navigation_status: NavigationStatus::UnderWayEngine,
        longitude: Some(2.35),
        latitude: Some(48.85),
        sog: Some(5),
        cog: Some(91),
        gnss_position_status: true,
    }
}

fn boundary_classastaticvoyage() -> ClassAStaticVoyage {
    ClassAStaticVoyage {
        repeat_indicator: 0,
        mmsi: 366_123_456,
        ais_version: 3,
        imo: 9_876_543,
        callsign: "WDC1234".to_string(),
        vessel_name: "TEST VESSEL".to_string(),
        ship_type: 70,
        dimension_to_bow: 100,
        dimension_to_stern: 30,
        dimension_to_port: 10,
        dimension_to_starboard: 10,
        position_fixing_device: 1,
        eta_month: Some(7),
        eta_day: Some(28),
        eta_hour: Some(12),
        eta_minute: Some(30),
        draught_meters: Some(6.5),
        destination: "LE HAVRE".to_string(),
        dte: false,
    }
}

fn boundary_aidtonavigation() -> AidToNavigation {
    AidToNavigation {
        repeat_indicator: 0,
        mmsi: 992_001_001,
        aid_type: 1,
        name: "TEST BUOY".to_string(),
        longitude: Some(2.352_2),
        latitude: Some(48.856_6),
        position_accuracy: true,
        dimension_to_bow: 2,
        dimension_to_stern: 2,
        dimension_to_port: 1,
        dimension_to_starboard: 1,
        position_fixing_device: 1,
        timestamp: PositionTimestamp::Inoperative,
        off_position: false,
        regional_application: 0,
        raim: false,
        virtual_aid: false,
        assigned_mode: false,
        name_extension: Some("WEST".to_string()),
    }
}

#[test]
fn classaposition_sog_boundaries() {
    let mut report = boundary_classaposition();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0.0), Some(102.2)] {
        report.sog = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [
        Some(-0.1),
        Some(102.3),
        Some(f32::NAN),
        Some(f32::INFINITY),
        Some(f32::NEG_INFINITY),
    ] {
        report.sog = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("sog")),
            "{value:?}"
        );
    }
}

#[test]
fn classaposition_cog_boundaries() {
    let mut report = boundary_classaposition();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0.0), Some(359.9)] {
        report.cog = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [
        Some(-0.1),
        Some(360.0),
        Some(f32::NAN),
        Some(f32::INFINITY),
        Some(f32::NEG_INFINITY),
    ] {
        report.cog = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("cog")),
            "{value:?}"
        );
    }
}

#[test]
fn classaposition_heading_boundaries() {
    let mut report = boundary_classaposition();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(359)] {
        report.heading = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    {
        let value = Some(360);
        report.heading = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("heading")),
            "{value:?}"
        );
    }
}

#[test]
fn classaposition_rate_of_turn_boundaries() {
    let mut report = boundary_classaposition();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(-127), Some(127)] {
        report.rate_of_turn = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    {
        let value = Some(-128);
        report.rate_of_turn = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("rate_of_turn")),
            "{value:?}"
        );
    }
}

#[test]
fn classaposition_latitude_boundaries() {
    let mut report = boundary_classaposition();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0.0), Some(-90.0), Some(90.0)] {
        report.latitude = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [
        Some(-90.001),
        Some(90.001),
        Some(f64::NAN),
        Some(f64::INFINITY),
        Some(f64::NEG_INFINITY),
    ] {
        report.latitude = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("latitude")),
            "{value:?}"
        );
    }
}

#[test]
fn classaposition_longitude_boundaries() {
    let mut report = boundary_classaposition();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0.0), Some(-180.0), Some(180.0)] {
        report.longitude = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [
        Some(-180.001),
        Some(180.001),
        Some(f64::NAN),
        Some(f64::INFINITY),
        Some(f64::NEG_INFINITY),
    ] {
        report.longitude = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("longitude")),
            "{value:?}"
        );
    }
}

#[test]
fn longrangeposition_latitude_boundaries() {
    let mut report = boundary_longrangeposition();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0.0), Some(-90.0), Some(90.0)] {
        report.latitude = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [
        Some(-90.001),
        Some(90.001),
        Some(f64::NAN),
        Some(f64::INFINITY),
        Some(f64::NEG_INFINITY),
    ] {
        report.latitude = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("latitude")),
            "{value:?}"
        );
    }
}

#[test]
fn longrangeposition_longitude_boundaries() {
    let mut report = boundary_longrangeposition();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0.0), Some(-180.0), Some(180.0)] {
        report.longitude = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [
        Some(-180.001),
        Some(180.001),
        Some(f64::NAN),
        Some(f64::INFINITY),
        Some(f64::NEG_INFINITY),
    ] {
        report.longitude = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("longitude")),
            "{value:?}"
        );
    }
}

#[test]
fn basestation_year_boundaries() {
    let mut report = boundary_basestation();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(1), Some(9999)] {
        report.year = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(0), Some(10000)] {
        report.year = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("year")),
            "{value:?}"
        );
    }
}

#[test]
fn basestation_month_boundaries() {
    let mut report = boundary_basestation();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(1), Some(12)] {
        report.month = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(0), Some(13)] {
        report.month = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("month")),
            "{value:?}"
        );
    }
}

#[test]
fn basestation_day_boundaries() {
    let mut report = boundary_basestation();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(1), Some(31)] {
        report.day = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(0), Some(32)] {
        report.day = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("day")),
            "{value:?}"
        );
    }
}

#[test]
fn basestation_hour_boundaries() {
    let mut report = boundary_basestation();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0), Some(23)] {
        report.hour = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(24), Some(255)] {
        report.hour = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("hour")),
            "{value:?}"
        );
    }
}

#[test]
fn basestation_minute_boundaries() {
    let mut report = boundary_basestation();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0), Some(59)] {
        report.minute = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(60), Some(255)] {
        report.minute = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("minute")),
            "{value:?}"
        );
    }
}

#[test]
fn basestation_second_boundaries() {
    let mut report = boundary_basestation();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0), Some(59)] {
        report.second = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(60), Some(255)] {
        report.second = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("second")),
            "{value:?}"
        );
    }
}

#[test]
fn classastaticvoyage_eta_month_boundaries() {
    let mut report = boundary_classastaticvoyage();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(1), Some(12)] {
        report.eta_month = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(0), Some(13)] {
        report.eta_month = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("eta_month")),
            "{value:?}"
        );
    }
}

#[test]
fn classastaticvoyage_eta_day_boundaries() {
    let mut report = boundary_classastaticvoyage();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(1), Some(31)] {
        report.eta_day = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(0), Some(32)] {
        report.eta_day = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("eta_day")),
            "{value:?}"
        );
    }
}

#[test]
fn classastaticvoyage_eta_hour_boundaries() {
    let mut report = boundary_classastaticvoyage();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0), Some(23)] {
        report.eta_hour = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(24), Some(255)] {
        report.eta_hour = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("eta_hour")),
            "{value:?}"
        );
    }
}

#[test]
fn classastaticvoyage_eta_minute_boundaries() {
    let mut report = boundary_classastaticvoyage();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0), Some(59)] {
        report.eta_minute = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(60), Some(255)] {
        report.eta_minute = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("eta_minute")),
            "{value:?}"
        );
    }
}

#[test]
fn saraircraft_altitude_boundaries() {
    let mut report = boundary_saraircraft();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0), Some(4094)] {
        report.altitude = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(4095), Some(4096)] {
        report.altitude = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("altitude")),
            "{value:?}"
        );
    }
}

#[test]
fn saraircraft_sog_boundaries() {
    let mut report = boundary_saraircraft();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0), Some(1022)] {
        report.sog = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(1023), Some(1024)] {
        report.sog = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("sog")),
            "{value:?}"
        );
    }
}

#[test]
fn longrangeposition_sog_boundaries() {
    let mut report = boundary_longrangeposition();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0), Some(62)] {
        report.sog = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(63), Some(64)] {
        report.sog = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("sog")),
            "{value:?}"
        );
    }
}

#[test]
fn longrangeposition_cog_boundaries() {
    let mut report = boundary_longrangeposition();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0), Some(359)] {
        report.cog = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [Some(360), Some(511)] {
        report.cog = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("cog")),
            "{value:?}"
        );
    }
}

#[test]
fn classastaticvoyage_draught_meters_boundaries() {
    let mut report = boundary_classastaticvoyage();
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    for value in [None, Some(0.1), Some(25.5)] {
        report.draught_meters = value;
        assert!(report.to_sentences(options).is_ok(), "{value:?}");
    }
    for value in [
        Some(0.0),
        Some(25.6),
        Some(f32::NAN),
        Some(f32::INFINITY),
        Some(f32::NEG_INFINITY),
    ] {
        report.draught_meters = value;
        assert_eq!(
            report.to_sentences(options),
            Err(EncodeError::InvalidAisField("draught_meters")),
            "{value:?}"
        );
    }
}

#[test]
fn safety_addressed_text_limit() {
    let mut report = SafetyAddressed {
        repeat_indicator: 0,
        mmsi: 244670316,
        sequence: 0,
        destination_mmsi: 235009217,
        retransmit: false,
        text: "A".repeat(156),
    };
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    let lines = report.to_sentences(options).expect("156 accepted");
    let bits: Vec<_> = lines.iter().flat_map(|line| payload_bits(line)).collect();
    assert_eq!(bits.len(), 1008);
    assert_eq!(
        parse_frame(lines.last().expect("last fragment"))
            .expect("frame")
            .fields[5],
        "0"
    );
    for character in bits[72..].chunks_exact(6) {
        assert_eq!(character, [0, 0, 0, 0, 0, 1]);
    }
    report.text.push('A');
    assert_eq!(
        report.to_sentences(options),
        Err(EncodeError::AisTextTooLong {
            field: "text",
            max_chars: 156,
            actual_chars: 157
        })
    );
}

#[test]
fn aton_extension_limit() {
    let mut report = boundary_aidtonavigation();
    report.name_extension = Some("A".repeat(14));
    let options = AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1);
    let lines = report.to_sentences(options).expect("14 accepted");
    let bits: Vec<_> = lines.iter().flat_map(|line| payload_bits(line)).collect();
    for character in bits[272..356].chunks_exact(6) {
        assert_eq!(character, [0, 0, 0, 0, 0, 1]);
    }
    assert!(bits[356..].iter().all(|&bit| bit == 0));
    report.name_extension = Some("A".repeat(15));
    assert_eq!(
        report.to_sentences(options),
        Err(EncodeError::AisTextTooLong {
            field: "name_extension",
            max_chars: 14,
            actual_chars: 15
        })
    );
}

#[test]
fn class_a_wire_sentinels_and_extremes() {
    let mut report = boundary_classaposition();
    let options = AisTransmitOptions::vdm(AisChannel::A);
    report.sog = None;
    report.cog = None;
    report.heading = None;
    report.latitude = None;
    report.longitude = None;
    report.rate_of_turn = None;
    let lines = report.to_sentences(options).expect("encode");
    let bits = payload_bits(&lines[0]);
    for (offset, width, expected) in [
        (42, 8, 128),
        (50, 10, 1023),
        (61, 28, 108600000),
        (89, 27, 54600000),
        (116, 12, 3600),
        (128, 9, 511),
    ] {
        assert_eq!(
            bits[offset..offset + width]
                .iter()
                .fold(0_u32, |v, b| (v << 1) | u32::from(*b)),
            expected
        );
    }
    report.sog = Some(102.2);
    report.cog = Some(359.9);
    report.heading = Some(359);
    for (latitude, longitude) in [(-90.0, -180.0), (0.0, 0.0), (90.0, 180.0)] {
        report.latitude = Some(latitude);
        report.longitude = Some(longitude);
        let lines = report.to_sentences(options).expect("extremes");
        let bits = payload_bits(&lines[0]);
        for (offset, width, expected) in [
            (61, 28, (longitude * 600000.0) as i32),
            (89, 27, (latitude * 600000.0) as i32),
        ] {
            let raw = bits[offset..offset + width]
                .iter()
                .fold(0_i64, |v, b| (v << 1) | i64::from(*b));
            let signed = if bits[offset] == 1 {
                raw - (1_i64 << width)
            } else {
                raw
            };
            assert_eq!(signed, i64::from(expected));
        }
        assert_eq!(extract_u32(&bits, 50, 10), Some(1022));
        assert_eq!(extract_u32(&bits, 116, 12), Some(3599));
    }
}

#[test]
fn specialized_wire_sentinels_and_extremes() {
    let options = AisTransmitOptions::vdm(AisChannel::A);
    let mut long = boundary_longrangeposition();
    long.latitude = None;
    long.longitude = None;
    long.sog = None;
    long.cog = None;
    let lines = long.to_sentences(options).expect("sentinels");
    let bits = payload_bits(&lines[0]);
    for (offset, width, expected) in [(44, 18, 108600), (62, 17, 54600), (79, 6, 63), (85, 9, 511)]
    {
        assert_eq!(
            bits[offset..offset + width]
                .iter()
                .fold(0_u32, |v, b| (v << 1) | u32::from(*b)),
            expected
        );
    }
    for (lat, lon) in [(-90.0, -180.0), (0.0, 0.0), (90.0, 180.0)] {
        long.latitude = Some(lat);
        long.longitude = Some(lon);
        let lines = long.to_sentences(options).expect("extremes");
        let bits = payload_bits(&lines[0]);
        for (offset, width, expected) in [
            (44, 18, (lon * 600.0) as i32),
            (62, 17, (lat * 600.0) as i32),
        ] {
            let raw = bits[offset..offset + width]
                .iter()
                .fold(0_i64, |v, b| (v << 1) | i64::from(*b));
            assert_eq!(
                if bits[offset] == 1 {
                    raw - (1_i64 << width)
                } else {
                    raw
                },
                i64::from(expected)
            );
        }
    }
    let mut sar = boundary_saraircraft();
    sar.altitude = None;
    sar.sog = None;
    let lines = sar.to_sentences(options).expect("SAR sentinels");
    let bits = payload_bits(&lines[0]);
    assert_eq!(extract_u32(&bits, 38, 12), Some(4095));
    assert_eq!(extract_u32(&bits, 50, 10), Some(1023));
}

#[test]
fn class_b_communication_state_bits() {
    let mut report = ClassBPosition {
        repeat_indicator: 0,
        mmsi: 235_009_217,
        sog: Some(5.4),
        position_accuracy: true,
        longitude: Some(2.352_2),
        latitude: Some(48.856_6),
        cog: Some(91.2),
        heading: Some(91),
        timestamp: PositionTimestamp::Exact(15),
        transmit_power_low: false,
        class_b_cs: true,
        display_available: true,
        dsc_capable: false,
        full_band_capable: true,
        message_22_capable: true,
        assigned_mode: false,
        raim: false,
        communication_state: ClassBCommunicationState::Itdma(0),
    };
    for (state, selector, expected) in [
        (
            ClassBCommunicationState::Sotdma(0x12345),
            0,
            "0010010001101000101",
        ),
        (
            ClassBCommunicationState::Itdma(0x54321),
            1,
            "1010100001100100001",
        ),
    ] {
        report.communication_state = state;
        let lines = report
            .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
            .expect("radio encode");
        let bits = payload_bits(&lines[0]);
        assert_eq!(bits[148], selector);
        assert_eq!(
            bits[149..168],
            expected.bytes().map(|b| b - b'0').collect::<Vec<_>>()
        );
    }
    for state in [
        ClassBCommunicationState::Sotdma(1 << 19),
        ClassBCommunicationState::Itdma(1 << 19),
    ] {
        report.communication_state = state;
        assert_eq!(
            report.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
            Err(EncodeError::InvalidAisField("communication_state"))
        );
    }
}

#[test]
fn sar_communication_state_bits() {
    let mut report = boundary_saraircraft();
    for (state, selector, expected) in [
        (
            ClassBCommunicationState::Sotdma(0x12345),
            0,
            "0010010001101000101",
        ),
        (
            ClassBCommunicationState::Itdma(0x54321),
            1,
            "1010100001100100001",
        ),
    ] {
        report.communication_state = state;
        let lines = report
            .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
            .expect("radio encode");
        let bits = payload_bits(&lines[0]);
        assert_eq!(bits[148], selector);
        assert_eq!(
            bits[149..168],
            expected.bytes().map(|b| b - b'0').collect::<Vec<_>>()
        );
    }
    for state in [
        ClassBCommunicationState::Sotdma(1 << 19),
        ClassBCommunicationState::Itdma(1 << 19),
    ] {
        report.communication_state = state;
        assert_eq!(
            report.to_sentences(AisTransmitOptions::vdm(AisChannel::A)),
            Err(EncodeError::InvalidAisField("communication_state"))
        );
    }
}

#[test]
fn safety_text_emits_exact_alphabet() {
    const ALPHABET: &str = r##"@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_ !"#$%&'()*+,-./0123456789:;<=>?"##;
    let report = SafetyBroadcast {
        repeat_indicator: 0,
        mmsi: 123456789,
        text: ALPHABET.to_string(),
    };
    let lines = report
        .to_sentences(AisTransmitOptions::vdm(AisChannel::A).with_sequence_id(1))
        .expect("alphabet encode");
    let bits: Vec<u8> = lines.iter().flat_map(|line| payload_bits(line)).collect();
    assert_eq!(bits.len(), 424);
    assert_eq!(bits[..6], [0, 0, 1, 1, 1, 0]);
    let expected: Vec<u8> = (0_u8..64)
        .flat_map(|value| (0..6).rev().map(move |shift| (value >> shift) & 1))
        .collect();
    assert_eq!(bits[40..424], expected);
    match decode_lines(&lines) {
        AisMessage::Safety(message) => assert_eq!(message.text, ALPHABET),
        other => panic!("unexpected {other:?}"),
    }
}
