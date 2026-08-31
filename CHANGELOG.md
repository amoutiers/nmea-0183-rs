# Changelog

All notable changes to nmea-kit are documented here.

## [Unreleased]

## [0.8.3] — 2026-08-31

### Added
- EVE (General Event Message) sentence type — event time, tag code, and message text.
- FIR (Fire Detection) sentence type — fire type, time, system and detector identifiers, condition, acknowledgement state, and message text.

NMEA sentence coverage: 68 → 70 types.

## [0.8.2] — 2026-08-19

### Added
- DSC (Digital Selective Calling) sentence type — format, address, category, command, position, MMSI, distress, acknowledgement, and expansion fields.
- DSE (DSC Extended) sentence type — sentence sequence, acknowledgement, MMSI, and variable code/data pairs.

NMEA sentence coverage: 66 → 68 types.

## [0.8.1] — 2026-08-15

### Added
- AIS transponder-message encoding for `!AIVDM` and `!AIVDO`: Types 1/2/3, 5, 18, and 24. The encoder validates bit-field values and AIS text, adds the checksum, and fragments payloads at 60 armor characters.
- AIS transponder-message encoding for Types 4, 9, 11, 12, 14, 19, 21, and 27, including Type 12/14 safety-text fragmentation and explicit manual, dead-reckoning, and inoperative position-timestamp states.
- Typed AIS decoding for Types 10, 16, 17, 20, 22, 23, 25, and 26. Existing decoders now preserve the non-reserved protocol metadata that was previously discarded.
- ALF (Alert) sentence type — alert fragments, message identifier, category, priority, state, alert metadata, and text.
- DOR (Door Status Detection) sentence type — door type, identifiers, status, switch setting, and message text.

NMEA sentence coverage: 64 → 66 types.

## [0.8.0] — 2026-07-27

### Changed (breaking)
- `encode_frame()` now returns `Result<String, EncodeError>` and validates the prefix, the address (ASCII, non-empty type), and every field (no `,`, `*`, `\r`, `\n`, non-ASCII)
- `NmeaEncodable::to_sentence()` and `encode()` now return `Result<_, EncodeError>`; same for XDR `to_sentences()`, AIS ABM/BBM, and NMEA VSD
- Invalid coordinate magnitudes (NaN, infinite, negative) now produce an explicit `EncodeError::InvalidCoordinate` instead of malformed wire output. This also removes a debug-build panic in `FieldWriter`
- The IEC 61162-450 tag block checksum is now validated when present; invalid checksums are rejected (`FrameError::BadTagChecksum`), and `NmeaFrame::tag_block` no longer includes the `*hh` suffix
- AIS reserved values now decode to `None`: Type 9 timestamp 61-63, Types 4/11 hour 25-31, minute 61-63, second 61-63, month 13-15, and heading 360-510 (Types 1/2/3/18/19)
- `Vsd` moved from `nmea_kit::ais::sentences::Vsd` to `nmea_kit::nmea::sentences::Vsd`
- `AisSentence::Vsd` was removed; dispatch `$--VSD` with `NmeaSentence::parse()` and match `NmeaSentence::Vsd`
- Feature `vsd` is now an NMEA feature; the `ais` umbrella feature enables ABM/BBM application sentences plus AIS VDM/VDO decoding

### Fixed
- Single-fragment AIS frames now enforce the 256-character payload limit (DoS hardening)
- Field-less proprietary (`$PABC*10`) and talkerless (`$RMC*5C`) frames are no longer rejected as `TooShort`
- A tag block without a following sentence now returns `TooShort` instead of `Empty`
- XDR rejects `NaN`/`inf` values like every other sentence type
- Proprietary dispatch no longer triggers on hand-built frames with a non-empty talker
- `AisPayload::channel` is `'A'`/`'B'` as documented when the numeric channel syntax (`1`/`2`) is used
- NMEA and AIS application dispatchers reject frames with the wrong wire prefix

### Documentation
- README feature examples now target `nmea-kit` 0.8.
- Proprietary-sentence contribution and release instructions match `NmeaEncodable`.

