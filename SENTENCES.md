# NMEA 0183 Sentence Coverage

Checked = supported by nmea-kit.
Sources : go-nmea, gpsd.

## NMEA Sentences

### Position

- [x] DTM — Datum Reference
- [x] GGA — Global Positioning System Fix Data
- [x] GLL — Geographic Position, Latitude/Longitude
- [x] GNS — GNSS Fix Data
- [ ] GTD — Geographic Location in Time Differences
- [ ] RMA — Recommended Minimum Navigation Information
- [x] RMC — Recommended Minimum Navigation Information

### Satellites

- [x] GBS — GPS Satellite Fault Detection
- [ ] GRS — GPS Range Residuals
- [x] GSA — GPS DOP and Active Satellites
- [x] GST — GPS Pseudorange Noise Statistics
- [x] GSV — Satellites in View

### Heading

- [x] HDG — Heading, Deviation & Variation
- [x] HDM — Heading, Magnetic
- [x] HDT — Heading, True
- [x] THS — True Heading and Status

### Course & Speed

- [x] OSD — Own Ship Data
- [x] VBW — Dual Ground/Water Speed
- [x] VHW — Water Speed and Heading
- [x] VLW — Distance Traveled through Water
- [x] VPW — Speed, Measured Parallel to Wind
- [x] VTG — Track Made Good and Ground Speed

### Wind

- [x] MWD — Wind Direction & Speed
- [x] MWV — Wind Speed and Angle
- [x] VWR — Relative Wind Speed and Angle
- [x] VWT — True Wind Speed and Angle

### Depth

- [x] DBK — Depth Below Keel
- [x] DBS — Depth Below Surface
- [x] DBT — Depth Below Transducer
- [x] DPT — Depth of Water

### Steering

- [ ] APA — Autopilot Sentence "A"
- [x] APB — Autopilot Sentence "B"
- [x] HSC — Heading Steering Command
- [x] ROT — Rate of Turn
- [x] RSA — Rudder Sensor Angle

### Waypoints & Routes

- [x] AAM — Waypoint Arrival Alarm
- [x] BEC — Bearing & Distance to Waypoint, Dead Reckoning
- [x] BOD — Bearing, Waypoint to Waypoint
- [x] BWC — Bearing & Distance to Waypoint, Great Circle
- [x] BWR — Bearing & Distance to Waypoint, Rhumb Line
- [x] BWW — Bearing, Waypoint to Waypoint
- [ ] R00 — Waypoints in Active Route
- [x] RMB — Recommended Minimum Navigation Information (to waypoint)
- [x] RTE — Routes
- [x] WCV — Waypoint Closure Velocity
- [ ] WNC — Distance, Waypoint to Waypoint
- [x] WPL — Waypoint Location
- [x] XTE — Cross-Track Error, Measured
- [ ] XTR — Cross-Track Error, Dead Reckoning
- [ ] ZFO — UTC & Time from Origin Waypoint
- [ ] ZTG — UTC & Time to Destination Waypoint

### Environment

- [x] MDA — Meteorological Composite
- [x] MTA — Air Temperature
- [x] MTW — Mean Temperature of Water
- [x] XDR — Transducer Measurement

### Time

- [x] ZDA — Time & Date

### AIS Interface

- [x] VSD — AIS Voyage Static Data

### Targets

- [x] RSD — Radar System Data
- [ ] TLB — Target Label
- [x] TLL — Target Latitude and Longitude
- [ ] TTD — Tracked Target Data
- [x] TTM — Tracked Target Message

### Safety & Alarms

- [x] ACK — Acknowledge Alarm
- [x] ACN — Alert Command
- [x] ALA — Alert
- [x] ALC — Cyclic Alert List
- [x] ALF — Alert
- [x] ALR — Alert Response
- [x] ARC — Alert Response Command
- [x] DOR — Door Status
- [x] DSC — Digital Selective Calling Information
- [x] DSE — DSC Extended
- [x] EVE — Event
- [x] FIR — Fire Detection
- [x] HBT — Heartbeat Supervision

### Vessel Systems

- [x] RPM — Revolutions
- [x] VDR — Set and Drift

### Communication

- [ ] ALM — GPS Almanac Data
- [ ] FSI — Frequency Set Information
- [ ] MSK — Control for a Beacon Receiver
- [ ] MSS — Beacon Receiver Status
- [ ] RLM — Return Link Message
- [ ] SFI — Scanning Frequency Information
- [ ] STN — Multiple Data ID
- [x] TXT — Text Transmission

### Trawl / Fishing

- [ ] HFB — Trawl Headrope to Footrope and Bottom
- [ ] ITS — Trawl Door Spread 2 Distance
- [ ] TDS — Trawl Door Spread Distance
- [ ] TFI — Trawl Filling Indicator
- [ ] TPC — Trawl Position Cartesian Coordinates
- [ ] TPR — Trawl Position Relative Vessel
- [ ] TPT — Trawl Position True

### Proprietary

- [x] PASHR — RT300 Inertial Attitude (Hemisphere/Ashtech)
- [ ] PCDIN — NMEA 2000 via SeaSmart bridge
- [x] PGRME — Garmin Estimated Position Error
- [ ] PGRMT — Garmin Sensor Status
- [ ] PHTRO — Vessel Pitch and Roll (Xsens)
- [ ] PKLDS — Keel Depth Sentence
- [ ] PKLID — Keel ID
- [ ] PKLSH — Keel Shallow
- [ ] PKNDS — Knudsen Depth
- [ ] PKNID — Knudsen ID
- [ ] PKNSH — Knudsen Shallow
- [ ] PKWDWPL — Waypoint (unknown vendor)
- [ ] PMTK — MediaTek GPS Command
- [ ] PRDID — RDI Doppler Instrument Data
- [x] PSKPDPT — Skipper Depth
- [ ] PSONCMS — SMC IMU Data (SBG Systems)

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
