# NMEA 0183 Sentence Coverage

Cette matrice distingue les sentence formatters publiés par les six éditions de l’IEC 61162-1, des formats legacy, propriétaires et AIS.

Les coches indiquent un support actuellement présent dans nmea-kit. `HTC` et `HTD` sont comptés séparément bien qu’ils soient regroupés dans une même rubrique du sommaire IEC.

**Inférence vérifiable :** les colonnes des éditions 1 et 3 sont reconstituées à partir des TOC des éditions 2 et 4, ainsi que des deltas explicitement publiés par les éditions suivantes.

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

Le support reflète `nmea-kit` aujourd’hui. Chaque colonne donne la version NMEA et l’année IEC entre parenthèses. La dernière colonne est la version actuelle.

### Position

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `DTM` | Datum reference | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `GGA` | Global positioning system (GPS) fix data | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `GLL` | Geographic position, latitude/longitude | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `GNS` | GNSS fix data | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `RMB` | Recommended minimum navigation information | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `RMC` | Recommended minimum specific GNSS data | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `POS` | Device position and ship dimensions report or configuration command |  |  |  |  | ✓ | ✓ | ✓ |
| `RMA` | Recommended minimum specific LORAN-C data |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `DCN` | Decca position |  | ✓ | ✓ | ✓ |  |  |  |
| `GLC` | Geographic position, LORAN-C |  | ✓ | ✓ | ✓ |  |  |  |
| `GMP` | GNSS map projection fix data |  |  |  | ✓ |  |  |  |
| `GXA` | TRANSIT position |  | ✓ |  |  |  |  |  |
| `OLN` | OMEGA lane numbers |  | ✓ |  |  |  |  |  |
| `TRF` | TRANSIT fix data |  | ✓ |  |  |  |  |  |

### Satellites

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `GBS` | GNSS satellite fault detection | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `GSA` | GNSS DOP and active satellites | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `GST` | GNSS pseudorange error statistics | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `GSV` | GNSS satellites in view | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `GDC` | GNSS differential correction |  |  |  |  |  |  | ✓ |
| `GFA` | GNSS fix accuracy and integrity |  |  |  |  | ✓ | ✓ | ✓ |
| `GRS` | GNSS range residuals |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `ALM` | GPS almanac data |  | ✓ | ✓ | ✓ |  |  |  |
| `MLA` | GLONASS almanac data |  |  | ✓ | ✓ |  |  |  |

### Heading

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `HDG` | Heading, deviation and variation | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `HDT` | Heading true | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `HSC` | Heading steering command | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `THS` | True heading and status | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |
| `HCR` | Heading correction report |  |  |  |  |  | ✓ | ✓ |
| `HMR` | Heading monitor receive |  |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `HMS` | Heading monitor set |  |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `HRM` | Heel angle, roll period and roll amplitude measurement device |  |  |  |  |  | ✓ | ✓ |

### Course & Speed

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `OSD` | Own ship data | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `ROT` | Rate of turn | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `VBW` | Dual ground/water speed | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `VDR` | Set and drift | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `VHW` | Water speed and heading | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `VLW` | Dual ground/water distance | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `VPW` | Speed measured parallel to wind | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `VTG` | Course over ground and ground speed | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `WCV` | Waypoint closure velocity | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `VBC` | Water-referenced and ground-referenced docking speed data |  |  |  |  |  |  | ✓ |

### Wind

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `MWD` | Wind direction and speed | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `MWV` | Wind speed and angle | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### Depth

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `DBT` | Depth below transducer | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `DPT` | Depth | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### Steering

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `APB` | Heading/track controller (autopilot) sentence B | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `RPM` | Revolutions | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `RSA` | Rudder sensor angle | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `EPM` | Command or report long equipment property value |  |  |  |  |  |  | ✓ |
| `EPV` | Command or report equipment property value |  |  |  |  |  | ✓ | ✓ |
| `ETL` | Engine telegraph operation status |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `HTC` | Heading/track control command |  |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `HTD` | Heading/track control data |  |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `PRC` | Propulsion remote control status |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `ROR` | Rudder order status |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `TRC` | Thruster control data |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `TRD` | Thruster response data |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `ASD` | Autopilot system data |  | ✓ |  |  |  |  |  |

### Waypoints & Routes

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `AAM` | Waypoint arrival alarm | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `BEC` | Bearing and distance to waypoint, dead reckoning | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `BOD` | Bearing origin to destination | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `BWC` | Bearing and distance to waypoint, great circle | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `BWR` | Bearing and distance to waypoint, rhumb line | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `BWW` | Bearing waypoint to waypoint | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `RTE` | Routes | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `WPL` | Waypoint location | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `XTE` | Cross-track error, measured | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `RRT` | Report route transfer |  |  |  |  |  | ✓ | ✓ |
| `WNC` | Distance waypoint to waypoint |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `XTR` | Cross-track error, dead reckoning |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `ZDL` | Time and distance to variable point |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `ZFO` | UTC and time from origin waypoint |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `ZTG` | UTC and time to destination waypoint |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### Environment

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `MTW` | Water temperature | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `XDR` | Transducer measurements | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `CUR` | Water current layer, multi-layer water current data |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `WAT` | Water level detection |  |  |  | ✓ | ✓ | ✓ | ✓ |

