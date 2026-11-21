# GFOpenPhotoCraft for AmigaOS 3.x

GFOpenPhotoCraft is a native AmigaOS 68k port of the MIT-licensed PhotoCraft
image editor. It is an Amiga Hunk executable and not a host wrapper, webview or
remote interface.

## Qualification scope

The first native slice contains a bounded RGBA raster/layer model and a
GFOpenLayout editor window. It performs real image operations in that model:

- create a new 6 x 4 RGBA image;
- add and show/hide raster layers;
- invert and grayscale the active layer;
- raise and lower brightness;
- undo and redo the last pixel operation;
- redraw the preview from composited layer pixels.

The model supports images up to 4,096 pixels on either side, subject to the
1,048,576-pixel native memory guard, and up to 64 layers. Host tests cover
compositing, operations, history input and hostile sizes.

Open and Save are deliberately disabled until the ASL requester and
GFOpenDataTypes/IFF path has native proof. Pointer brush painting is also a
later gate. The Q1 package is therefore not called an RC.

## Native evidence

On 21 November 2026 the Hunk build with SHA-256
`7d40a2adcbff320c21c555d67acbb55e23ec3e7ab9025600812e899762f24fd8`
was run in the isolated `t1-photocraft-20261120` 68040 lab. The editor opened,
Grayscale redrew all 24 preview cells with distinct luminance values, Undo
restored the colour raster, and Redo restored the grayscale raster. Window
titles and status text followed each action. Close/return is still an explicit
promotion gate because the native close gadget did not close the window in
that run.

## Native services

- GFOpenLayout + OpenGadTools: editor window and controls;
- GFOpenDataTypes: planned bounded import/export boundary;
- GFOpenUp: planned updater boundary.

## Build gates

`check.sh` runs formatting, host tests, clippy, the freestanding m68k build,
symbol checks and relocation checks. `package-qualification.sh` additionally
uses an Amiga-capable binutils prefix to produce the Hunk and LHA archive.
