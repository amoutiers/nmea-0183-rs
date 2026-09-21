# Releasing nmea-0183-rs

Templates for CHANGELOG entries and GitHub release descriptions. Keep them in sync — the GH release body should mirror the CHANGELOG entry, not invent new wording.

## First publication

`nmea-0183-rs` is a new crates.io crate name. The first publication must be
made manually because crates.io cannot configure Trusted Publishing for a crate
that does not exist yet.

1. Confirm that the crates.io account has a verified email address.
2. Run `cargo publish --dry-run --locked` from the clean release commit.
3. Publish `0.9.0` manually with a crates.io API token using `cargo login`, then
   run `cargo publish --locked`.
4. Revoke or remove the token after publication.
5. In the crates.io crate settings, add GitHub Actions as a Trusted Publisher:
   repository `amoutiers/nmea-0183-rs`, workflow `publish.yml`, with no
   environment configured.
6. Verify the published crate and docs.rs page.
7. Push the `v0.9.0` tag. CI will run, skip the duplicate crates.io upload for
   this first release, and create the GitHub Release.

Publication is permanent. Never commit, paste into an issue, or store a crates.io
token in the repository.

## Workflow

1. While developing, add bullets under `## [Unreleased]` in `CHANGELOG.md`.
2. Cutting a release:
   - Bump version in `Cargo.toml`.
   - Rename `[Unreleased]` → `[X.Y.Z] — YYYY-MM-DD` (date = day you actually publish, not when you wrote the bullets).
   - Add the version link footnote at the bottom.
   - Commit, tag `vX.Y.Z`, push tag.
3. For versions after the first publication, do not run `cargo publish` locally.
   The tag validates its version and dated CHANGELOG section, then the workflow
   publishes to crates.io and creates the GitHub Release. Monitor the GitHub
   Actions workflow after pushing the tag. Recover manually only after
   inspecting whether publication or release creation already completed.

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

[0.5.7]: https://github.com/amoutiers/nmea-0183-rs/releases/tag/v0.5.7
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

📜 [Full changelog](https://github.com/amoutiers/nmea-0183-rs/blob/master/CHANGELOG.md#057--2026-05-01)
📦 [crates.io](https://crates.io/crates/nmea-0183-rs/0.5.7)
```