### Time

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `ZDA` | Time and date | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### AIS Interface

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `ABM` | AIS addressed binary and safety-related message | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |
| `BBM` | AIS broadcast binary message | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |
| `VDM` | AIS VHF data-link message | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |
| `VDO` | AIS VHF data-link own-vessel report | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |
| `VSD` | AIS voyage static data | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |
| `ABK` | AIS addressed and binary broadcast acknowledgement |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `ACA` | AIS channel assignment |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `ACS` | AIS channel management information source |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `AIR` | AIS interrogation request |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `LR1` | AIS long-range reply sentence 1 |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `LR2` | AIS long-range reply sentence 2 |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `LR3` | AIS long-range reply sentence 3 |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `LRF` | AIS long-range function |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `LRI` | AIS long-range interrogation |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `SSD` | AIS ship static data |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `TRL` | AIS transmitter non-functioning log |  |  |  |  |  | ✓ | ✓ |

### Targets

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `RSD` | Radar system data | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `TLB` | Target label | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `TLL` | Target latitude and longitude | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `TTD` | Tracked target data | ✓ |  |  |  | ✓ | ✓ | ✓ |
| `TTM` | Tracked target message | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### Safety & Alarms

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `ACK` | Acknowledge alarm | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `ACN` | Alert command | ✓ |  |  |  |  | ✓ | ✓ |
| `ALA` | Report detailed alarm condition | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |
| `ALC` | Cyclic alert list | ✓ |  |  |  |  | ✓ | ✓ |
| `ALF` | Alert sentence | ✓ |  |  |  |  | ✓ | ✓ |
| `ALR` | Set alarm state | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `ARC` | Alert command refused | ✓ |  |  |  |  | ✓ | ✓ |
| `DOR` | Door status detection | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |
| `DSC` | Digital selective calling information | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `DSE` | Expanded digital selective calling | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `EVE` | General event message | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |
| `FIR` | Fire detection | ✓ |  |  | ✓ | ✓ | ✓ | ✓ |
| `HBT` | Heartbeat supervision sentence | ✓ |  |  |  | ✓ | ✓ | ✓ |
| `AGL` | Alert group list |  |  |  |  |  |  | ✓ |
| `AKD` | Acknowledge detail alarm condition |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `MOB` | Man over board notification |  |  |  |  |  | ✓ | ✓ |
| `NAK` | Negative acknowledgement |  |  |  |  | ✓ | ✓ | ✓ |
| `NSR` | Navigation status report |  |  |  |  |  | ✓ | ✓ |
| `SM1` | SafetyNET message, all ships/NavArea |  |  |  |  |  | ✓ | ✓ |
| `SM2` | SafetyNET message, coastal warning area |  |  |  |  |  | ✓ | ✓ |
| `SM3` | SafetyNET message, circular area address |  |  |  |  |  | ✓ | ✓ |
| `SM4` | SafetyNET message, rectangular area address |  |  |  |  |  | ✓ | ✓ |
| `SMB` | IMO SafetyNET message body |  |  |  |  |  | ✓ | ✓ |
| `SMV` | SafetyNET message, vessel in distress information |  |  |  |  |  |  | ✓ |
| `DSI` | DSC transponder initiate |  |  | ✓ | ✓ |  |  |  |
| `DSR` | DSC transponder response |  |  | ✓ | ✓ |  |  |  |

### Vessel Systems

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `DDC` | Display dimming control |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `GEN` | Generic binary information |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `HSS` | Hull stress surveillance systems |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `NLS` | Navigation light status |  |  |  |  |  |  | ✓ |
| `SEL` | Selection report |  |  |  |  |  |  | ✓ |
| `SLM` | Steering location/mode |  |  |  |  |  |  | ✓ |
| `STN` | Multiple data ID |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### Communication

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| `TXT` | Text transmission | ✓ |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `FSI` | Frequency set information |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `MSK` | MSK receiver interface |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `MSS` | MSK receiver signal status |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `NRM` | NAVTEX receiver mask |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `NRX` | MSI received message |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `RLM` | Return link message |  |  |  |  |  | ✓ | ✓ |
| `SFI` | Scanning frequency information |  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `SPW` | Security password sentence |  |  |  |  |  | ✓ | ✓ |
| `TUT` | Transmission of multi-language text |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `UID` | User identification code transmission |  |  |  | ✓ | ✓ | ✓ | ✓ |
| `VER` | Version |  |  |  |  | ✓ | ✓ | ✓ |
| `LCD` | LORAN-C signal data |  | ✓ | ✓ | ✓ |  |  |  |

### Trawl / Fishing

| Formatter | Description | Support | NMEA 2.1<br>(1995) | NMEA 2.30<br>(2000) | NMEA 3.01<br>(2007) | NMEA 4.00<br>(2010) | NMEA 4.10<br>(2016) | NMEA 6<br>(2024) |
| --- | --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |

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
