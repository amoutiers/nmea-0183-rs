#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 3 ]; then
  echo "usage: $0 TAG VERSION CHANGELOG" >&2
  exit 2
fi

tag=$1
version=$2
changelog=$3
[ "$tag" = "v$version" ] || { echo "tag must be v$version" >&2; exit 1; }

awk -v version="$version" '
  BEGIN { prefix = "## [" version "]" }
  index($0, prefix) == 1 {
    if (found++) { exit 1 }
    if ($0 !~ /[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]/) { exit 1 }
    active = 1
    next
  }
  active && /^## \[/ { active = 0 }
  active && /^\[/ { active = 0 }
  active { lines[++count] = $0; if ($0 ~ /^-[[:space:]]+/) content = 1 }
  END {
    if (found != 1 || !content) exit 1
    start = 1
    while (start <= count && lines[start] ~ /^[[:space:]]*$/) start++
    while (count && lines[count] ~ /^[[:space:]]*$/) count--
    for (i = start; i <= count; i++) print lines[i]
  }
' "$changelog" || { echo "missing, duplicate, dated, or non-empty changelog section for $version" >&2; exit 1; }
