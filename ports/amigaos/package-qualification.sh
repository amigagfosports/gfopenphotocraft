#!/bin/sh
# Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
set -eu

port=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$port/../.." && pwd)
stove=${1:-/home/da1ek/AmiCradle/kitchen/stoves/os32}
name=GFOpenPhotoCraft-0.1.0-rc1
dist="$root/dist/amigaos"
drawer="$dist/$name"

mkdir -p "$drawer"
STOVE="$stove" OUT="$drawer" "$port/build-amiga.sh"

if command -v lha >/dev/null 2>&1; then
    (cd "$dist" && lha aq "$name.lha" "$name")
    (cd "$dist" && sha256sum "$name.lha" "$name/PhotoCraft" "$name/README.txt" > SHA256SUMS)
    echo "Release package: $dist/$name.lha"
else
    echo "Release drawer: $drawer"
fi
