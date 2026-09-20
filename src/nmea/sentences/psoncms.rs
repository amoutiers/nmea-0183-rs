use crate::nmea::field::{FieldReader, FieldWriter, NmeaEncodable};

/// PSONCMS — IMU/AHRS Device Data (Xsens).
///
/// Proprietary sentence from Xsens AHRS devices.
/// Wire: `$PSONCMS,q0,q1,q2,q3,ax,ay,az,rx,ry,rz,mx,my,mz,temp`
///
/// Proprietary sentence: `parse_frame` sets `talker = ""`, `sentence_type = "PSONCMS"`.
#[derive(Debug, Clone, PartialEq)]
pub struct Psoncms {
    /// Quaternion component 0.
    pub quaternion_0: Option<f32>,
    /// Quaternion component 1.
    pub quaternion_1: Option<f32>,
    /// Quaternion component 2.
    pub quaternion_2: Option<f32>,
    /// Quaternion component 3.
    pub quaternion_3: Option<f32>,
    /// Acceleration X (m/s²).
    pub accel_x: Option<f32>,
    /// Acceleration Y (m/s²).
    pub accel_y: Option<f32>,
    /// Acceleration Z (m/s²).
    pub accel_z: Option<f32>,
    /// Rotation rate X (rad/s).
    pub rot_x: Option<f32>,
    /// Rotation rate Y (rad/s).
    pub rot_y: Option<f32>,
    /// Rotation rate Z (rad/s).
    pub rot_z: Option<f32>,
    /// Magnetic field X.
    pub mag_x: Option<f32>,
    /// Magnetic field Y.
    pub mag_y: Option<f32>,
    /// Magnetic field Z.
    pub mag_z: Option<f32>,
    /// Temperature (°C).
    pub temperature: Option<f32>,
}

impl Psoncms {
    /// Parse fields from a decoded NMEA frame.
    /// Always returns `Some`; missing or malformed fields become `None`.
    pub fn parse(fields: &[&str]) -> Option<Self> {
        let mut r = FieldReader::new(fields);
        let quaternion_0 = r.f32();
        let quaternion_1 = r.f32();
        let quaternion_2 = r.f32();
        let quaternion_3 = r.f32();
        let accel_x = r.f32();
        let accel_y = r.f32();
        let accel_z = r.f32();
        let rot_x = r.f32();
        let rot_y = r.f32();
        let rot_z = r.f32();
        let mag_x = r.f32();
        let mag_y = r.f32();
        let mag_z = r.f32();
        let temperature = r.f32();
        Some(Self {
            quaternion_0,
            quaternion_1,
            quaternion_2,
            quaternion_3,
            accel_x,
            accel_y,
            accel_z,
            rot_x,
            rot_y,
            rot_z,
            mag_x,
            mag_y,
            mag_z,
            temperature,
        })
    }
}

impl NmeaEncodable for Psoncms {
    const SENTENCE_TYPE: &str = "PSONCMS";
    const PROPRIETARY: bool = true;

    fn encode(&self) -> Result<Vec<String>, crate::EncodeError> {
        let mut w = FieldWriter::new();
        w.f32(self.quaternion_0);
        w.f32(self.quaternion_1);
        w.f32(self.quaternion_2);
        w.f32(self.quaternion_3);
        w.f32(self.accel_x);
        w.f32(self.accel_y);
        w.f32(self.accel_z);
        w.f32(self.rot_x);
        w.f32(self.rot_y);
        w.f32(self.rot_z);
        w.f32(self.mag_x);
        w.f32(self.mag_y);
        w.f32(self.mag_z);
        w.f32(self.temperature);
        w.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_frame;

    #[test]
    fn psoncms_empty() {
        let s = Psoncms {
            quaternion_0: None,
            quaternion_1: None,
            quaternion_2: None,
            quaternion_3: None,
            accel_x: None,
            accel_y: None,
            accel_z: None,
            rot_x: None,
            rot_y: None,
            rot_z: None,
            mag_x: None,
            mag_y: None,
            mag_z: None,
            temperature: None,
        }
        .to_sentence("").expect("encode");
        let f = parse_frame(s.trim()).expect("valid");
        let p = Psoncms::parse(&f.fields).expect("parse");
        assert!(p.quaternion_0.is_none());
        assert!(p.temperature.is_none());
    }

    #[test]
    fn psoncms_encode_roundtrip() {
        let original = Psoncms {
            quaternion_0: Some(0.0905),
            quaternion_1: Some(0.4217),
            quaternion_2: Some(0.9020),
            quaternion_3: Some(-0.0196),
            accel_x: Some(-1.7685),
            accel_y: Some(0.3861),
            accel_z: Some(-9.6648),
            rot_x: Some(-0.0116),
            rot_y: Some(0.0065),
            rot_z: Some(-0.0080),
            mag_x: Some(0.0581),
            mag_y: Some(0.3846),
            mag_z: Some(0.7421),
            temperature: Some(33.1),
        };
        let sentence = original.to_sentence("").expect("encode");
        let frame = parse_frame(sentence.trim()).expect("re-parse");
        let parsed = Psoncms::parse(&frame.fields).expect("parse");
        assert_eq!(original, parsed);
    }

    #[test]
    fn psoncms_values() {
        let f = parse_frame(
            "$PSONCMS,0.0905,0.4217,0.9020,-0.0196,-1.7685,0.3861,-9.6648,-0.0116,0.0065,-0.0080,0.0581,0.3846,0.7421,33.1*76",
        )
        .expect("valid PSONCMS");
        let p = Psoncms::parse(&f.fields).expect("parse PSONCMS");
        assert!((p.quaternion_0.expect("q0") - 0.0905).abs() < 0.0001);
        assert!((p.quaternion_1.expect("q1") - 0.4217).abs() < 0.0001);
        assert!((p.quaternion_2.expect("q2") - 0.9020).abs() < 0.0001);
        assert!((p.quaternion_3.expect("q3") - (-0.0196)).abs() < 0.0001);
        assert!((p.accel_x.expect("ax") - (-1.7685)).abs() < 0.0001);
        assert!((p.accel_y.expect("ay") - 0.3861).abs() < 0.0001);
        assert!((p.accel_z.expect("az") - (-9.6648)).abs() < 0.0001);
        assert!((p.rot_x.expect("rx") - (-0.0116)).abs() < 0.0001);
        assert!((p.rot_y.expect("ry") - 0.0065).abs() < 0.0001);
        assert!((p.rot_z.expect("rz") - (-0.0080)).abs() < 0.0001);
        assert!((p.mag_x.expect("mx") - 0.0581).abs() < 0.0001);
        assert!((p.mag_y.expect("my") - 0.3846).abs() < 0.0001);
        assert!((p.mag_z.expect("mz") - 0.7421).abs() < 0.0001);
        assert!((p.temperature.expect("temp") - 33.1).abs() < 0.01);
    }
}
