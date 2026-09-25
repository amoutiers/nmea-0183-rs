# Unwired sentence drafts

These files are reference material for NMEA sentence types that the crate does not yet support. They are not compiled or tested, and their APIs and fixtures may be out of date.

For each sentence added to the crate, move its implementation from `sentences/` to `src/nmea/sentences/` and its tests from `tests/` to `tests/nmea/`. Adapt both to the current API, validate the fixtures, and follow `CONTRIBUTING.md`. Remove the draft pair in the same change.