### Migration
- `let s = x.to_sentence("GP");` → `let s = x.to_sentence("GP")?;` (or `.expect("...")`)
- `encode_frame(...)` → `encode_frame(...)?`
- If you parse `frame.tag_block` yourself, the `*hh` checksum suffix is no longer present

## [0.7.6] — 2026-07-26

### Added
- ALA (System Faults and Alarms) sentence type — alarm source, condition, acknowledgement state, and message
- ALC (Cyclic Alert List) sentence type — fragment metadata and alert entries

NMEA sentence coverage: 61 → 63 types.

## [0.7.5] — 2026-07-25

### Added
- ALR (Set Alarm State) sentence type — alarm time, identifier, condition, acknowledgement state, and description
- ARC (Alert Response Command) sentence type — alert time, manufacturer, identifier, instance, and refused command

NMEA sentence coverage: 59 → 61 types.

## [0.7.4] — 2026-07-12

### Added
- ABM (AIS Addressed Binary Message) sentence type — fragment metadata, destination MMSI, channel, VDL message number, payload, and fill bits
- BBM (AIS Broadcast Binary Message) sentence type — fragment metadata, channel, VDL message number, payload, and fill bits
- VSD (AIS Voyage Static Data) sentence type — ship type, draught, persons on board, destination, ETA, navigation status, and regional field

### Changed
- CI now checks every feature independently without default features.

### Fixed
- AIS armor extractors now return `None` for overflowing offsets instead of panicking.
- AIS armor decoding rejects invalid fill-bit counts.
- AIS fragment reassembly rejects invalid channels instead of combining them with channel A fragments.
- Fixed the README link to AIS message type coverage.

Sentence coverage: 59 NMEA sentence types + 3 AIS application-layer sentences.

## [0.7.3] — 2026-06-27

### Added
- ACN (Alert Command) sentence type — alert time, manufacturer, identifier, instance, command, and state

### Fixed
- Single-feature unit test runs no longer require unrelated RMC or PSKPDPT features.

NMEA sentence coverage: 58 → 59 types.

## [0.7.2] — 2026-06-20

### Added
- HSC (Heading Steering Command) sentence type — commanded heading true and magnetic, with optional status
- OSD (Own Ship Data) sentence type — heading, course, speed, set and drift with reference indicators

NMEA sentence coverage: 56 → 58 types.

## [0.7.1] — 2026-06-13

### Added
- MTA (Mean Temperature of Air) sentence type — air temperature with unit indicator
- VPW (Speed Measured Parallel to Wind) sentence type — speed parallel to wind (VMG to wind), in knots and m/s

NMEA sentence coverage: 54 → 56 types.

## [0.7.0] - 2026-06-01

### Breaking
- `FieldReader` / `FieldWriter` are no longer part of the public API (now `pub(crate)`) — they were unintended glob-re-export leakage.
- `FrameError` has a new variant `NonAsciiAddress` — exhaustive matches on `FrameError` must be updated.
- `Gsv.signal_id`: `Option<u8>` → `Option<char>` (hex signal IDs were silently dropped).
- `Rmc`: new field `nav_status: Option<char>` (NMEA 4.1).
- `Gsa`: new field `system_id: Option<char>` (NMEA 4.11).
- `Rmb.dest_lat`/`dest_lon`, `Bwc.lat`/`lon`, `Bwr.lat`/`lon`, `Bec.lat`/`lon`: `Option<f32>` → `Option<f64>` (coordinate precision).
- `NmeaEncodable`: removed `PROPRIETARY_ID` and `to_proprietary_sentence()`; added `const PROPRIETARY: bool`. Proprietary types now set `SENTENCE_TYPE` to the full address. Migrate `x.to_proprietary_sentence()` → `x.to_sentence("")`.
- `Pskpdpt.range_scale`: `Option<u32>` → `Option<f32>` (fractional range scales were silently dropped).

