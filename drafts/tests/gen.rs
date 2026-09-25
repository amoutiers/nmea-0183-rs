use nmea_0183_rs::nmea::sentences::Gen;
use nmea_0183_rs::parse_frame;

#[test]
fn gen_pynmeagps_values() {
    let frame = parse_frame("$VRGEN,0000,011200.00,0123,4567,89AB,CDEF,0123,4567,89AB,CDEF*64")
        .expect("valid GEN");
    let generic = Gen::parse(&frame.fields).expect("parse GEN");
    assert_eq!(generic.index.as_deref(), Some("0000"));
    assert_eq!(generic.time.as_deref(), Some("011200.00"));
    assert_eq!(
        generic.data,
        [
            "0123", "4567", "89AB", "CDEF", "0123", "4567", "89AB", "CDEF"
        ]
    );
}
