# nmea-kit

Bidirectional NMEA 0183 parser/encoder + AIS decoder and transponder-message encoder. Zero dependencies. MIT/Apache-2.0.

| Key | Value |
|---|---|
| Crate | `nmea-kit` v0.8.3 |
| Edition | 2024, MSRV 1.85.0 |
| Dependencies | 0 |
| NMEA sentences | 70 (bidirectional) |
| AIS application sentences | 2 (bidirectional) |
| AIS message types | All numeric Types 1-27 decoded; Types 1/2/3, 4, 5, 9, 11, 12, 14, 18, 19, 21, 24 and 27 also encoded |
| Tests | 791, 0 failures |
| Unsafe blocks | 0 |

For contribution workflow, test rules, and the sentence-type checklist see [CONTRIBUTING.md](CONTRIBUTING.md).

## API surface

### Imports and signatures

```rust
// Frame layer (always available)
use nmea_kit::{parse_frame, encode_frame, EncodeError, NmeaFrame, FrameError};

parse_frame(input: &str) -> Result<NmeaFrame, FrameError>
encode_frame(prefix: char, talker: &str, sentence_type: &str, fields: &[&str]) -> Result<String, EncodeError>

// NMEA dispatch
use nmea_kit::{NmeaSentence, NmeaEncodable};

NmeaSentence::parse(&frame) -> NmeaSentence   // enum variant per type
value.to_sentence(talker: &str) -> Result<String, EncodeError> // NmeaEncodable; proprietary types ignore talker

// Individual sentence types
use nmea_kit::nmea::sentences::{Mwd, Rmc, Dbt, Vsd, ...}; // standard
use nmea_kit::nmea::sentences::{Pashr, Pskpdpt, ...};    // proprietary

Type::parse(fields: &[&str]) -> Option<Self>   // always Some for known types
value.encode() -> Result<Vec<String>, EncodeError> // fields in wire order

// Coordinate helpers
use nmea_kit::nmea::{ddmm_to_decimal, decimal_to_ddmm};

ddmm_to_decimal(ddmm: f64) -> f64   // DDMM.MMMM → decimal degrees
decimal_to_ddmm(decimal: f64) -> f64 // decimal degrees → DDMM.MMMM

// AIS decoder and !-prefixed application-layer sentences
use nmea_kit::ais::{AisParser, AisMessage};
use nmea_kit::ais::sentences::{Abm, Bbm, AisSentence};
use nmea_kit::ais::transmit::{AisChannel, AisEncodable, AisTransmitOptions, ClassAPosition};

let mut parser = AisParser::new();
parser.decode(&frame) -> Option<AisMessage>    // None while awaiting fragments
parser.reset()                                  // clear fragment buffers
message.to_sentences(AisTransmitOptions::vdm(AisChannel::A)) -> Result<Vec<String>, EncodeError>
```

### Error model

- **Frame layer**: `parse_frame()` returns `Result<NmeaFrame, FrameError>`. Variants: `Empty`, `InvalidPrefix`, `MalformedChecksum`, `BadChecksum`, `MalformedTagBlock`, `BadTagChecksum`, `TooShort`, `NonAsciiAddress`. When a tag-block checksum is present it is validated, and `tag_block` excludes its `*hh` suffix.
- **Encode layer**: all encode APIs return `Result<_, EncodeError>`. Variants: `InvalidPrefix`, `NonAsciiAddress`, `EmptySentenceType`, `InvalidFieldCharacter`, `InvalidCoordinate`, `InvalidAisField`, `AisTextTooLong`, `MissingAisSequenceId`, `TooManyAisFragments`.
- **NMEA content**: `parse()` always returns `Some`. Missing/malformed fields → `None` inside the struct. Intentional for marine instruments that send partial data.
- **AIS content**: `decode()` returns `Option<AisMessage>`. `None` = awaiting fragments or decode failure.
- **No panics**: 0 `panic!`, 0 `unwrap()`, 0 `todo!` in library code.

### NmeaFrame

```rust
pub struct NmeaFrame<'a> {
    pub prefix: char,               // '$' for NMEA, '!' for AIS
    pub talker: &'a str,            // e.g. "GP", "WI", "AI" — "" for proprietary
    pub sentence_type: &'a str,     // "RMC" (standard) or "PASHR" (proprietary)
    pub fields: Vec<&'a str>,       // comma-split payload
    pub tag_block: Option<&'a str>, // IEC 61162-450 content if present
}
```

### Key NMEA structs

