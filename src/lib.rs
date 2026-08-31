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
//! # nmea-kit
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
//! - [`parse_frame`] / [`encode_frame`] — frame layer (always available)
//! - [`NmeaSentence`] — dispatch enum for all typed NMEA sentences
//! - [`NmeaEncodable`] — trait for encoding NMEA sentences to wire format
//! - [`ais`] — AIS decoder, transponder-message encoder, and `!`-prefixed AIS application sentences
//!
//! ## Features
//!
//! - `nmea` (default) — all 70 NMEA sentence types
//! - `ais` (default) — decoding for all numeric AIS Types 1-27, encoder support for Types 1/2/3, 4, 5, 9, 11, 12, 14, 18, 19, 21, 24 and 27, plus ABM/BBM application sentences
//! - `dbs`, `dbt`, `dpt`, … — individual sentence types

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
            feature = "pgrme",
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

pub use error::*;
pub use frame::*;

nmea_item! { pub use nmea::NmeaSentence; }
nmea_item! { pub use nmea::NmeaEncodable; }