### Fixed
- `parse_frame` no longer panics on a non-ASCII address; returns `FrameError::NonAsciiAddress`.
- `parse_frame` now rejects malformed checksums (wrong length, non-hex characters); lowercase hex checksums (e.g. `*0e`) remain valid.
- `FieldReader`: `"NaN"` / `"inf"` / `"-inf"` float fields now yield `None` instead of `Some(NaN)` / `Some(inf)`.
- `FieldWriter`: non-finite float values emit an empty field (not `"NaN"`); `-0.0` normalizes to `"0"`.
- AIS Type 27 latitude/longitude were scaled by 10 instead of 600 (1/10 minute) — positions were corrupted/dropped.
- AIS Type 24 Part B `callsign` and `ship_type` were read at the wrong bit offsets (garbage output).
- AIS Type 9 `dte`/`assigned`/`raim` flags were read at the wrong bit offsets.
- AIS 6-bit armor gap characters (`0x58`–`0x5F`) are now rejected; `extract_i32` no longer overflows at `len == 32`.
- AIS fragment reassembly: concurrent multi-part messages on channels A and B with the same sequence ID no longer collide; duplicate continuation fragments are now silently ignored instead of discarding the in-progress assembly.
- AIS payloads with an out-of-range `fill_bits` value (> 5) are now rejected instead of being coerced to 0.
- AIS AtoN and safety-message names now preserve leading spaces; only trailing 6-bit padding (`@`) is stripped.
- GSV hex signal IDs and the RMC/GSA NMEA 4.x trailing fields were silently dropped on parse and lost on re-encode.
- `TXT` sentences whose free-text payload contains commas are now fully preserved on parse.
- Position sentences now encode coordinates with zero-padded degrees (`00454.5784`, not `454.5784`).
- Coordinate encoder now always emits a decimal point for whole-number values (`0.0` → `"0000.0"`, not `"0000"`), fixing compatibility with strict NMEA parsers.
- GSV `has_signal_id` heuristic no longer fires on an empty trailing field produced by a stray trailing comma.
- Removed the last panic-family macro (`unreachable!`) from library code.

### Changed
- `unsafe_code` is now `forbid` at the crate level; `panic!` / `expect` / `todo!` / `unreachable!` are denied in non-test builds. These invariants were already respected; they are now enforced by the compiler.

### Note
- NMEA sentence coverage is **54** (51 standard + 3 proprietary). Earlier changelog running tallies under-counted by 2; this is the authoritative total.

## [0.6.2] — 2026-05-28

### Added
- BEC (Bearing and Distance to Waypoint, Dead Reckoning) sentence type — UTC time, lat/lon, true and magnetic bearing, distance in nautical miles, waypoint identifier
- RTE (Routes) sentence type — sentence count, sequence number, mode (complete/working), route name, variable-length list of waypoint identifiers

NMEA sentence coverage expanded from 50 to 52 types.

## [0.6.1] — 2026-05-20

### Added
- ACK (Acknowledge Alarm) sentence type — alert identifier
- HBT (Heartbeat Supervision) sentence type — repeat interval, equipment operation status, sequential message identifier

NMEA sentence coverage expanded from 48 to 50 types.

## [0.6.0] — 2026-05-12

### Added
- VWT (True Wind Speed and Angle) sentence type — true wind angle (port/starboard), speed in knots, m/s, and km/h
- WCV (Waypoint Closure Velocity) sentence type — closure velocity in knots, waypoint identifier, mode indicator

NMEA sentence coverage expanded from 46 to 48 types.

## [0.5.10] — 2026-05-07

### Added
- TLL (Target Latitude and Longitude) sentence type — target number, lat/lon in DDMM format, name, UTC time, status (lost/acquiring/tracking), reference target flag
- TTM (Tracked Target Message) sentence type — target number, range, bearing, speed, course, CPA distance/time, speed units, name, status, acquisition type

NMEA sentence coverage expanded from 44 to 46 types.

## [0.5.9] — 2026-05-05

### Added
- AAM (Waypoint Arrival Alarm) sentence type — arrival circle entered, perpendicular passed, circle radius + unit, waypoint ID
- BWW (Bearing, Waypoint to Waypoint) sentence type — true/magnetic bearings + origin/destination waypoint IDs

NMEA sentence coverage expanded from 42 to 44 types.

## [0.5.8] — 2026-05-03

### Added
- BOD (Bearing Origin to Destination) sentence type — true/magnetic bearings + origin/destination waypoint IDs
- RSD (Radar System Data) sentence type — VRM, bearing lines, cursor position, range scale and display rotation

NMEA sentence coverage expanded from 40 to 42 types.

## [0.5.7] — 2026-05-01

