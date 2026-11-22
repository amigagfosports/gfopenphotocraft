# GFOpenPhotoCraft for AmigaOS 3.x

GFOpenPhotoCraft is a native AmigaOS 68k port of the MIT-licensed PhotoCraft
image editor. It is an Amiga Hunk executable and not a host wrapper, webview or
remote interface.

## RC1 scope

The first native slice contains a bounded RGBA raster/layer model and a
GFOpenLayout editor window. It performs real image operations in that model:

- create a new 6 x 4 RGBA image;
- add and show/hide raster layers;
- invert and grayscale the active layer;
- raise and lower brightness;
- undo and redo the last pixel operation;
- redraw the preview from composited layer pixels.
- save the bounded layered RGBA model as PCF1 to `SYS:PhotoCraft.pcf`;
- open PCF1 from `SYS:PhotoCraft.pcf`, rejecting malformed, truncated,
  oversized and trailing data without replacing the live document.

The model supports images up to 4,096 pixels on either side, subject to the
1,048,576-pixel native memory guard, and up to 64 layers. Host tests cover
compositing, operations, history input and hostile sizes.

PCF1 is the native subset format for this first release candidate. An ASL
requester, GFOpenDataTypes IFF import/export and pointer brush painting remain
later gates; the UI does not claim those features.

## Native evidence

On 21 November 2026 the pinned-dependency Hunk build with SHA-256
`269066c44383b4a36897031f61b0a5593d3a7778989591470aa88bcb6841bd8a`
was run in the isolated `t1-photocraft-20261120` 68040 lab. The editor opened,
Grayscale redrew all 24 preview cells with distinct luminance values, Undo
restored the colour raster, and Redo restored the grayscale raster. Window
titles and status text followed each action. The close gadget then removed the
window and AmigaDOS `Status` confirmed that PhotoCraft was no longer running.
That receipt covers the earlier Q1 raster interaction build. The RC1 PCF1
candidate at executable SHA-256
`9b545f0135b66de0959db48a34b5bb8dc941ff25fed1ef6469559f2303da1a2b`
has independent host and Hunk build proof; its exact native Save, Open and
hostile-file run is recorded separately in `NATIVE-UAT-20261122.md`.

## Native services

- GFOpenLayout + OpenGadTools: editor window and controls;
- GFOpenDataTypes: planned bounded import/export boundary;
- GFOpenUp: planned updater boundary.

## Build gates

`check.sh` runs formatting, host tests, clippy, the freestanding m68k build,
symbol checks and relocation checks. `build-amiga.sh` produces the same Hunk
plate on the os32 Kitchen stove; `package-qualification.sh` optionally wraps
that plate in an LHA archive when `lha` is installed.
