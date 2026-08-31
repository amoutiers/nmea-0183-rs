# nmea-kit

Bidirectional NMEA 0183 parser/encoder with AIS decoding and transponder-message encoding, written in Rust.

| | |
| --- | --- |
| **Crate** | `nmea-kit` |
| **Version** | 0.8.3 |
| **MSRV** | 1.85.0 |
| **Edition** | 2024 |
| **Dependencies** | 0 |
| **License** | MIT OR Apache-2.0 |
| **NMEA sentences** | 70 (bidirectional: parse + encode) |
| **AIS application sentences** | 2 (bidirectional: parse + encode) |
| **AIS message types** | All numeric Types 1-27 decoded; Types 1/2/3, 4, 5, 9, 11, 12, 14, 18, 19, 21, 24 and 27 also encoded |

- **Shared frame layer** — handles `$` (NMEA) and `!` (AIS) framing, IEC 61162-450 tag blocks
- **No `nom`, no proc-macro** — `FieldReader`/`FieldWriter` helpers for clean sequential parsing

## Quick start

### Parse an NMEA sentence

```rust
use nmea_kit::{parse_frame, NmeaSentence};

let frame = parse_frame("$IIMWD,046.,T,046.,M,10.1,N,05.2,M*43").unwrap();
let sentence = NmeaSentence::parse(&frame);

match sentence {
    NmeaSentence::Mwd(mwd) => {
        println!("True wind dir: {:?}°", mwd.wind_dir_true);
        println!("Wind speed: {:?} kts", mwd.wind_speed_kts);
    }
    _ => {}
}
```

### Encode and send an NMEA sentence

```rust
use nmea_kit::NmeaEncodable;
use nmea_kit::nmea::sentences::Dbt;

let dbt = Dbt {
    depth_feet: Some(7.7),
    depth_meters: Some(2.3),
    depth_fathoms: Some(1.3),
};

let sentence = dbt.to_sentence("SD").expect("valid depth sentence");
// "$SDDBT,7.7,f,2.3,M,1.3,F*05\r\n"
```

### Decode AIS messages

```rust
use nmea_kit::parse_frame;
use nmea_kit::ais::{AisParser, AisMessage};

let mut parser = AisParser::new();
let frame = parse_frame("!AIVDM,1,1,,A,13aEOK?P00PD2wVMdLDRhgvL289?,0*26").unwrap();

if let Some(AisMessage::Position(pos)) = parser.decode(&frame) {
    println!("MMSI: {}, lat: {:?}, lon: {:?}", pos.mmsi, pos.latitude, pos.longitude);
}
```

### Encode an AIS transponder message

```rust
use nmea_kit::ais::messages::NavigationStatus;
use nmea_kit::ais::transmit::{
    AisChannel, AisEncodable, AisTransmitOptions, ClassAPosition, ClassAPositionType,
};

let report = ClassAPosition {
    message_type: ClassAPositionType::PositionReport,
    repeat_indicator: 0,
    mmsi: 244_670_316,
    navigation_status: NavigationStatus::UnderWayEngine,
    rate_of_turn: None,
    sog: Some(10.0),
    position_accuracy: true,
    longitude: Some(4.379_285),
    latitude: Some(51.894_750),
    cog: Some(70.6),
    heading: Some(71),
    timestamp: Some(5),
    maneuver_indicator: 0,
    raim: false,
    communication_state: 0,
};

let sentences = report
    .to_sentences(AisTransmitOptions::vdm(AisChannel::A))
    .expect("valid Type 1 report");
// `sentences` contains complete, checksummed !AIVDM lines ready for the simulator.
```

`AisTransmitOptions::vdo()` emits `!AIVDO` instead. Messages that require several fragments,
including Type 5 and long Type 12/14 safety text, require `.with_sequence_id(0..=9)`. Every
emitted line is at most 82 characters, including the checksum and CRLF terminator. The crate
constructs sentences only: the simulator owns cadence, TDMA access, and reactions to Type 15.

### Encode an AIS application-layer sentence

```rust
use nmea_kit::ais::sentences::Abm;

let abm = Abm {
    num_frags: Some(1),
    frag_num: Some(1),
    msg_id: Some(0),
    mmsi: Some(123456789),
    channel: Some('1'),
    vdl_msg_num: Some(6),
    payload: Some("testpayload".to_string()),
    fill_bits: Some(0),
};

let sentence = abm.to_sentence("AI").expect("valid AIS sentence");
// "!AIABM,1,1,0,123456789,1,6,testpayload,0*08\r\n"
```

## Architecture

```mermaid
flowchart TD
    raw["raw line"] --> pf["parse_frame()"]
    pf --> frame["NmeaFrame\nprefix · talker · sentence_type · fields"]
    frame --> known["$ + known type (incl. VSD)"]
    frame --> unknown["$ + unknown type"]
    frame --> ais_in["! AIVDM/AIVDO"]
    frame --> ais_sentence["! AIS app sentence"]
    known --> typed["Typed struct\nMwd, Rmc…"]
    unknown --> raw_fields["Raw fields\npass-through"]
    ais_in --> ais_msg["AisMessage enum\nAll numeric Types 1-27"]
    ais_sentence --> ais_typed["AIS sentence struct\nAbm, Bbm"]
```

