#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
valid="$tmp/valid.md"
printf '%s\n' '## [0.9.0] — 2026-09-21' '' '### Fixed' '- Correct output.' '' '[0.9.0]: link' > "$valid"
expected=$'### Fixed\n- Correct output.'
[ "$(bash "$root/.github/scripts/release-notes.sh" v0.9.0 0.9.0 "$valid")" = "$expected" ]
for case in tag missing empty duplicate; do
  file="$tmp/$case.md"
  case "$case" in
    tag) cp "$valid" "$file"; if bash "$root/.github/scripts/release-notes.sh" v0.9.1 0.9.0 "$file"; then exit 1; fi ;;
    missing) printf '%s\n' '## [0.8.9] — 2026-09-19' '- old' > "$file"; if bash "$root/.github/scripts/release-notes.sh" v0.9.0 0.9.0 "$file"; then exit 1; fi ;;
    empty) printf '%s\n' '## [0.9.0] — 2026-09-21' '' '### Fixed' > "$file"; if bash "$root/.github/scripts/release-notes.sh" v0.9.0 0.9.0 "$file"; then exit 1; fi ;;
    duplicate) printf '%s\n' '## [0.9.0] — 2026-09-21' '- one' '## [0.9.0] — 2026-09-21' '- two' > "$file"; if bash "$root/.github/scripts/release-notes.sh" v0.9.0 0.9.0 "$file"; then exit 1; fi ;;
  esac
done
