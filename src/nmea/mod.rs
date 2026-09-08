//! NMEA 0183 sentence parsing and encoding.

mod field;
pub mod sentences;

pub use field::{NmeaEncodable, ddmm_to_decimal, decimal_to_ddmm};
pub use sentences::*;

use crate::NmeaFrame;

macro_rules! nmea_sentences {
    (
        standard: [ $( [$feat:literal, $variant:ident, $wire:literal] ),* $(,)? ],
        proprietary: [ $( [$pfeat:literal, $pvariant:ident, $pwire:literal] ),* $(,)? ]
    ) => {
        /// Unified enum covering all supported NMEA 0183 sentence types.
        ///
        /// Use `NmeaSentence::parse(&frame)` to dispatch a parsed frame to the
        /// appropriate typed struct. Unknown sentence types are captured in the
        /// `Unknown` variant.
        ///
        /// Proprietary sentences (`$P...`) are dispatched separately from standard
        /// sentences, so there is no risk of collision between e.g. `$PSKPDPT` and
        /// standard `$IIDPT`.
        #[non_exhaustive]
        #[derive(Debug, Clone, PartialEq)]
        pub enum NmeaSentence {
            $(
                #[cfg(feature = $feat)]
                $variant(sentences::$variant),
            )*
            $(
                #[cfg(feature = $pfeat)]
                $pvariant(sentences::$pvariant),
            )*
            Unknown {
                sentence_type: String,
                fields: Vec<String>,
            },
        }

        impl NmeaSentence {
            /// Parse a frame into a typed sentence variant.
            ///
            /// Standard sentences are dispatched on `frame.sentence_type` (3-char code).
            /// Proprietary sentences (where `frame.talker` is empty) are dispatched on
            /// the full address in `frame.sentence_type` (e.g. `"PASHR"`, `"PSKPDPT"`).
            ///
            /// Returns `Unknown` for unrecognized types.
            pub fn parse(frame: &NmeaFrame<'_>) -> Self {
                if frame.prefix != '$' {
                    return Self::from_frame(frame);
                }

                macro_rules! try_parse {
                    ($parser:expr, $v:ident) => {
                        match $parser(&frame.fields) {
                            Some(v) => return Self::$v(v),
                            // parse() is documented to always return Some for known
                            // types; degrade to Unknown rather than panic if that
                            // invariant is ever broken by a future change.
                            None => return Self::from_frame(frame),
                        }
                    };
                }

                // Standard table matches the 3-char code for '$'-prefixed frames.
                // Proprietary wire codes (PASHR, PSKPDPT, …) never collide with a
                // 3-char standard code, so a proprietary frame falls through.
                match frame.sentence_type {
                    $(
                        #[cfg(feature = $feat)]
                        $wire => try_parse!(sentences::$variant::parse, $variant),
                    )*
                    _ => {}
                }

                // Proprietary table — full address in `sentence_type`, only for
                // proprietary frames (talker is empty per the frame parser).
                if frame.talker.is_empty() {
                    match frame.sentence_type {
                        $(
                            #[cfg(feature = $pfeat)]
                            $pwire => try_parse!(sentences::$pvariant::parse, $pvariant),
                        )*
                        _ => {}
                    }
                }

                Self::from_frame(frame)
            }

            /// Build an `Unknown` variant preserving the frame's sentence type and fields.
            fn from_frame(frame: &NmeaFrame<'_>) -> Self {
                Self::Unknown {
                    sentence_type: frame.sentence_type.to_string(),
                    fields: frame.fields.iter().map(|f| f.to_string()).collect(),
                }
            }
        }
    };
}