**Frame layer** validates checksum, strips tag blocks, extracts talker ID and sentence type. Shared by both NMEA and AIS.

**NMEA content** uses `FieldReader`/`FieldWriter` for sequential field parsing and encoding. Each sentence type is a standalone struct with `parse()`, `encode()`, and `to_sentence()`. Parsing is lenient: `parse()` always returns `Some` for known types, mapping missing or malformed fields to `None`. This is intentional for marine instruments that often produce partial data.

**AIS content** decodes AIVDM/AIVDO 6-bit ASCII armor into a bitstream, handles multi-fragment reassembly, and extracts typed fields. `ais::transmit` encodes complete `!AIVDM` or `!AIVDO` lines for Types 1/2/3, 4, 5, 9, 11, 12, 14, 18, 19, 21, 24 and 27. It owns sentence fragmentation, while the simulator remains responsible for choosing its emission cadence. The `!`-prefixed AIS application sentences ABM and BBM live under `ais::sentences`. VSD is a conventional NMEA sentence (`$--VSD`) exposed under `nmea::sentences`.

## Supported types

### NMEA 0183 sentences (bidirectional) — [full coverage list](SENTENCES.md)

| Category           | Sentences             |
| ------------------ | --------------------- |
| Position           | DTM, RMC, GGA, GLL, GNS |
| Satellites         | GBS, GSA, GSV, GST    |
| Wind               | MWD, MWV, VPW, VWR, VWT    |
| Heading            | HDT, HDG, HDM, THS    |
| Course & Speed     | RPM, VBW, VDR, VLW, VTG, VHW    |
| Depth              | DPT, DBT, DBS, DBK    |
| Steering           | HSC, ROT, RSA         |
| Environment        | MDA, MTA, MTW, XDR¹        |
| Waypoints & Routes | AAM, APB, BEC, BOD, BWC, BWR, BWW, RMB, RTE, WCV, WPL, XTE |
| Radar / Targets    | OSD, RSD, TLL, TTM    |
| Safety & Alarms    | ACK, ACN, ALA, ALC, ALF, ALR, ARC, DOR, DSC, DSE, EVE, FIR, HBT |
| AIS interface      | VSD (`$--VSD`)        |
| Communication      | TXT                   |
| Time               | ZDA                   |
| Proprietary        | PASHR, PGRME, PSKPDPT |

¹ `Xdr` has an additional `to_sentences() -> Result<Vec<String>, EncodeError>` method that automatically splits many measurements into multiple sentences to stay within the 82-character NMEA line limit.

### AIS application sentences (bidirectional)

| Sentences |
| --------- |
| ABM, BBM  |

### AIS messages — [full type list](SENTENCES.md#ais)

| Type(s) | Decoded struct      | Encoded model | Description                                          |
| ------- | ------------------- | ------------- | ---------------------------------------------------- |
| 1, 2, 3 | `PositionReport`    | `ClassAPosition` | Class A position report                           |
| 4       | `BaseStationReport` | `BaseStation` | Base station UTC + position                          |
| 5       | `StaticVoyageData`  | `ClassAStaticVoyage` | Static and voyage data (Class A)                |
| 6       | `BinaryAddressed`   |               | Addressed binary message (DAC/FID + data)            |
| 7, 13   | `BinaryAck`         |               | Binary / safety acknowledge                          |
| 8       | `BinaryBroadcast`   |               | Binary broadcast message (DAC/FID + data)            |
| 9       | `SarAircraftReport` | `SarAircraft` | Standard SAR aircraft position                       |
| 10      | `UtcDateInquiry`    |               | UTC/date inquiry                                     |
| 11      | `UtcDateResponse`   | `UtcDateResponse` | UTC/date response (mobile station)                |
| 12      | `SafetyAddressed`   | `SafetyAddressed` | Addressed safety-related message                   |
| 14      | `SafetyBroadcast`   | `SafetyBroadcast` | Safety-related broadcast message                   |
| 15      | `Interrogation`     |               | Interrogation (request data from vessel)             |
| 16      | `AssignmentModeCommand` |            | Assigned mode command                                |
| 17      | `DgnssBroadcast`    |               | DGNSS correction broadcast                           |
| 18      | `PositionReport`    | `ClassBPosition` | Class B standard position                         |
| 19      | `PositionReport`    | `ClassBExtendedPosition` | Class B+ extended position             |
| 20      | `DataLinkManagement` |              | FATDMA slot reservations                             |
| 21      | `AidToNavigation`   | `AidToNavigation` | Aid-to-navigation report (buoys, beacons)          |
| 22      | `ChannelManagement` |               | Channel management                                   |
| 23      | `GroupAssignment`   |               | Group assignment command                             |
| 24      | `StaticDataReport`  | `ClassBStaticPartA`, `ClassBStaticPartB` | Static data report (Class B) |
| 25      | `BinarySingleSlot`  |               | Single-slot binary message                           |
| 26      | `BinaryMultiSlot`   |               | Multiple-slot binary message                         |
| 27      | `LongRangePosition` | `LongRangePosition` | Long range position (satellite AIS, 1/10 minute precision) |