### Added
- WPL (Waypoint Location) sentence type — waypoint lat/lon + identifier
- BWR (Bearing & Distance to Waypoint — Rhumb Line) sentence type — rhumb-line counterpart to the existing great-circle BWC
- `FieldReader::u16` / `i16` / `i32` and matching `FieldWriter::u16` / `i16` / `i32` methods, removing the need to round-trip through `f32` or manual `String::parse` for sentences with these ranges.

### Changed
- Deduplicated the three identical `#[cfg(any(feature = "..."))]` blocks in `src/lib.rs` into a single `nmea_item!` declarative macro. Adding a new sentence now requires editing one feature list, not three.
- `NmeaEncodable::SENTENCE_TYPE` and `PROPRIETARY_ID` are now `&'static str` (previously `&str` with elided lifetime). All impl sites use string literals so no downstream changes are needed.

NMEA sentence coverage expanded from 38 to 40 types.

## [0.5.6] — 2026-04-23

### Added
- VWR (Relative Wind Speed and Angle) sentence type — wind angle, L/R indicator, speed in knots/m/s/km·h
- DTM (Datum Reference) sentence type — local datum code, subdivision, lat/lon/altitude offsets, reference datum

### Fixed
- Bad checksum in DTM pynmeagps fixture (`*4F` → `*6F`) — would have caused DTM integration tests to fail on first run after the sentence was wired in.

NMEA sentence coverage expanded from 36 to 38 types.

## [0.5.5] — 2026-04-19

### Added
- MDA (Meteorological Composite) sentence type — barometric pressure, air/water temperature, humidity, dew point, wind direction and speed
- BWC (Bearing & Distance to Waypoint, Great Circle) sentence type

NMEA sentence coverage expanded from 34 to 36 types.

### Fixed
- `rpm` and `vdr` features were missing from the three `any(...)` cfg gates in `src/lib.rs`, making standalone `--features rpm` or `--features vdr` builds fail to expose `NmeaSentence` and `NmeaEncodable`

## [0.5.4] — 2026-04-19

### Changed
- Maintenance republish of 0.5.3 with no source changes.

## [0.5.3] — 2026-04-18

### Added
- RPM (Revolutions) sentence type
- VDR (Set and Drift) sentence type

## [0.5.2] — 2026-04-15

### Added
- APB (Autopilot Sentence B) sentence type
- XTE (Cross-Track Error, Measured) sentence type

## [0.5.1] — 2026-04-13

### Added
- GSA (GPS DOP and Active Satellites) sentence type
- GSV (Satellites in View) sentence type

NMEA sentence coverage expanded from 30 to 32 types.

## [0.5.0] — 2026-04-12

### Added
- Proprietary sentence support — `parse_frame` now detects `$P...` addresses per NMEA 0183, setting `talker = ""` and `sentence_type` to the full address (e.g. `"PASHR"`, `"PSKPDPT"`). Standard and proprietary dispatch paths are separate, preventing collisions.
- `NmeaEncodable::PROPRIETARY_ID` constant and `to_proprietary_sentence()` method
- Two-path `nmea_sentences!` macro (`standard:` / `proprietary:` sections)
- PASHR — Roll, Pitch, Heading (Ashtech/Trimble proprietary)
- PGRME — Garmin Estimated Position Error (proprietary)
- PSKPDPT — Skipper Depth (proprietary)
- `FragmentCollector` payload size limits: `MAX_PAYLOAD_SIZE = 256` chars and `MAX_FRAGMENTS = 5`, grounded in ITU-R M.1371-5 (1152-bit / 5-slot TDMA ceiling)

### Fixed
- Removed `unwrap()` in RTE waypoint parsing loop (replaced with `while let`)

### Changed
- CONTRIBUTING.md documents proprietary sentence workflow

NMEA sentence coverage expanded from 27 to 30 types (27 standard + 3 proprietary).

## [0.4.0] — 2026-04-11

### Added
- AIS Type 6 — Addressed Binary Message
- AIS Type 7 / 13 — Binary / Safety Acknowledge
- AIS Type 8 — Binary Broadcast Message
- AIS Type 9 — Standard SAR Aircraft Position Report
- AIS Type 11 — UTC/Date Response
- AIS Type 12 — Addressed Safety-Related Message
- AIS Type 15 — Interrogation

