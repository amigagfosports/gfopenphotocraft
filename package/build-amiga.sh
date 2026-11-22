#!/bin/sh
# Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/.." && pwd)
out=${OUT:-"$root/build-amiga-package"}
archive=GFOpenPhotoCraft-0.1.0-rc1-68040
stage=$(mktemp -d "${TMPDIR:-/tmp}/gfopenphotocraft-package.XXXXXX")
trap 'rm -rf "$stage"' EXIT HUP INT TERM

drawer="$stage/$archive/PhotoCraft"
mkdir -p "$out" "$drawer"
OUT="$drawer" "$root/ports/amigaos/build-amiga.sh"

install -m 0644 "$root/LICENSE-MIT" "$drawer/LICENSE-MIT"
install -m 0644 "$root/LICENSE-APACHE" "$drawer/LICENSE-APACHE"
install -m 0644 "$root/NOTICE" "$drawer/NOTICE"
install -m 0644 "$root/ATTRIBUTION.md" "$drawer/ATTRIBUTION.md"

(cd "$stage" && lha aq "$archive.lha" "$archive")
install -m 0644 "$stage/$archive.lha" "$out/$archive.lha"
(cd "$out" && sha256sum "$archive.lha" > "$archive.lha.sha256")
install -m 0644 "$root/ports/amigaos/RELEASE-NOTES.txt" "$out/$archive.readme"

echo "GFOpenPhotoCraft RC package: $out/$archive.lha"