### Key improvements over existing crates

| Issue                  | `nmea` 0.7 / `ais` 0.12             | `nmea-kit`                               |
| ---------------------- | ----------------------------------- | ---------------------------------------- |
| NMEA sentence coverage | ~10 types, rest manual              | 70 NMEA types + 2 AIS application sentences |
| AIS message coverage   | ~5 types                            | All numeric Types 1-27                    |
| Encoding               | Read-only                           | All NMEA + Types 1/2/3, 4, 5, 9, 11, 12, 14, 18, 19, 21, 24, 27 |
| Error distinction      | Can't tell unsupported vs malformed | Frame errors vs content errors           |
| AIS lat/lon precision  | `f32` (11m error)                   | `f64`                                    |
| AIS sentinels          | 91/181/511 leak to caller           | Filtered to `None` at decode             |
| Tag blocks             | Manual stripping                    | Built into frame layer                   |
| Dependencies           | `nom` (AIS)                         | Zero                                     |

## Features

```toml
[dependencies]
nmea-kit = "0.8"
```

| Feature                                                                                                                                                                                                                                | Default    | Enables                    |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | -------------------------- |
| `nmea`                                                                                                                                                                                                         | yes        | All 70 NMEA sentence types |
| `ais`                                                                                                                                                                                                                                  | yes        | 24 AIS message decoders, transponder encoding, and ABM/BBM application sentences |
| `positioning`                                                                                                                                                                                                                          | via `nmea` | GGA, GLL, RMC, GNS         |
| `speed`                                                                                                                                                                                                                                | via `nmea` | VTG, VHW, VBW, RMC         |
| `heading`                                                                                                                                                                                                                              | via `nmea` | HDG, HDM, HDT, THS         |
| `wind`                                                                                                                                                                                                                                 | via `nmea` | MWD, MWV                   |
| `depth`                                                                                                                                                                                                                                | via `nmea` | DBT, DBS, DBK, DPT         |
| `aam`, `ack`, `acn`, `ala`, `alc`, `alf`, `alr`, `arc`, `apb`, `bec`, `bod`, `bwc`, `bwr`, `bww`, `dbk`, `dbs`, `dbt`, `dor`, `dpt`, `dsc`, `dse`, `dtm`, `eve`, `fir`, `gbs`, `gga`, `gll`, `gns`, `gsa`, `gsv`, `gst`, `hbt`, `hdg`, `hdm`, `hdt`, `hsc`, `mda`, `mta`, `mtw`, `mwd`, `mwv`, `osd`, `pashr`, `pgrme`, `pskpdpt`, `rmb`, `rmc`, `rot`, `rpm`, `rsa`, `rsd`, `rte`, `ths`, `tll`, `ttm`, `txt`, `vbw`, `vdr`, `vhw`, `vlw`, `vpw`, `vsd`, `vtg`, `vwr`, `vwt`, `wcv`, `wpl`, `xdr`, `xte`, `zda` | via `nmea` | Individual NMEA sentence types |
| `abm`, `bbm` | via `ais` | Individual AIS application-layer sentence types |

Use a group feature for common use cases:

```toml
# Only positioning sentences (GGA, GLL, RMC, GNS), no AIS
nmea-kit = { version = "0.8", default-features = false, features = ["positioning"] }
```

Cherry-pick individual sentences you need:

```toml
nmea-kit = { version = "0.8", default-features = false, features = ["rmc", "mwd"] }
```

NMEA-only (no AIS, all sentences):

```toml
nmea-kit = { version = "0.8", default-features = false, features = ["nmea"] }
```

## Coordinate conversion

NMEA sentences encode lat/lon as `DDMM.MMMM`; AIS uses decimal degrees. Two helpers bridge the gap:

```rust
use nmea_kit::nmea::{ddmm_to_decimal, decimal_to_ddmm};

// Parse a GGA latitude field: "4807.038" N → 48.1173°
let lat = ddmm_to_decimal(4807.038); // → 48.1173

// Encode back for a sentence
let ddmm = decimal_to_ddmm(48.1173); // → 4807.038
```

Apply the N/S / E/W sign separately (negate for S or W).

## Documentation

| File                               | Purpose                                                           |
| ---------------------------------- | ----------------------------------------------------------------- |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Getting started, TDD workflow, test rules, adding a sentence type |
| [SENTENCES.md](SENTENCES.md)       | Full NMEA / AIS coverage matrix                                   |
| [CHANGELOG.md](CHANGELOG.md)       | Release history                                                   |
| [AGENTS.md](AGENTS.md)             | API surface, struct fields, and patterns (optimized for LLMs)     |

## License

MIT OR Apache-2.0
