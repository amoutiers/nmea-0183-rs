# NMEA 0183 Sentence Coverage

Cette matrice distingue les sentence formatters publiés par les six éditions de l’IEC 61162-1, des formats legacy, propriétaires et AIS.

Les coches indiquent un support actuellement présent dans nmea-kit. `HTC` et `HTD` sont comptés séparément bien qu’ils soient regroupés dans une même rubrique du sommaire IEC.

| Code | Version IEC 61162-1 | Année |
| --- | --- | --- |
| E1 | 1.0 | 1995 |
| E2 | 2.0 | 2000 |
| E3 | 3.0 | 2007 |
| E4 | 4.0 | 2010 |
| E5 | 5.0 | 2016 |
| E6 | 6.0 | 2024 |

**Inférence vérifiable :** les colonnes E1 et E3 sont reconstituées à partir des TOC E2/E4 et des deltas explicitement publiés par les éditions suivantes.

Sources des TOC et deltas : [IEC 61162-1:1995](https://webstore.iec.ch/en/publication/19259), [IEC 61162-1:2000](https://webstore.iec.ch/en/publication/19260), [IEC 61162-1:2007](https://standards.iteh.ai/catalog/standards/iec/2bf36711-dd76-4e3e-b19c-1b46bbf88495/iec-61162-1-2007), [IEC 61162-1:2010](https://webstore.iec.ch/en/publication/4709), [IEC 61162-1:2016](https://webstore.iec.ch/en/publication/25754) et [IEC 61162-1:2024](https://webstore.iec.ch/en/publication/72729).

## Current status

- **66** des **133** formatters IEC 61162-1:2024 sont supportés.
- **67** formatters IEC 61162-1:2024 restent à intégrer.
- **7** formatters legacy supplémentaires sont supportés hors de la liste IEC 2024.
- Les features Cargo individuelles sont toutes représentées dans les tableaux ci-dessous.

**Cargo features**

- `default` : active par défaut, équivalente à `nmea` + `ais`.
- `nmea` : toutes les sentences individuelles de la section NMEA, y compris les propriétaires.
- `ais` : les sentences AIS `ABM` et `BBM`, ainsi que le décodage des messages AIS.
- `positioning` : `GGA`, `GLL`, `RMC`, `GNS`.
- `speed` : `VTG`, `VHW`, `VBW`, `RMC`, `RPM`, `VDR`.
- `heading` : `HDG`, `HDM`, `HDT`, `THS`.
- `wind` : `MWD`, `MWV`.
- `depth` : `DBT`, `DBS`, `DBK`, `DPT`.

## NMEA Sentences

La colonne « Éditions IEC » indique dans quelles éditions le formatter figure. Les formatters historiques retirés restent dans cette matrice.

| Formatter | Description | Support | Éditions IEC |
| --- | --- | :---: | --- |
| `AAM` | Waypoint arrival alarm | ✓ | E1–E6 |
| `ABK` | AIS addressed and binary broadcast acknowledgement |  | E3–E6 |
| `ABM` | AIS addressed binary and safety-related message | ✓ | E3–E6 |
| `ACA` | AIS channel assignment |  | E3–E6 |
| `ACK` | Acknowledge alarm | ✓ | E2–E6 |
| `ACN` | Alert command | ✓ | E5–E6 |
| `ACS` | AIS channel management information source |  | E3–E6 |
| `AGL` | Alert group list |  | E6 |
| `AIR` | AIS interrogation request |  | E3–E6 |
| `AKD` | Acknowledge detail alarm condition |  | E3–E6 |
| `ALA` | Report detailed alarm condition | ✓ | E3–E6 |
| `ALC` | Cyclic alert list | ✓ | E5–E6 |
| `ALF` | Alert sentence | ✓ | E5–E6 |
| `ALM` | GPS almanac data |  | E1–E3 |
| `ALR` | Set alarm state | ✓ | E2–E6 |
| `APB` | Heading/track controller (autopilot) sentence B | ✓ | E1–E6 |
| `ARC` | Alert command refused | ✓ | E5–E6 |
| `ASD` | Autopilot system data |  | E1 |
| `BBM` | AIS broadcast binary message | ✓ | E3–E6 |
| `BEC` | Bearing and distance to waypoint, dead reckoning | ✓ | E1–E6 |
| `BOD` | Bearing origin to destination | ✓ | E1–E6 |
| `BWC` | Bearing and distance to waypoint, great circle | ✓ | E1–E6 |
| `BWR` | Bearing and distance to waypoint, rhumb line | ✓ | E1–E6 |
| `BWW` | Bearing waypoint to waypoint | ✓ | E1–E6 |
| `CUR` | Water current layer, multi-layer water current data |  | E3–E6 |
| `DBT` | Depth below transducer | ✓ | E1–E6 |
| `DCN` | Decca position |  | E1–E3 |
| `DDC` | Display dimming control |  | E3–E6 |
| `DOR` | Door status detection | ✓ | E3–E6 |
| `DPT` | Depth | ✓ | E1–E6 |
| `DSC` | Digital selective calling information | ✓ | E1–E6 |
| `DSE` | Expanded digital selective calling | ✓ | E2–E6 |
| `DSI` | DSC transponder initiate |  | E2–E3 |
| `DSR` | DSC transponder response |  | E2–E3 |
| `DTM` | Datum reference | ✓ | E1–E6 |
| `EPM` | Command or report long equipment property value |  | E6 |
| `EPV` | Command or report equipment property value |  | E5–E6 |
| `ETL` | Engine telegraph operation status |  | E3–E6 |
| `EVE` | General event message | ✓ | E3–E6 |
| `FIR` | Fire detection | ✓ | E3–E6 |
| `FSI` | Frequency set information |  | E1–E6 |
| `GBS` | GNSS satellite fault detection | ✓ | E1–E6 |
| `GDC` | GNSS differential correction |  | E6 |
| `GEN` | Generic binary information |  | E3–E6 |
| `GFA` | GNSS fix accuracy and integrity |  | E4–E6 |
| `GGA` | Global positioning system (GPS) fix data | ✓ | E1–E6 |
| `GLC` | Geographic position, LORAN-C |  | E1–E3 |
| `GLL` | Geographic position, latitude/longitude | ✓ | E1–E6 |
| `GMP` | GNSS map projection fix data |  | E3 |
| `GNS` | GNSS fix data | ✓ | E2–E6 |
| `GRS` | GNSS range residuals |  | E1–E6 |
| `GSA` | GNSS DOP and active satellites | ✓ | E1–E6 |
| `GST` | GNSS pseudorange error statistics | ✓ | E1–E6 |
| `GSV` | GNSS satellites in view | ✓ | E1–E6 |
| `GXA` | TRANSIT position |  | E1 |
| `HBT` | Heartbeat supervision sentence | ✓ | E4–E6 |
| `HCR` | Heading correction report |  | E5–E6 |
| `HDG` | Heading, deviation and variation | ✓ | E1–E6 |
| `HDT` | Heading true | ✓ | E1–E6 |
| `HMR` | Heading monitor receive |  | E2–E6 |
| `HMS` | Heading monitor set |  | E2–E6 |
| `HRM` | Heel angle, roll period and roll amplitude measurement device |  | E5–E6 |
| `HSC` | Heading steering command | ✓ | E1–E6 |
| `HSS` | Hull stress surveillance systems |  | E3–E6 |
| `HTC` | Heading/track control command |  | E2–E6 |
| `HTD` | Heading/track control data |  | E2–E6 |
| `LCD` | LORAN-C signal data |  | E1–E3 |
| `LR1` | AIS long-range reply sentence 1 |  | E3–E6 |
| `LR2` | AIS long-range reply sentence 2 |  | E3–E6 |
| `LR3` | AIS long-range reply sentence 3 |  | E3–E6 |
| `LRF` | AIS long-range function |  | E3–E6 |
| `LRI` | AIS long-range interrogation |  | E3–E6 |
| `MLA` | GLONASS almanac data |  | E2–E3 |
| `MOB` | Man over board notification |  | E5–E6 |
| `MSK` | MSK receiver interface |  | E1–E6 |
| `MSS` | MSK receiver signal status |  | E1–E6 |
| `MTW` | Water temperature | ✓ | E1–E6 |
| `MWD` | Wind direction and speed | ✓ | E2–E6 |
| `MWV` | Wind speed and angle | ✓ | E1–E6 |
| `NAK` | Negative acknowledgement |  | E4–E6 |
| `NLS` | Navigation light status |  | E6 |
| `NRM` | NAVTEX receiver mask |  | E3–E6 |
| `NRX` | MSI received message |  | E3–E6 |
| `NSR` | Navigation status report |  | E5–E6 |
| `OLN` | OMEGA lane numbers |  | E1 |
| `OSD` | Own ship data | ✓ | E1–E6 |
| `POS` | Device position and ship dimensions report or configuration command |  | E4–E6 |
| `PRC` | Propulsion remote control status |  | E3–E6 |
| `RLM` | Return link message |  | E5–E6 |
| `RMA` | Recommended minimum specific LORAN-C data |  | E1–E6 |
| `RMB` | Recommended minimum navigation information | ✓ | E1–E6 |
| `RMC` | Recommended minimum specific GNSS data | ✓ | E1–E6 |
| `ROR` | Rudder order status |  | E3–E6 |
| `ROT` | Rate of turn | ✓ | E1–E6 |
| `RPM` | Revolutions | ✓ | E1–E6 |
| `RRT` | Report route transfer |  | E5–E6 |
| `RSA` | Rudder sensor angle | ✓ | E1–E6 |
| `RSD` | Radar system data | ✓ | E1–E6 |
| `RTE` | Routes | ✓ | E1–E6 |
| `SEL` | Selection report |  | E6 |
| `SFI` | Scanning frequency information |  | E1–E6 |
| `SLM` | Steering location/mode |  | E6 |
| `SM1` | SafetyNET message, all ships/NavArea |  | E5–E6 |
| `SM2` | SafetyNET message, coastal warning area |  | E5–E6 |
| `SM3` | SafetyNET message, circular area address |  | E5–E6 |
| `SM4` | SafetyNET message, rectangular area address |  | E5–E6 |
| `SMB` | IMO SafetyNET message body |  | E5–E6 |
| `SMV` | SafetyNET message, vessel in distress information |  | E6 |
| `SPW` | Security password sentence |  | E5–E6 |
| `SSD` | AIS ship static data |  | E3–E6 |
| `STN` | Multiple data ID |  | E1–E6 |
| `THS` | True heading and status | ✓ | E3–E6 |
| `TLB` | Target label | ✓ | E2–E6 |
| `TLL` | Target latitude and longitude | ✓ | E1–E6 |
| `TRC` | Thruster control data |  | E3–E6 |
| `TRD` | Thruster response data |  | E3–E6 |
| `TRF` | TRANSIT fix data |  | E1 |
| `TRL` | AIS transmitter non-functioning log |  | E5–E6 |
| `TTD` | Tracked target data | ✓ | E4–E6 |
| `TTM` | Tracked target message | ✓ | E1–E6 |
| `TUT` | Transmission of multi-language text |  | E3–E6 |
| `TXT` | Text transmission | ✓ | E2–E6 |
| `UID` | User identification code transmission |  | E3–E6 |
| `VBC` | Water-referenced and ground-referenced docking speed data |  | E6 |
| `VBW` | Dual ground/water speed | ✓ | E1–E6 |
| `VDM` | AIS VHF data-link message | ✓ | E3–E6 |
| `VDO` | AIS VHF data-link own-vessel report | ✓ | E3–E6 |
| `VDR` | Set and drift | ✓ | E1–E6 |
| `VER` | Version |  | E4–E6 |
| `VHW` | Water speed and heading | ✓ | E1–E6 |
| `VLW` | Dual ground/water distance | ✓ | E1–E6 |
| `VPW` | Speed measured parallel to wind | ✓ | E1–E6 |
| `VSD` | AIS voyage static data | ✓ | E3–E6 |
| `VTG` | Course over ground and ground speed | ✓ | E1–E6 |
| `WAT` | Water level detection |  | E3–E6 |
| `WCV` | Waypoint closure velocity | ✓ | E1–E6 |
| `WNC` | Distance waypoint to waypoint |  | E1–E6 |
| `WPL` | Waypoint location | ✓ | E1–E6 |
| `XDR` | Transducer measurements | ✓ | E1–E6 |
| `XTE` | Cross-track error, measured | ✓ | E1–E6 |
| `XTR` | Cross-track error, dead reckoning |  | E1–E6 |
| `ZDA` | Time and date | ✓ | E1–E6 |
| `ZDL` | Time and distance to variable point |  | E1–E6 |
| `ZFO` | UTC and time from origin waypoint |  | E1–E6 |
| `ZTG` | UTC and time to destination waypoint |  | E1–E6 |

## Legacy hors TOC IEC

| Formatter | Description | Support |
| --- | --- | :---: |
| `DBK` | Depth below keel | ✓ |
| `DBS` | Depth below surface | ✓ |
| `HDM` | Heading, magnetic | ✓ |
| `MDA` | Meteorological composite | ✓ |
| `MTA` | Air temperature | ✓ |
| `VWR` | Relative wind speed and angle | ✓ |
| `VWT` | True wind speed and angle | ✓ |

## Proprietary
- [x] PASHR — RT300 Inertial Attitude (Hemisphere/Ashtech)
- [x] PCDIN — NMEA 2000 via SeaSmart bridge
- [x] PGRME — Garmin Estimated Position Error
- [x] PGRMT — Garmin Sensor Status
- [x] PHTRO — Vessel Pitch and Roll (Xsens)
- [x] PKLDS — Keel Depth Sentence
- [x] PKLID — Keel ID
- [x] PKLSH — Keel Shallow
- [x] PKNDS — Knudsen Depth
- [x] PKNID — Knudsen ID
- [x] PKNSH — Knudsen Shallow
- [x] PKWDWPL — Waypoint (unknown vendor)
- [x] PMTK — MediaTek GPS Command
- [x] PRDID — RDI Doppler Instrument Data
- [x] PSKPDPT — Skipper Depth
- [x] PSONCMS — SMC IMU Data (SBG Systems)

## AIS

### Sentences

- [x] ABM — Addressed Binary Message
- [x] BBM — Broadcast Binary Message

### Message Types (decoded from AIVDM/AIVDO; selected types also encoded)

- [x] Type 1 — Class A Position Report (under way, engine) [decode + encode]
- [x] Type 2 — Class A Position Report (at anchor) [decode + encode]
- [x] Type 3 — Class A Position Report (special maneuver) [decode + encode]
- [x] Type 4 — Base Station Report (UTC + position) [decode + encode]
- [x] Type 5 — Static and Voyage Related Data (Class A) [decode + encode]
- [x] Type 6 — Addressed Binary Message
- [x] Type 7 — Binary Acknowledge
- [x] Type 8 — Binary Broadcast Message
- [x] Type 9 — Standard SAR Aircraft Position Report [decode + encode]
- [x] Type 10 — UTC/Date Inquiry
- [x] Type 11 — UTC/Date Response [decode + encode]
- [x] Type 12 — Addressed Safety-Related Message [decode + encode]
- [x] Type 13 — Safety-Related Acknowledge
- [x] Type 14 — Safety-Related Broadcast Message [decode + encode]
- [x] Type 15 — Interrogation
- [x] Type 16 — Assignment Mode Command
- [x] Type 17 — DGNSS Broadcast Binary Message
- [x] Type 18 — Standard Class B Position Report [decode + encode]
- [x] Type 19 — Extended Class B Position Report [decode + encode]
- [x] Type 20 — Data Link Management
- [x] Type 21 — Aid-to-Navigation Report [decode + encode]
- [x] Type 22 — Channel Management
- [x] Type 23 — Group Assignment Command
- [x] Type 24 — Static Data Report (Class B) [decode + encode]
- [x] Type 25 — Single Slot Binary Message
- [x] Type 26 — Multiple Slot Binary Message
- [x] Type 27 — Long Range Position Report (satellite AIS) [decode + encode]
