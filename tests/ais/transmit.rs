use super::expect_message;
use nmea_0183_rs::ais::AisDecodeOutcome;
use nmea_0183_rs::ais::armor::{decode_armor, extract_u32};
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
    decode_armor(payload, fill_bits).expect("decode emitted armor")
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
