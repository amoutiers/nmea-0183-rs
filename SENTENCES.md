# NMEA 0183 Sentence Coverage

Cette liste distingue les sentence formatters approuvés par l’[IEC 61162-1:2024](https://webstore.iec.ch/en/publication/72729) des types legacy, propriétaires et AIS.

Les cases indiquent un support actuellement présent dans nmea-kit. La liste IEC reprend les 133 formatters de la section 8.3 de l’édition 6.0. `HTC` et `HTD` sont deux formatters regroupés dans une même rubrique de la norme.

Version de la couverture : `nmea-kit` 0.8.7, avec 85 sentences NMEA bidirectionnelles et 2 sentences AIS d’application.

Sources : [IEC 61162-1:2024](https://webstore.iec.ch/en/publication/72729), [go-nmea](https://github.com/adrianmo/go-nmea), [gpsd](https://gitlab.com/gpsd/gpsd), [SignalK](https://github.com/SignalK/signalk-parser-nmea0183) et [pynmeagps](https://github.com/semuconsulting/pynmeagps).

## Current status

- **66** des **133** formatters IEC 61162-1:2024 sont supportés.
- **67** formatters IEC 61162-1:2024 restent à intégrer.
- **7** formatters legacy supplémentaires sont supportés hors de la liste IEC 2024.
- Les features Cargo individuelles sont toutes représentées dans les rubriques ci-dessous.

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

### Position

- [x] DTM — Datum reference
- [x] GGA — Global positioning system (GPS) fix data
- [x] GLL — Geographic position, latitude/longitude
- [x] GNS — GNSS fix data
- [ ] POS — Device position and ship dimensions report or configuration command
- [ ] RMA — Recommended minimum specific LORAN-C data
- [x] RMB — Recommended minimum navigation information
- [x] RMC — Recommended minimum specific GNSS data

### Satellites

- [x] GBS — GNSS satellite fault detection
- [ ] GDC — GNSS differential correction
- [ ] GFA — GNSS fix accuracy and integrity
- [ ] GRS — GNSS range residuals
- [x] GSA — GNSS DOP and active satellites
- [x] GST — GNSS pseudorange error statistics
- [x] GSV — GNSS satellites in view

### Heading

- [ ] HCR — Heading correction report
- [x] HDG — Heading, deviation and variation
- [x] HDT — Heading true
- [ ] HMR — Heading monitor receive
- [ ] HMS — Heading monitor set
- [ ] HRM — Heel angle, roll period and roll amplitude measurement device
- [x] HSC — Heading steering command
- [x] THS — True heading and status
- [x] HDM — Heading, magnetic (legacy, hors IEC 61162-1:2024)

### Course & Speed

- [x] OSD — Own ship data
- [ ] VBC — Water-referenced and ground-referenced docking speed data
- [x] VBW — Dual ground/water speed
- [x] VDR — Set and drift
- [x] VHW — Water speed and heading
- [x] VLW — Dual ground/water distance
- [x] VPW — Speed measured parallel to wind
- [x] VTG — Course over ground and ground speed
- [x] WCV — Waypoint closure velocity
- [x] ROT — Rate of turn

### Wind

- [x] MWD — Wind direction and speed
- [x] MWV — Wind speed and angle
- [x] VWR — Relative wind speed and angle (legacy, hors IEC 61162-1:2024)
- [x] VWT — True wind speed and angle (legacy, hors IEC 61162-1:2024)

### Depth

- [x] DBT — Depth below transducer
- [x] DPT — Depth
- [x] DBS — Depth below surface (legacy, hors IEC 61162-1:2024)
- [x] DBK — Depth below keel (legacy, hors IEC 61162-1:2024)

### Steering

- [x] APB — Heading/track controller (autopilot) sentence B
- [ ] EPM — Command or report long equipment property value
- [ ] EPV — Command or report equipment property value
- [ ] ETL — Engine telegraph operation status
- [ ] HTC — Heading/track control command
- [ ] HTD — Heading/track control data
- [ ] PRC — Propulsion remote control status
- [ ] ROR — Rudder order status
- [x] RSA — Rudder sensor angle
- [x] RPM — Revolutions
- [ ] TRC — Thruster control data
- [ ] TRD — Thruster response data

### Waypoints & Routes

- [x] AAM — Waypoint arrival alarm
- [x] BEC — Bearing and distance to waypoint, dead reckoning
- [x] BOD — Bearing origin to destination
- [x] BWC — Bearing and distance to waypoint, great circle
- [x] BWR — Bearing and distance to waypoint, rhumb line
- [x] BWW — Bearing waypoint to waypoint
- [ ] RRT — Report route transfer
- [x] RTE — Routes
- [ ] WNC — Distance waypoint to waypoint
- [x] WPL — Waypoint location
- [x] XTE — Cross-track error, measured
- [ ] XTR — Cross-track error, dead reckoning
- [ ] ZFO — UTC and time from origin waypoint
- [ ] ZTG — UTC and time to destination waypoint
- [ ] ZDL — Time and distance to variable point

### Environment

- [ ] CUR — Water current layer, multi-layer water current data
- [x] MTW — Water temperature
- [ ] WAT — Water level detection
- [x] XDR — Transducer measurements
- [x] MDA — Meteorological composite (legacy, hors IEC 61162-1:2024)
- [x] MTA — Air temperature (legacy, hors IEC 61162-1:2024)

### Time

- [x] ZDA — Time and date

### AIS Interface

- [ ] ABK — AIS addressed and binary broadcast acknowledgement
- [x] ABM — AIS addressed binary and safety-related message
- [ ] ACA — AIS channel assignment
- [ ] ACS — AIS channel management information source
- [ ] AIR — AIS interrogation request
- [x] BBM — AIS broadcast binary message
- [ ] LR1 — AIS long-range reply sentence 1
- [ ] LR2 — AIS long-range reply sentence 2
- [ ] LR3 — AIS long-range reply sentence 3
- [ ] LRF — AIS long-range function
- [ ] LRI — AIS long-range interrogation
- [ ] SSD — AIS ship static data
- [ ] TRL — AIS transmitter non-functioning log
- [x] VDM — AIS VHF data-link message
- [x] VDO — AIS VHF data-link own-vessel report
- [x] VSD — AIS voyage static data

### Targets

- [x] RSD — Radar system data
- [x] TLB — Target label
- [x] TLL — Target latitude and longitude
- [x] TTD — Tracked target data
- [x] TTM — Tracked target message

### Safety & Alarms

- [x] ACK — Acknowledge alarm
- [x] ACN — Alert command
- [ ] AGL — Alert group list
- [ ] AKD — Acknowledge detail alarm condition
- [x] ALA — Report detailed alarm condition
- [x] ALC — Cyclic alert list
- [x] ALF — Alert sentence
- [x] ALR — Set alarm state
- [x] ARC — Alert command refused
- [x] DOR — Door status detection
- [x] DSC — Digital selective calling information
- [x] DSE — Expanded digital selective calling
- [x] EVE — General event message
- [x] FIR — Fire detection
- [x] HBT — Heartbeat supervision sentence
- [ ] MOB — Man over board notification
- [ ] NAK — Negative acknowledgement
- [ ] NSR — Navigation status report
- [ ] SM1 — SafetyNET message, all ships/NavArea
- [ ] SM2 — SafetyNET message, coastal warning area
- [ ] SM3 — SafetyNET message, circular area address
- [ ] SM4 — SafetyNET message, rectangular area address
- [ ] SMB — IMO SafetyNET message body
- [ ] SMV — SafetyNET message, vessel in distress information

### Vessel Systems

- [ ] DDC — Display dimming control
- [ ] GEN — Generic binary information
- [ ] HSS — Hull stress surveillance systems
- [ ] SEL — Selection report
- [ ] SLM — Steering location/mode
- [ ] STN — Multiple data ID
- [ ] NLS — Navigation light status

### Communication

- [ ] FSI — Frequency set information
- [ ] MSK — MSK receiver interface
- [ ] MSS — MSK receiver signal status
- [ ] NRM — NAVTEX receiver mask
- [ ] NRX — MSI received message
- [ ] RLM — Return link message
- [ ] SFI — Scanning frequency information
- [ ] SPW — Security password sentence
- [x] TXT — Text transmission
- [ ] TUT — Transmission of multi-language text
- [ ] UID — User identification code transmission
- [ ] VER — Version

### Trawl / Fishing

Aucun formatter de cette rubrique n’est publié dans la section 8.3 de l’IEC 61162-1:2024.

### Proprietary

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
