# NMEA 0183 Sentence Coverage

Cette matrice distingue les sentence formatters publiés par les six éditions de l’IEC 61162-1, des formats legacy, propriétaires et AIS.

Les coches indiquent un support actuellement présent dans nmea-kit. `HTC` et `HTD` sont comptés séparément bien qu’ils soient regroupés dans une même rubrique du sommaire IEC.

Sources des TOC et deltas : [IEC 61162-1:1995](https://webstore.iec.ch/en/publication/19259), [IEC 61162-1:2000](https://webstore.iec.ch/en/publication/19260), [IEC 61162-1:2007](https://standards.iteh.ai/catalog/standards/iec/2bf36711-dd76-4e3e-b19c-1b46bbf88495/iec-61162-1-2007), [IEC 61162-1:2010](https://webstore.iec.ch/en/publication/4709), [IEC 61162-1:2016](https://webstore.iec.ch/en/publication/25754) et [IEC 61162-1:2024](https://webstore.iec.ch/en/publication/72729). Chronologie des versions NMEA : [NMEA 0183](https://en.wikipedia.org/wiki/NMEA_0183#Revisions).

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

Le support reflète `nmea-kit` aujourd’hui. Les deux bornes donnent les première et dernière versions NMEA observées dans les TOC IEC disponibles, avec leur année de publication. Elles ne prouvent pas une présence continue entre ces points.

**Inférence vérifiable :** les bornes qui passent par 2.10 ou 3.01 reposent sur les TOC IEC reconstitués des éditions 1 et 3, à partir des TOC voisins et des deltas publiés.

### Position

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `DTM` | Datum reference | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `GGA` | Global positioning system (GPS) fix data | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `GLL` | Geographic position, latitude/longitude | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `GNS` | GNSS fix data | ✓ | 2.30<br>(1998) | 4.30<br>(2023) |
| `RMB` | Recommended minimum navigation information | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `RMC` | Recommended minimum specific GNSS data | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `POS` | Device position and ship dimensions report or configuration command |   | 4.00<br>(2008) | 4.30<br>(2023) |
| `RMA` | Recommended minimum specific LORAN-C data |   | 2.10<br>(1995) | 4.30<br>(2023) |
| `DCN` | Decca position |   | 2.10<br>(1995) | 3.01<br>(2002) |
| `GLC` | Geographic position, LORAN-C |   | 2.10<br>(1995) | 3.01<br>(2002) |
| `GMP` | GNSS map projection fix data |   | 3.01<br>(2002) | 3.01<br>(2002) |
| `GXA` | TRANSIT position |   | 2.10<br>(1995) | 2.10<br>(1995) |
| `OLN` | OMEGA lane numbers |   | 2.10<br>(1995) | 2.10<br>(1995) |
| `TRF` | TRANSIT fix data |   | 2.10<br>(1995) | 2.10<br>(1995) |

### Satellites

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `GBS` | GNSS satellite fault detection | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `GSA` | GNSS DOP and active satellites | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `GST` | GNSS pseudorange error statistics | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `GSV` | GNSS satellites in view | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `GDC` | GNSS differential correction |   | 4.30<br>(2023) | 4.30<br>(2023) |
| `GFA` | GNSS fix accuracy and integrity |   | 4.00<br>(2008) | 4.30<br>(2023) |
| `GRS` | GNSS range residuals |   | 2.10<br>(1995) | 4.30<br>(2023) |
| `ALM` | GPS almanac data |   | 2.10<br>(1995) | 3.01<br>(2002) |
| `MLA` | GLONASS almanac data |   | 2.30<br>(1998) | 3.01<br>(2002) |

### Heading

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `HDG` | Heading, deviation and variation | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `HDT` | Heading true | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `HSC` | Heading steering command | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `THS` | True heading and status | ✓ | 3.01<br>(2002) | 4.30<br>(2023) |
| `HCR` | Heading correction report |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `HMR` | Heading monitor receive |   | 2.30<br>(1998) | 4.30<br>(2023) |
| `HMS` | Heading monitor set |   | 2.30<br>(1998) | 4.30<br>(2023) |
| `HRM` | Heel angle, roll period and roll amplitude measurement device |   | 4.10<br>(2012) | 4.30<br>(2023) |

### Course & Speed

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `OSD` | Own ship data | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `ROT` | Rate of turn | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `VBW` | Dual ground/water speed | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `VDR` | Set and drift | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `VHW` | Water speed and heading | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `VLW` | Dual ground/water distance | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `VPW` | Speed measured parallel to wind | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `VTG` | Course over ground and ground speed | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `WCV` | Waypoint closure velocity | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `VBC` | Water-referenced and ground-referenced docking speed data |   | 4.30<br>(2023) | 4.30<br>(2023) |

### Wind

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `MWD` | Wind direction and speed | ✓ | 2.30<br>(1998) | 4.30<br>(2023) |
| `MWV` | Wind speed and angle | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |

### Depth

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `DBT` | Depth below transducer | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `DPT` | Depth | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |

### Steering

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `APB` | Heading/track controller (autopilot) sentence B | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `RPM` | Revolutions | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `RSA` | Rudder sensor angle | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `EPM` | Command or report long equipment property value |   | 4.30<br>(2023) | 4.30<br>(2023) |
| `EPV` | Command or report equipment property value |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `ETL` | Engine telegraph operation status |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `HTC` | Heading/track control command |   | 2.30<br>(1998) | 4.30<br>(2023) |
| `HTD` | Heading/track control data |   | 2.30<br>(1998) | 4.30<br>(2023) |
| `PRC` | Propulsion remote control status |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `ROR` | Rudder order status |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `TRC` | Thruster control data |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `TRD` | Thruster response data |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `ASD` | Autopilot system data |   | 2.10<br>(1995) | 2.10<br>(1995) |

### Waypoints & Routes

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `AAM` | Waypoint arrival alarm | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `BEC` | Bearing and distance to waypoint, dead reckoning | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `BOD` | Bearing origin to destination | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `BWC` | Bearing and distance to waypoint, great circle | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `BWR` | Bearing and distance to waypoint, rhumb line | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `BWW` | Bearing waypoint to waypoint | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `RTE` | Routes | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `WPL` | Waypoint location | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `XTE` | Cross-track error, measured | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `RRT` | Report route transfer |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `WNC` | Distance waypoint to waypoint |   | 2.10<br>(1995) | 4.30<br>(2023) |
| `XTR` | Cross-track error, dead reckoning |   | 2.10<br>(1995) | 4.30<br>(2023) |
| `ZDL` | Time and distance to variable point |   | 2.10<br>(1995) | 4.30<br>(2023) |
| `ZFO` | UTC and time from origin waypoint |   | 2.10<br>(1995) | 4.30<br>(2023) |
| `ZTG` | UTC and time to destination waypoint |   | 2.10<br>(1995) | 4.30<br>(2023) |

### Environment

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `MTW` | Water temperature | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `XDR` | Transducer measurements | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `CUR` | Water current layer, multi-layer water current data |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `WAT` | Water level detection |   | 3.01<br>(2002) | 4.30<br>(2023) |

### Time

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `ZDA` | Time and date | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |

### AIS Interface

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `ABM` | AIS addressed binary and safety-related message | ✓ | 3.01<br>(2002) | 4.30<br>(2023) |
| `BBM` | AIS broadcast binary message | ✓ | 3.01<br>(2002) | 4.30<br>(2023) |
| `VDM` | AIS VHF data-link message | ✓ | 3.01<br>(2002) | 4.30<br>(2023) |
| `VDO` | AIS VHF data-link own-vessel report | ✓ | 3.01<br>(2002) | 4.30<br>(2023) |
| `VSD` | AIS voyage static data | ✓ | 3.01<br>(2002) | 4.30<br>(2023) |
| `ABK` | AIS addressed and binary broadcast acknowledgement |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `ACA` | AIS channel assignment |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `ACS` | AIS channel management information source |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `AIR` | AIS interrogation request |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `LR1` | AIS long-range reply sentence 1 |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `LR2` | AIS long-range reply sentence 2 |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `LR3` | AIS long-range reply sentence 3 |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `LRF` | AIS long-range function |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `LRI` | AIS long-range interrogation |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `SSD` | AIS ship static data |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `TRL` | AIS transmitter non-functioning log |   | 4.10<br>(2012) | 4.30<br>(2023) |

### Targets

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `RSD` | Radar system data | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `TLB` | Target label | ✓ | 2.30<br>(1998) | 4.30<br>(2023) |
| `TLL` | Target latitude and longitude | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `TTD` | Tracked target data | ✓ | 4.00<br>(2008) | 4.30<br>(2023) |
| `TTM` | Tracked target message | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |

### Safety & Alarms

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `ACK` | Acknowledge alarm | ✓ | 2.30<br>(1998) | 4.30<br>(2023) |
| `ACN` | Alert command | ✓ | 4.10<br>(2012) | 4.30<br>(2023) |
| `ALA` | Report detailed alarm condition | ✓ | 3.01<br>(2002) | 4.30<br>(2023) |
| `ALC` | Cyclic alert list | ✓ | 4.10<br>(2012) | 4.30<br>(2023) |
| `ALF` | Alert sentence | ✓ | 4.10<br>(2012) | 4.30<br>(2023) |
| `ALR` | Set alarm state | ✓ | 2.30<br>(1998) | 4.30<br>(2023) |
| `ARC` | Alert command refused | ✓ | 4.10<br>(2012) | 4.30<br>(2023) |
| `DOR` | Door status detection | ✓ | 3.01<br>(2002) | 4.30<br>(2023) |
| `DSC` | Digital selective calling information | ✓ | 2.10<br>(1995) | 4.30<br>(2023) |
| `DSE` | Expanded digital selective calling | ✓ | 2.30<br>(1998) | 4.30<br>(2023) |
| `EVE` | General event message | ✓ | 3.01<br>(2002) | 4.30<br>(2023) |
| `FIR` | Fire detection | ✓ | 3.01<br>(2002) | 4.30<br>(2023) |
| `HBT` | Heartbeat supervision sentence | ✓ | 4.00<br>(2008) | 4.30<br>(2023) |
| `AGL` | Alert group list |   | 4.30<br>(2023) | 4.30<br>(2023) |
| `AKD` | Acknowledge detail alarm condition |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `MOB` | Man over board notification |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `NAK` | Negative acknowledgement |   | 4.00<br>(2008) | 4.30<br>(2023) |
| `NSR` | Navigation status report |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `SM1` | SafetyNET message, all ships/NavArea |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `SM2` | SafetyNET message, coastal warning area |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `SM3` | SafetyNET message, circular area address |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `SM4` | SafetyNET message, rectangular area address |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `SMB` | IMO SafetyNET message body |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `SMV` | SafetyNET message, vessel in distress information |   | 4.30<br>(2023) | 4.30<br>(2023) |
| `DSI` | DSC transponder initiate |   | 2.30<br>(1998) | 3.01<br>(2002) |
| `DSR` | DSC transponder response |   | 2.30<br>(1998) | 3.01<br>(2002) |

### Vessel Systems

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `DDC` | Display dimming control |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `GEN` | Generic binary information |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `HSS` | Hull stress surveillance systems |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `NLS` | Navigation light status |   | 4.30<br>(2023) | 4.30<br>(2023) |
| `SEL` | Selection report |   | 4.30<br>(2023) | 4.30<br>(2023) |
| `SLM` | Steering location/mode |   | 4.30<br>(2023) | 4.30<br>(2023) |
| `STN` | Multiple data ID |   | 2.10<br>(1995) | 4.30<br>(2023) |

### Communication

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |
| `TXT` | Text transmission | ✓ | 2.30<br>(1998) | 4.30<br>(2023) |
| `FSI` | Frequency set information |   | 2.10<br>(1995) | 4.30<br>(2023) |
| `MSK` | MSK receiver interface |   | 2.10<br>(1995) | 4.30<br>(2023) |
| `MSS` | MSK receiver signal status |   | 2.10<br>(1995) | 4.30<br>(2023) |
| `NRM` | NAVTEX receiver mask |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `NRX` | MSI received message |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `RLM` | Return link message |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `SFI` | Scanning frequency information |   | 2.10<br>(1995) | 4.30<br>(2023) |
| `SPW` | Security password sentence |   | 4.10<br>(2012) | 4.30<br>(2023) |
| `TUT` | Transmission of multi-language text |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `UID` | User identification code transmission |   | 3.01<br>(2002) | 4.30<br>(2023) |
| `VER` | Version |   | 4.00<br>(2008) | 4.30<br>(2023) |
| `LCD` | LORAN-C signal data |   | 2.10<br>(1995) | 3.01<br>(2002) |

### Trawl / Fishing

| Formatter | Description | Support | Première version NMEA<br>(année) | Dernière version NMEA<br>(année) |
| --- | --- | :---: | :---: | :---: |

Aucun formatter de cette rubrique n’est publié dans les six éditions de l’IEC 61162-1.
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