nmea_sentences![
    standard: [
        // Position
        ["dtm", Dtm, "DTM"],
        ["rmc", Rmc, "RMC"],
        // Satellites
        ["gbs", Gbs, "GBS"],
        ["gsa", Gsa, "GSA"],
        ["gsv", Gsv, "GSV"],
        ["gst", Gst, "GST"],
        ["gga", Gga, "GGA"],
        ["gll", Gll, "GLL"],
        ["gns", Gns, "GNS"],
        // Wind
        ["mwd", Mwd, "MWD"],
        ["mwv", Mwv, "MWV"],
        ["vpw", Vpw, "VPW"],
        ["vwr", Vwr, "VWR"],
        ["vwt", Vwt, "VWT"],
        // AIS interface
        ["vsd", Vsd, "VSD"],
        // Heading
        ["hdt", Hdt, "HDT"],
        ["hdg", Hdg, "HDG"],
        ["hdm", Hdm, "HDM"],
        ["hsc", Hsc, "HSC"],
        ["rot", Rot, "ROT"],
        ["ths", Ths, "THS"],
        // Navigation
        ["aam", Aam, "AAM"],
        ["osd", Osd, "OSD"],
        ["apb", Apb, "APB"],
        ["bec", Bec, "BEC"],
        ["bod", Bod, "BOD"],
        ["bwc", Bwc, "BWC"],
        ["bwr", Bwr, "BWR"],
        ["bww", Bww, "BWW"],
        ["rmb", Rmb, "RMB"],
        ["rte", Rte, "RTE"],
        ["wcv", Wcv, "WCV"],
        ["wpl", Wpl, "WPL"],
        ["xte", Xte, "XTE"],
        // Rudder
        ["rsa", Rsa, "RSA"],
        // Speed
        ["vbw", Vbw, "VBW"],
        ["vlw", Vlw, "VLW"],
        ["vtg", Vtg, "VTG"],
        ["vhw", Vhw, "VHW"],
        ["rpm", Rpm, "RPM"],
        ["vdr", Vdr, "VDR"],
        // Depth
        ["dpt", Dpt, "DPT"],
        ["dbt", Dbt, "DBT"],
        ["dbs", Dbs, "DBS"],
        ["dbk", Dbk, "DBK"],
        // Environment
        ["mda", Mda, "MDA"],
        ["mta", Mta, "MTA"],
        ["mtw", Mtw, "MTW"],
        ["xdr", Xdr, "XDR"],
        // Radar / Target tracking
        ["rsd", Rsd, "RSD"],
        ["tll", Tll, "TLL"],
        ["tlb", Tlb, "TLB"],
        ["ttd", Ttd, "TTD"],
        ["ttm", Ttm, "TTM"],
        // Alert
        ["ack", Ack, "ACK"],
        ["acn", Acn, "ACN"],
        ["ala", Ala, "ALA"],
        ["alc", Alc, "ALC"],
        ["alf", Alf, "ALF"],
        ["alr", Alr, "ALR"],
        ["arc", Arc, "ARC"],
        ["dor", Dor, "DOR"],
        ["eve", Eve, "EVE"],
        ["fir", Fir, "FIR"],
        // Communication
        ["dsc", Dsc, "DSC"],
        ["dse", Dse, "DSE"],
        ["txt", Txt, "TXT"],
        // Heartbeat
        ["hbt", Hbt, "HBT"],
        // Time
        ["zda", Zda, "ZDA"],
    ],
    proprietary: [
        // Proprietary — Ashtech/Trimble
        ["pashr", Pashr, "PASHR"],
        // Proprietary — Garmin
        ["pgrme", Pgrme, "PGRME"],
        ["pgrmt", Pgrmt, "PGRMT"],
        ["phtro", Phtro, "PHTRO"],
        // Proprietary — Skipper
        ["pskpdpt", Pskpdpt, "PSKPDPT"],
    ]
];

#[cfg(test)]
mod tests {
    #[cfg(any(feature = "pgrme", feature = "pskpdpt", feature = "rmc"))]
    use super::*;
    #[cfg(any(feature = "pgrme", feature = "pskpdpt", feature = "rmc"))]
    use crate::parse_frame;

    #[test]
    #[cfg(feature = "pgrme")]
    fn proprietary_dispatch_requires_empty_talker() {
        let frame = NmeaFrame {
            prefix: '$',
            talker: "GP",
            sentence_type: "PGRME",
            fields: vec!["1", "2", "3"],
            tag_block: None,
        };
        assert!(matches!(
            NmeaSentence::parse(&frame),
            NmeaSentence::Unknown { .. }
        ));
    }

    #[test]
    #[cfg(feature = "rmc")]
    fn talkerless_standard_address_dispatches() {
        // A bare "$RMC,..." (no talker) parses with talker == "".
        let frame = parse_frame("$RMC,175957.917,A,3857.1234,N,07705.1234,W,0.0,0.0,010100,,,A*60")
            .expect("valid frame");
        assert_eq!(frame.talker, "");
        assert_eq!(frame.sentence_type, "RMC");
        // It must dispatch to the standard Rmc variant, not Unknown.
        assert!(matches!(NmeaSentence::parse(&frame), NmeaSentence::Rmc(_)));
    }

    #[test]
    #[cfg(feature = "pskpdpt")]
    fn proprietary_still_dispatches() {
        let frame = parse_frame("$PSKPDPT,0002.5,+00.0,0010,10,03,*77").expect("valid");
        assert!(matches!(
            NmeaSentence::parse(&frame),
            NmeaSentence::Pskpdpt(_)
        ));
    }
}
