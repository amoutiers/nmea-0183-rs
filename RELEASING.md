# Releasing nmea-0183-rs

Templates for CHANGELOG entries and GitHub release descriptions. Keep them in sync — the GH release body should mirror the CHANGELOG entry, not invent new wording.

## Workflow

1. While developing, add bullets under `## [Unreleased]` in `CHANGELOG.md`.
2. Cutting a release:
   - Bump version in `Cargo.toml`.
   - Rename `[Unreleased]` → `[X.Y.Z] — YYYY-MM-DD` (date = day you actually publish, not when you wrote the bullets).
   - Add the version link footnote at the bottom.
   - Commit, tag `vX.Y.Z`, push tag.
   - `gh release create vX.Y.Z` with the body filled from the template below (paste the entry, drop the date heading).
3. **Do not run `cargo publish` locally.** The crate is published to crates.io automatically via [GitHub Trusted Publishing](https://crates.io/docs/trusted-publishing) when the tag is pushed. Monitor the GitHub Actions workflow after pushing the tag.

## CHANGELOG entry template

```markdown
## [Unreleased]

### Added
- <FOO> (<Full Sentence Name>) sentence type — <one-line field summary>

### Changed
- <Brief description of behavior change, with rationale if non-obvious>

### Fixed
- <Bug description> — <impact: what would break / who would notice>

### Removed
- <What was removed and why>

NMEA sentence coverage: <prev> → <new> types.   <!-- only if the count changed -->
```

Rules:
- Sections in fixed order: **Added, Changed, Fixed, Removed**. Omit empty sections.
- One bullet = one logical change. Don't bundle.
- For sentence additions: `<TLA> (<Full Name>) — <field summary>`. No field count unless the field-count form is used consistently in the same release.
- For fixes: state the **impact**, not just the symptom. "Bad checksum in DTM fixture" → say *what would have broken* if shipped.
- Avoid internal jargon. A user reading the CHANGELOG without the codebase open should understand each line.
- Convert relative dates to absolute (`YYYY-MM-DD`).

## GitHub release body template

Paste the version's CHANGELOG section, minus the `## [X.Y.Z] — DATE` heading. Add a trailing link to the CHANGELOG and the crates.io release.

```markdown
## Added
- ...

## Changed
- ...

## Fixed
- ...

NMEA sentence coverage: <prev> → <new> types.

---

📜 [Full changelog](https://github.com/amoutiers/nmea-0183-rs/blob/master/CHANGELOG.md#xyz--YYYY-MM-DD)
📦 [crates.io](https://crates.io/crates/nmea-0183-rs/X.Y.Z)
```

Rules:
- The body should NOT contradict the CHANGELOG. If you reword on GitHub, update the CHANGELOG too.
- No "would have caused" or other ambiguous backreferences — name the affected sentence/feature explicitly.
- Don't introduce new bullets here that aren't in the CHANGELOG.

## Version link footnote (bottom of CHANGELOG)

After cutting each release, add:

```markdown
[X.Y.Z]: https://github.com/amoutiers/nmea-0183-rs/releases/tag/vX.Y.Z
```

Newest version on top. Every entry heading must have a corresponding footnote.

## Worked example

CHANGELOG entry:

```markdown
## [0.5.7] — 2026-05-01

### Changed
- Moved 47 unwired sentence files from `src/nmea/sentences/` to a gitignored `drafts/` directory. They were never compiled and were drifting from `FieldReader`/`FieldWriter` API changes.
- Deduplicated the three identical `cfg(any(feature = "..."))` blocks in `src/lib.rs` into a single `nmea_item!` macro. Adding a new sentence now requires editing one feature list, not three.

### Added
- `FieldReader::u16` / `i16` / `i32` and matching `FieldWriter` methods.
- `NmeaEncodable::SENTENCE_TYPE` is now `&'static str` (was `&str`).

[0.5.7]: https://github.com/amoutiers/nmea-kit/releases/tag/v0.5.7
```

GitHub release body:

```markdown
## Changed
- Moved 47 unwired sentence files from `src/nmea/sentences/` to a gitignored `drafts/` directory. They were never compiled and were drifting from `FieldReader`/`FieldWriter` API changes.
- Deduplicated the three identical `cfg(any(feature = "..."))` blocks in `src/lib.rs` into a single `nmea_item!` macro. Adding a new sentence now requires editing one feature list, not three.

## Added
- `FieldReader::u16` / `i16` / `i32` and matching `FieldWriter` methods.
- `NmeaEncodable::SENTENCE_TYPE` is now `&'static str` (was `&str`).

---

📜 [Full changelog](https://github.com/amoutiers/nmea-kit/blob/master/CHANGELOG.md#057--2026-05-01)
📦 [crates.io](https://crates.io/crates/nmea-kit/0.5.7)
```
