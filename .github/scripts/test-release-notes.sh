#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
valid="$tmp/valid.md"
printf '%s\n' '## [0.8.7] — 2026-09-20' '' '### Fixed' '- Correct output.' '' '[0.8.7]: link' > "$valid"
expected=$'### Fixed\n- Correct output.'
[ "$(bash "$root/.github/scripts/release-notes.sh" v0.8.7 0.8.7 "$valid")" = "$expected" ]
for case in tag missing empty duplicate; do
  file="$tmp/$case.md"
  case "$case" in
    tag) cp "$valid" "$file"; ! bash "$root/.github/scripts/release-notes.sh" v0.8.8 0.8.7 "$file" ;;
    missing) printf '%s\n' '## [0.8.6] — 2026-09-19' '- old' > "$file"; ! bash "$root/.github/scripts/release-notes.sh" v0.8.7 0.8.7 "$file" ;;
    empty) printf '%s\n' '## [0.8.7] — 2026-09-20' '' '### Fixed' > "$file"; ! bash "$root/.github/scripts/release-notes.sh" v0.8.7 0.8.7 "$file" ;;
    duplicate) printf '%s\n' '## [0.8.7] — 2026-09-20' '- one' '## [0.8.7] — 2026-09-20' '- two' > "$file"; ! bash "$root/.github/scripts/release-notes.sh" v0.8.7 0.8.7 "$file" ;;
  esac
done