AIS coverage expanded from 9 to 16 message types.

## [0.3.1] — 2026-04-11

### Added
- THS (True Heading and Status) sentence type
- TXT (Text Transmission) sentence type
- `ddmm_to_decimal` / `decimal_to_ddmm` coordinate conversion helpers
- CONTRIBUTING.md, CHANGELOG.md, `.githooks/pre-commit`

## [0.3.0] — 2026-04-10

### Added
- AIS message types 4 (Base Station Report) and 27 (Long Range / Satellite AIS)
- Shared bit-extraction helpers extracted to `ais/messages/utils.rs`
- AIS message type coverage added to SENTENCES.md and README.md

### Changed
- Tests reorganized into `tests/nmea/` and `tests/ais/` subdirectories

## [0.2.3] — 2026-04-09

### Added
- RSA (Rudder Sensor Angle) and VLW (Distance Traveled through Water) sentence types

### Changed
- Dispatch macro refactored; integration tests cleaned up

## [0.2.2] — 2026-04-09

### Fixed
- Review findings: doc corrections, test naming, minor API clean-up

## [0.2.1] — 2026-04-09

### Added
- MTW (Mean Temperature of Water) sentence type
- VBW (Dual Ground/Water Speed) sentence type

## [0.2.0] — 2026-04-09

### Changed
- Introduced `NmeaEncodable` trait with a `to_sentence()` default method — replaces the previous per-struct boilerplate

## [0.1.5] — 2026-04-08

### Added
- XDR (Transducer Measurement) sentence type with `to_sentences()` for multi-transducer payloads

## [0.1.4] — 2026-04-08

### Added
- DBK (Depth Below Keel) sentence type
- ZDA (Time & Date) sentence type

## [0.1.3] — 2026-04-08

### Fixed
- `lib.rs` feature gates, test naming conventions, Wire-format doc corrections

## [0.1.2] — 2026-04-07

### Added
- GBS (GPS Satellite Fault Detection) sentence type
- GST (GPS Pseudorange Noise Statistics) sentence type

## [0.1.1] — 2026-04-07

### Added
- ROT (Rate of Turn) sentence type
- RMB (Recommended Minimum Navigation Information) sentence type
- Revamped integration test structure

## [0.1.0] — 2026-04-07

### Added
- Initial release
- NMEA 0183 bidirectional parser/encoder: 18 sentence types (DBT, DBS, DPT, GGA, GLL, GNS, GST, HDG, HDM, HDT, MTW, MWD, MWV, RMC, VHW, VTG, VLW, ZDA)
- AIS decoder: Types 1/2/3 (Class A), 5 (Voyage), 14 (Safety Broadcast), 18 (Class B), 19 (Class B+), 21 (AtoN), 24 (Class B Static)
- Shared frame layer: `parse_frame` / `encode_frame`, IEC 61162-450 tag block support
- Zero external dependencies
- CI: tests, clippy, rustfmt, doc checks on stable + MSRV 1.85.0

[0.8.3]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.8.3
[0.8.2]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.8.2
[0.8.1]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.8.1
[0.8.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.8.0
[0.7.6]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.7.6
[0.7.5]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.7.5
[0.7.4]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.7.4
[0.7.3]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.7.3
[0.7.2]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.7.2
[0.7.1]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.7.1
[0.7.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.7.0
[0.6.2]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.6.2
[0.6.1]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.6.1
[0.6.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.6.0
[0.5.10]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.10
[0.5.9]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.9
[0.5.8]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.8
[0.5.7]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.7
[0.5.6]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.6
[0.5.5]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.5
[0.5.4]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.4
[0.5.3]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.3
[0.5.2]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.2
[0.5.1]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.1
[0.5.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.0
[0.4.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.4.0
[0.3.1]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.3.1
[0.3.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.3.0
[0.2.3]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.2.3
[0.2.2]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.2.2
[0.2.1]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.2.1
[0.2.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.2.0
[0.1.5]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.5
[0.1.4]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.4
[0.1.3]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.3
[0.1.2]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.2
[0.1.1]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.1
[0.1.0]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.1.0
