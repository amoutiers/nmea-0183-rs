use nmea_0183_rs::nmea::sentences::Nrm;
use nmea_0183_rs::parse_frame;

#[test]
fn nrm_pynmeagps_values() {
    let frame = parse_frame("$INNRM,2,1,00001E1F,00000023,C*38").expect("valid NRM");
    let nrm = Nrm::parse(&frame.fields).expect("parse NRM");
    assert_eq!(nrm.function, Some('2'));
    assert_eq!(nrm.transmit_coverage_mask.as_deref(), Some("00001E1F"));
    assert_eq!(nrm.status, Some('C'));
}