```rust
pub struct Rmc {
    pub time: Option<String>,       // "HHMMSS.SSS"
    pub status: Option<char>,       // 'A' = active, 'V' = void
    pub lat: Option<f64>,           // raw DDMM.MMMM — use ddmm_to_decimal()
    pub ns: Option<char>,           // 'N' or 'S'
    pub lon: Option<f64>,           // raw DDDMM.MMMM
    pub ew: Option<char>,           // 'E' or 'W'
    pub sog: Option<f32>,           // knots
    pub cog: Option<f32>,           // degrees true
    pub date: Option<String>,       // "DDMMYY"
    pub mag_var: Option<f32>,
    pub mag_var_ew: Option<char>,
    pub pos_mode: Option<char>,     // 'A'=autonomous, 'D'=differential
    pub nav_status: Option<char>,   // NMEA 4.1 nav status: 'S'/'C'/'U'/'V'
}

pub struct Mwd {
    pub wind_dir_true: Option<f32>,  // degrees
    pub wind_dir_mag: Option<f32>,   // degrees
    pub wind_speed_kts: Option<f32>, // knots
    pub wind_speed_ms: Option<f32>,  // m/s
}

pub struct Dbt {
    pub depth_feet: Option<f32>,
    pub depth_meters: Option<f32>,
    pub depth_fathoms: Option<f32>,
}
```

### Key AIS structs

```rust
pub struct PositionReport {          // Types 1/2/3/18/19
    pub msg_type: u8,
    pub mmsi: u32,
    pub nav_status: Option<NavigationStatus>,
    pub rate_of_turn: Option<f32>,
    pub sog: Option<f32>,            // 1/10 knot
    pub position_accuracy: bool,
    pub longitude: Option<f64>,      // decimal degrees (already converted)
    pub latitude: Option<f64>,       // decimal degrees (already converted)
    pub cog: Option<f32>,            // 1/10 degree
    pub heading: Option<u16>,        // integer degrees
    pub timestamp: Option<u8>,
    pub ais_class: AisClass,         // ClassA or ClassB
}

pub enum AisMessage {
    Position(PositionReport),             // Types 1/2/3/18/19
    BaseStation(BaseStationReport),       // Type 4
    StaticVoyage(StaticVoyageData),       // Type 5
    BinaryAddressed(BinaryAddressed),     // Type 6
    BinaryAck(BinaryAck),                // Types 7/13
    BinaryBroadcast(BinaryBroadcast),    // Type 8
    SarAircraft(SarAircraftReport),      // Type 9
    UtcDateInquiry(UtcDateInquiry),      // Type 10
    UtcDateResponse(UtcDateResponse),    // Type 11
    SafetyAddressed(SafetyAddressed),    // Type 12
    Safety(SafetyBroadcast),             // Type 14
    Interrogation(Interrogation),        // Type 15
    AssignmentMode(AssignmentModeCommand),// Type 16
    DgnssBroadcast(DgnssBroadcast),      // Type 17
    DataLinkManagement(DataLinkManagement), // Type 20
    AidToNavigation(AidToNavigation),    // Type 21
    ChannelManagement(ChannelManagement),// Type 22
    GroupAssignment(GroupAssignment),    // Type 23
    StaticReport(StaticDataReport),      // Type 24
    BinarySingleSlot(BinarySingleSlot),  // Type 25
    BinaryMultiSlot(BinaryMultiSlot),    // Type 26
    LongRangePosition(LongRangePosition),// Type 27
    Unknown { msg_type: u8 },              // unsupported message type
}
```

### Coordinate formats

| Source | Format | Example |
|---|---|---|
| NMEA (RMC, GGA, GLL…) | DDMM.MMMM (raw) | `4807.038` = 48°07.038' |
| AIS (PositionReport…) | Decimal degrees | `48.1173` = 48.1173° |

Convert: `ddmm_to_decimal(4807.038)` → `48.1173`. Apply N/S E/W sign separately (negate for S or W).

## Structure

```
src/lib.rs              → parse_frame, encode_frame, NmeaSentence, NmeaEncodable
src/frame.rs            → NmeaFrame struct, frame parsing logic
src/error.rs            → FrameError enum
src/nmea/mod.rs         → NmeaSentence enum + dispatch macro
src/nmea/field.rs       → FieldReader, FieldWriter, ddmm_to_decimal, decimal_to_ddmm
src/nmea/sentences/*.rs → one file per sentence type (struct + parse + encode)
src/ais/mod.rs          → AisParser + AisMessage enum
src/ais/sentences/*.rs  → !-prefixed AIS application-layer NMEA sentences: ABM, BBM
src/ais/armor.rs        → 6-bit ASCII decode + bit extraction
src/ais/encode.rs       → bit writer, AIS text/position quantization, 6-bit armor encoding
src/ais/fragments.rs    → multi-fragment reassembly
src/ais/messages/*.rs   → one file per AIS message type
src/ais/transmit/*.rs   → stateless AIVDM/AIVDO encoder models for a simulated transponder
```

