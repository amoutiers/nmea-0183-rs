#![cfg_attr(
    not(test),
    deny(
        clippy::panic,
        clippy::expect_used,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]
//! # nmea-0183-rs
//!
//! Bidirectional NMEA 0183 parser/encoder with AIS decoding and transponder-message encoding.
//!
//! ## Architecture
//!
//! ```text
//! raw line ──→ parse_frame() ──→ NmeaFrame { prefix, talker, sentence_type, fields }
//!                                     │
//!                      ┌──────────────┼──────────────┐
//!                      ▼              ▼               ▼
//!                $ + known      $ + unknown     ! AIVDM/AIVDO     AIS app sentences
//!                      │              │               │              │
//!                      ▼              ▼               ▼              ▼
//!               Typed struct    Raw fields      AisMessage enum   AIS sentence struct
//! ```
//!
//! ## Public API
//!
//! - [`parse_frame`] / [`encode_frame`] — device-compatible frame APIs (always available)
//! - [`parse_frame_strict`] / [`validate_sentence`] / [`encode_frame_strict`] — strict frame-envelope APIs (always available)
//! - `NmeaSentence` — dispatch enum for all typed NMEA sentences (requires an NMEA feature)
//! - `NmeaEncodable` — trait providing compatible and strict wire encoding (requires an NMEA feature)
//! - `ais` — AIS decoder, transponder-message encoder, and `!`-prefixed AIS application sentences (requires `ais`, `abm`, or `bbm`)
//!
//! `NmeaFrame::to_sentence()` re-encodes envelope data, including tag blocks,
//! while normalizing checksums and CRLF. Keep the original line for exact bytes.
//! `NmeaSentence` and `ais::sentences::AisSentence` also expose compatible and
//! strict encoding; their `Unknown` variants own and preserve the captured envelope.
//! Typed variants require the original frame to retain tags and the original talker.
//! NMEA and ABM/BBM structs implement `Default` for construction with partial data.
//! Their field parsers return `Self` directly; missing or malformed fields remain optional.
//!
//! With `ais`, `AisParser::decode()` distinguishes ignored frames,
//! pending fragments, messages and `AisDecodeError` through a `Result`.
//! Use one parser per physical source.
//! Position timestamps use `ais::messages::PositionTimestamp`, preserving the
//! unavailable, manual-input, dead-reckoning and inoperative states.
//!
//! ## Partial construction and enum encoding
//!
//! ```
//! # #[cfg(feature = "dpt")]
//! # {
//! use nmea_0183_rs::{NmeaSentence, parse_frame};
//! use nmea_0183_rs::nmea::Dpt;
//! let value = NmeaSentence::Dpt(Dpt { depth: Some(4.1), ..Default::default() });
//! let output = value.to_sentence_strict("SD")?;
//! assert_eq!(NmeaSentence::parse(&parse_frame(&output)?), value);
//! # }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Strict validation covers the shared frame envelope. It does not validate every
//! formatter's field semantics, serial transport settings, transmission timing,
//! or multipart reassembly outside the AIS parser.
//!
//! ## Features
//!
//! - `nmea` (default) — all 69 standard NMEA sentence types
//! - `nmea_proprietary` (default) — all 16 proprietary NMEA sentence types
//! - `ais` (default) — decoding for all numeric AIS Types 1-27, encoder support for Types 1/2/3, 4, 5, 9, 11, 12, 14, 18, 19, 21, 24 and 27, plus ABM, BBM and VSD sentences
//! - `dbs`, `dbt`, `dpt`, … — individual sentence types

mod compliance;
mod error;
mod frame;

// Single source of truth for the "any NMEA sentence feature is active" predicate.
// Add new sentence feature names here when wiring a new type.
macro_rules! nmea_item {
    ($item:item) => {
        #[cfg(any(
            feature = "nmea",
            feature = "aam",
            feature = "ack",
            feature = "acn",
            feature = "ala",
            feature = "alc",
            feature = "alf",
            feature = "alr",
            feature = "arc",
            feature = "apb",
            feature = "bec",
            feature = "bod",
            feature = "bwc",
            feature = "bwr",
            feature = "bww",
            feature = "dbk",
            feature = "dbs",
            feature = "dbt",
            feature = "dor",
            feature = "dpt",
            feature = "dsc",
            feature = "dse",
            feature = "dtm",
            feature = "eve",
            feature = "fir",
            feature = "gbs",
            feature = "gga",
            feature = "gll",
            feature = "gns",
            feature = "gsa",
            feature = "gsv",
            feature = "gst",
            feature = "hbt",
            feature = "hdg",
            feature = "hdm",
            feature = "hdt",
            feature = "hsc",
            feature = "mda",
            feature = "mta",
            feature = "mtw",
            feature = "mwd",
            feature = "mwv",
            feature = "osd",
            feature = "pashr",
            feature = "pcdin",
            feature = "pgrme",
            feature = "pgrmt",
            feature = "phtro",
            feature = "pklid",
            feature = "pklds",
            feature = "pklsh",
            feature = "pknds",
            feature = "pknid",
            feature = "pknsh",
            feature = "pkwdwpl",
            feature = "pmtk",
            feature = "prdid",
            feature = "psoncms",
            feature = "pskpdpt",
            feature = "rmb",
            feature = "rmc",
            feature = "rot",
            feature = "rpm",
            feature = "rsa",
            feature = "rsd",
            feature = "rte",
            feature = "ths",
            feature = "tll",
            feature = "tlb",
            feature = "ttd",
            feature = "ttm",
            feature = "txt",
            feature = "vbw",
            feature = "vdr",
            feature = "vhw",
            feature = "vlw",
            feature = "vpw",
            feature = "vtg",
            feature = "vwr",
            feature = "vwt",
            feature = "vsd",
            feature = "wcv",
            feature = "wpl",
            feature = "xdr",
            feature = "xte",
            feature = "zda",
        ))]
        $item
    };
}

nmea_item! { pub mod nmea; }

#[cfg(any(feature = "ais", feature = "abm", feature = "bbm"))]
pub mod ais;

pub use compliance::*;
pub use error::*;
pub use frame::*;

nmea_item! { pub use nmea::NmeaSentence; }
nmea_item! { pub use nmea::NmeaEncodable; }
