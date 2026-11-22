# PhotoCraft 0.1.0 RC1 native receipt — 22 November 2026

## Candidate identity

- repository: `amigagfosports/gfopenphotocraft`
- Forge branch: `team1/amigaos68k-native-20261120`
- source commit: `75e1a7062f077f28f5d83e9682d4d0d5d783ebf0`
- GFOpenRust dependency: `7a3e3e1523e6233311fc30dc2f5a8bc51f6291e5`
- Kitchen job: `j20261122T034258-c53a0d`
- stove: `os32`
- executable: AmigaOS LoadSeg Hunk, 36,120 bytes
- executable SHA-256:
  `9b545f0135b66de0959db48a34b5bb8dc941ff25fed1ef6469559f2303da1a2b`

The Kitchen output and the binary installed in the lab had the same size and
SHA-256. The previous Q1 binary was preserved in the lab archive before the
replacement.

## Independent build gates

The os32 Kitchen job completed with exit code zero. It ran:

- `cargo fmt --all --check`;
- seven host tests, all passing;
- clippy for all targets with warnings denied;
- the freestanding `m68k-unknown-none-elf` release build;
- Amiga Hunk format, required-symbol and undefined-symbol checks.

## Native system

- isolated lab: `t1-photocraft-20261120`
- source instance: AmiCradle Instance 52
- CPU target: 68040
- application path: `SYS:Tests/PhotoCraft/PhotoCraft`
- document path: `SYS:PhotoCraft.pcf`

## Save and Open

1. The exact Kitchen candidate opened its native GFOpenLayout editor.
2. `Add Layer` created a third visible raster layer.
3. `Grayscale` changed the active layer and redrew its affected cells.
4. `Save` reported `Document saved` and wrote a 342-byte PCF1 file.
5. `New` restored the two-layer colour document.
6. `Open` reported `Document opened`, restored the three layers and restored
   the edited pixels.

## Hostile-file handling

1. The valid 342-byte file was copied aside inside the lab.
2. A four-byte invalid file replaced `SYS:PhotoCraft.pcf`.
3. `Open` reported `Open failed: magic`.
4. The three live layers and edited canvas remained intact and responsive.
5. The valid file was restored and `Open` immediately restored the document
   again.
6. The close gadget removed the window; the agent returned to idle and the
   valid 342-byte file remained present.

## RC1 boundary

This receipt qualifies the native 6 x 4 layered editor and bounded PCF1
persistence path. It does not claim ASL file requesters, IFF/DataTypes
import/export, pointer painting, the upstream egui interface or full `.pcraft`
compatibility.

## RC1 distributable package

LumaVault Kitchen job `j20261122T145502-70d810` cooked exact Forge commit
`34c22376a02098e0f3c4c0c338c18ba831268e7d` on the `os32` stove with target
`package`. It repeated all seven tests, clippy, the native Hunk and symbol
gates before creating the Workbench-ready LhA. The job exited 0.

- archive: `GFOpenPhotoCraft-0.1.0-rc1-68040.lha`
- size: 33,959 bytes
- SHA-256:
  `7062add7426462127779f6b3eff5c7827bf1d6e0637bd03b078aa40e4abf9ece`
- contents: native PhotoCraft drawer, ReadMe, checksums, MIT and Apache-2.0
  licences, notice and attribution

RC1 is a manual Workbench installation: unpack the drawer and run
`PhotoCraft`. An Installer/updater path remains outside the claimed RC1 scope.