## Key patterns

### NMEA sentence implementation

Every NMEA sentence type follows the same pattern using `FieldReader`/`FieldWriter`:

- `SENTENCE_TYPE` — 3-char const (`"MWD"`, `"RMC"`, etc.)
- `parse(fields: &[&str]) -> Option<Self>` — sequential field reading (always returns `Some`, lenient)
- `encode(&self) -> Result<Vec<String>, EncodeError>` — sequential field writing
- `to_sentence(&self, talker: &str) -> Result<String, EncodeError>` — default impl on `NmeaEncodable` trait

Fixed indicator fields (T, M, N, K, f, F) are handled with `r.skip()` on parse and `w.fixed('T')` on encode.

### Proprietary sentences (`$P...`)

Per NMEA 0183, addresses starting with `P` are proprietary. The frame parser detects
these and sets `talker = ""`, `sentence_type = full address` (e.g. `"PASHR"`, `"PSKPDPT"`).
This prevents collisions with standard 3-char types (e.g. `$PSKPDPT` won't match `DPT`).

Proprietary types additionally set:
- `SENTENCE_TYPE` — the full wire address (`"PASHR"`, `"PSKPDPT"`, etc.)
- `const PROPRIETARY: bool = true` — makes `to_sentence(talker)` ignore the talker and emit the full address

Dispatch uses a two-path match in the `nmea_sentences!` macro: standard types match on
`sentence_type` when `talker` is non-empty, proprietary types match on the full address
when `talker` is empty.

### AIS message implementation

AIS types use bit-level extraction from decoded 6-bit armor. The transponder encoder uses a matching bit writer, armor encoder, and NMEA frame encoder:

- `extract_u32(bits, offset, len)` / `extract_i32(bits, offset, len)` for numeric fields
- `extract_string(bits, offset, num_chars)` for AIS 6-bit text
- Sentinel values (91/181 lat/lon, 511 heading, 1023 SOG, 3600 COG) filtered to `None`
- All lat/lon use `f64` (not `f32`), heading uses `u16` (integer degrees per AIS spec)
- Bit extraction uses manual `Vec<u8>` (one byte per bit), not `bitvec`
- `AisParser::reset()` clears in-progress fragment buffers

`ais::transmit` is stateless: callers select `AisChannel` and `AisSentenceKind` through `AisTransmitOptions`, then call `AisEncodable::to_sentences()`. It emits Types 1/2/3 (`ClassAPosition`), 4 (`BaseStation`), 5 (`ClassAStaticVoyage`), 9 (`SarAircraft`), 11 (`UtcDateResponse`), 12 (`SafetyAddressed`), 14 (`SafetyBroadcast`), 18 (`ClassBPosition`), 19 (`ClassBExtendedPosition`), 21 (`AidToNavigation`), 24 parts A/B (`ClassBStaticPartA` / `ClassBStaticPartB`), and 27 (`LongRangePosition`). It splits armor at 60 characters, requires a sequence ID for multi-fragment output, and never schedules transmission cycles.

Helper functions in `position_a.rs` are `pub(crate)` — shared by `position_b.rs` and `position_b_ext.rs`.

### AIS application-layer sentences

ABM and BBM live under `ais::sentences`, preserve their `!` prefix, and are dispatched with `AisSentence::parse(&frame)`. VSD is a conventional NMEA sentence (`$--VSD`) under `nmea::sentences`, dispatched with `NmeaSentence::parse(&frame)`.

## Field definitions reference

Sentence field layouts are sourced from [pynmeagps](https://github.com/semuconsulting/pynmeagps) (`nmeatypes_get.py`). Test fixtures from [SignalK](https://github.com/SignalK/signalk-parser-nmea0183) and [GPSD](https://gitlab.com/gpsd/gpsd). Full sentence coverage tracked in [SENTENCES.md](SENTENCES.md).

## Commands

```sh
cargo test --all-features                                # run all tests
cargo doc --all-features --open                          # browse docs locally
cargo clippy --all-features --all-targets -- -D warnings # lint
cargo fmt                                                # format
```

## Constraints

- No `nom`, no proc-macro, no `syn`/`quote` — keep compile times minimal
- Zero dependencies (serde was removed as unused)
- AIS AIVDM/AIVDO Types 1/2/3, 4, 5, 9, 11, 12, 14, 18, 19, 21, 24 and 27 are bidirectional; all other supported AIS Types 1-27 are decode-only. ABM/BBM and NMEA VSD are bidirectional
- No `unwrap()` in library code — `expect("description")` in tests only
- No `panic!`, `todo!`, or `#[allow(dead_code)]` in `src/`
- Edition 2024, MSRV 1.85.0
