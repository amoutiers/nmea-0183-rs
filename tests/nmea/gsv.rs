#![cfg(feature = "gsv")]
use nmea_0183_rs::nmea::NmeaEncodable;

use nmea_0183_rs::nmea::sentences::{Gsv, SatInfo};
use nmea_0183_rs::{NmeaSentence, parse_frame};

#[test]
fn decode_encode() {
    let frame = parse_frame("$GPGSV,3,1,09,09,73,246,35,02,51,060,40,06,16,058,37,07,16,291,25*78")
        .expect("valid");
    let gsv = Gsv::parse(&frame.fields).expect("parse");
    let sentence = gsv.to_sentence("GP").expect("encode");
    let frame2 = parse_frame(sentence.trim()).expect("re-parse");
    let gsv2 = Gsv::parse(&frame2.fields).expect("parse");
    assert_eq!(gsv, gsv2);
}

#[test]
fn dispatch() {
    let frame = parse_frame("$GPGSV,3,1,09,09,73,246,35,02,51,060,40,06,16,058,37,07,16,291,25*78")
        .expect("valid");
    assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Gsv(_)));
}

#[test]
fn roundtrip() {
    let original = Gsv {
        total_msgs: Some(3),
        msg_num: Some(1),
        sats_in_view: Some(9),
        sats: vec![
            SatInfo {
                prn: Some(9),
                elevation: Some(73),
                azimuth: Some(246),
                snr: Some(35),
            },
            SatInfo {
                prn: Some(2),
                elevation: Some(51),
                azimuth: Some(60),
                snr: Some(40),
            },
            SatInfo {
                prn: Some(6),
                elevation: Some(16),
                azimuth: Some(58),
                snr: Some(37),
            },
            SatInfo {
                prn: Some(7),
                elevation: Some(16),
                azimuth: Some(35),
                snr: Some(25),
            },
        ],
        signal_id: None,
    };
    let sentence = original.to_sentence("GP").expect("encode");
    let frame = parse_frame(sentence.trim()).expect("re-parse");
    let parsed = Gsv::parse(&frame.fields).expect("parse");
    assert_eq!(original, parsed);
}

#[test]
fn gsv_values() {
    let frame = parse_frame("$GPGSV,3,1,09,09,73,246,35,02,51,060,40,06,16,058,37,07,16,291,25*78")
        .expect("valid GSV fixture");
    let gsv = Gsv::parse(&frame.fields).expect("parse GSV");
    assert_eq!(gsv.total_msgs, Some(3));
    assert_eq!(gsv.msg_num, Some(1));
    assert_eq!(gsv.sats_in_view, Some(9));
    assert_eq!(gsv.sats.len(), 4);
    // sats[0]: PRN 09, elevation 73°, azimuth 246°, SNR 35
    assert_eq!(gsv.sats[0].prn, Some(9));
    assert_eq!(gsv.sats[0].elevation, Some(73));
    assert_eq!(gsv.sats[0].azimuth, Some(246));
    assert_eq!(gsv.sats[0].snr, Some(35));
    // sats[1]: PRN 02, elevation 51°, azimuth 060°, SNR 40
    assert_eq!(gsv.sats[1].prn, Some(2));
    assert_eq!(gsv.sats[1].elevation, Some(51));
    assert_eq!(gsv.sats[1].azimuth, Some(60));
    assert_eq!(gsv.sats[1].snr, Some(40));
    // sats[2]: PRN 06, elevation 16°, azimuth 058°, SNR 37
    assert_eq!(gsv.sats[2].prn, Some(6));
    assert_eq!(gsv.sats[2].elevation, Some(16));
    assert_eq!(gsv.sats[2].azimuth, Some(58));
    assert_eq!(gsv.sats[2].snr, Some(37));
    // sats[3]: PRN 07, elevation 16°, azimuth 291°, SNR 25
    assert_eq!(gsv.sats[3].prn, Some(7));
    assert_eq!(gsv.sats[3].elevation, Some(16));
    assert_eq!(gsv.sats[3].azimuth, Some(291));
    assert_eq!(gsv.sats[3].snr, Some(25));
    assert!(gsv.signal_id.is_none());
}
