# nmea-0183-rs

## Release 0.9.1 snapshot

- 87 NMEA formatters: 71 standard and 16 proprietary; two AIS application sentences.
- 974 tests passed across the all-features unit, integration, and doctest targets.

## Agent instructions

- Read [Cargo.toml](Cargo.toml) before changing package metadata or features.
- Inspect the nearest sentence, AIS message, encoder, and test before changing a parser or formatter. Reuse the local `FieldReader`/`FieldWriter` and bit-level patterns where they apply.
- Follow [CONTRIBUTING.md](CONTRIBUTING.md) for the TDD workflow, sentence checklist, fixtures, test conventions, code style, and API policy.
- Check the relevant feature-gated tests after a change, then run the documented validation proportionate to its scope.
- Do not infer public signatures or wire behavior from this file. Read source and rustdoc before relying on them.

## References

- [README.md](README.md): package overview, quick start, public examples, and unreleased migration guidance.
- [SENTENCES.md](SENTENCES.md): complete NMEA/AIS coverage matrix, feature categories, and protocol references.
- [CONTRIBUTING.md](CONTRIBUTING.md): contribution procedure, tests, style, and API policy.
- [Cargo.toml](Cargo.toml): effective package metadata and feature membership.
- [src/lib.rs](src/lib.rs): public exports; read the corresponding source and rustdoc for exact API contracts.
- [RELEASING.md](RELEASING.md): release workflow and release-note templates.
