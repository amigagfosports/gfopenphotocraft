// Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
#![cfg_attr(target_arch = "m68k", no_main)]
#![cfg_attr(target_arch = "m68k", no_std)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

extern crate alloc;

#[cfg(any(target_arch = "m68k", test))]
pub mod model;
#[cfg(any(target_arch = "m68k", test))]
mod platform;

#[cfg(target_arch = "m68k")]
use alloc::vec::Vec;
#[cfg(target_arch = "m68k")]
use model::{Document, ModelError, Pixel};

#[cfg(target_arch = "m68k")]
const EDITOR_MARKUP: &core::ffi::CStr = c"<column id=app><row id=toolbar><button id=new title='Create a fresh image'>New</button><button id=open disabled title='IFF open arrives after the native file requester gate'>Open</button><button id=save disabled title='IFF save arrives after the native file requester gate'>Save</button><div class=rule></div><button id=undo>Undo</button><button id=redo>Redo</button><div class=rule></div><button id=invert>Invert</button><button id=gray>Grayscale</button><button id=brighter>Brighter</button><button id=darker>Darker</button><div class=spacer></div><div class=badge>68k native</div></row><row id=workspace><column id=tools><div class=heading>Tools</div><button id=brush class=active>Brush</button><button id=layer>Add Layer</button><button id=visibility>Show / Hide</button><div class=heading>Colour</div><div class=swatchrow><div class='swatch ink'></div><div class='swatch paper'></div></div><div class=help>Image operations update the real bounded raster model.</div></column><column id=center><row id=documentbar><div class=tab>Untitled.pcraft *</div><div class=spacer></div><div id=zoom>600%</div></row><div id=viewport><column id=canvas><row class=pixels><div id=p0 class=pixel></div><div id=p1 class=pixel></div><div id=p2 class=pixel></div><div id=p3 class=pixel></div><div id=p4 class=pixel></div><div id=p5 class=pixel></div></row><row class=pixels><div id=p6 class=pixel></div><div id=p7 class=pixel></div><div id=p8 class=pixel></div><div id=p9 class=pixel></div><div id=p10 class=pixel></div><div id=p11 class=pixel></div></row><row class=pixels><div id=p12 class=pixel></div><div id=p13 class=pixel></div><div id=p14 class=pixel></div><div id=p15 class=pixel></div><div id=p16 class=pixel></div><div id=p17 class=pixel></div></row><row class=pixels><div id=p18 class=pixel></div><div id=p19 class=pixel></div><div id=p20 class=pixel></div><div id=p21 class=pixel></div><div id=p22 class=pixel></div><div id=p23 class=pixel></div></row></column></div></column><column id=inspector><div class=heading>Layers</div><div class='layer selected'><span class=eye>*</span><span id=activelabel>Paint layer</span></div><div id=addedlayer class='layer selected' hidden><span class=eye>*</span> Accent layer</div><div class=layer><span class=eye>*</span> Background</div><div class=heading>Properties</div><row><label>Width</label><input value='6 px' disabled></row><row><label>Height</label><input value='4 px' disabled></row><row><label>Mode</label><input value='RGBA 8' disabled></row><row><label>Opacity</label><input value='100%' disabled></row><div class=heading>Histogram</div><row class=hist><div class=h1></div><div class=h2></div><div class=h3></div><div class=h4></div><div class=h5></div></row></column></row><row id=status><div id=statusline>Ready - choose an image operation</div><div class=spacer></div><div>6 x 4 pixels / <span id=layercount>2 layers</span></div></row></column>";

#[cfg(target_arch = "m68k")]
const EDITOR_CSS: &core::ffi::CStr = c"*{box-sizing:border-box}#app{width:100%;height:100%;background:#20242b;color:#e9edf4}#toolbar{height:48px;padding:7px;gap:5px;background:#303640;border-bottom:1px solid #080a0e;align-items:center}#toolbar button{min-width:64px;height:32px;background:#4a5260;color:#ffffff;border:1px solid #717b8b}.spacer{flex:1}.rule{width:1px;height:29px;background:#6a7280;margin:0 3px}.badge{padding:7px 10px;background:#236b56;color:#ffffff;border-radius:4px}#workspace{flex:1;min-height:0}#tools{width:142px;padding:10px;gap:8px;background:#292f38;border-right:1px solid #080a0e}#tools button{height:37px;text-align:left;background:#3b424e;color:#f4f7fb;border:1px solid #5f6877}#tools .active{background:#28775f;border-color:#45b28d}.heading{font-weight:bold;padding:8px 3px 5px;color:#aeb8c8;border-bottom:1px solid #56606f}.swatchrow{height:48px;gap:8px}.swatch{width:38px;height:38px;border:2px solid #bfc7d2}.ink{background:#ee6a45}.paper{background:#f4d47a}.help{padding-top:8px;color:#b9c2ce;line-height:1.35}#center{flex:1;min-width:0;background:#161a20}#documentbar{height:38px;padding:5px 8px;background:#282e37;border-bottom:1px solid #080a0e;align-items:center}.tab{padding:7px 18px;background:#3a424e;border:1px solid #687281;border-bottom:0}#zoom{padding:5px 9px;background:#1c2128;border:1px solid #515a68}#viewport{flex:1;padding:24px;background:#4d535d;align-items:center;justify-content:center;overflow:auto}#canvas{padding:12px;background:#d9d9d9;border:1px solid #111318;box-shadow:5px 6px 12px #20242a;gap:0}.pixels{height:56px;gap:0}.pixel{width:56px;height:56px;border:1px solid #59606a}#p0{background:#1c2d4a}#p1{background:#264c6c}#p2{background:#2d7777}#p3{background:#479770}#p4{background:#97bb6f}#p5{background:#e8ce70}#p6{background:#2d435d}#p7{background:#376b84}#p8{background:#42918b}#p9{background:#68b480}#p10{background:#ccbf68}#p11{background:#eb9655}#p12{background:#453c5f}#p13{background:#654e7b}#p14{background:#975b7e}#p15{background:#c86370}#p16{background:#e8765b}#p17{background:#efa95e}#p18{background:#53454e}#p19{background:#7c525b}#p20{background:#ae6866}#p21{background:#d38667}#p22{background:#e3ae76}#p23{background:#eed3a4}#inspector{width:222px;padding:10px;gap:6px;background:#292f38;border-left:1px solid #080a0e}.layer{padding:8px;background:#373e49;border:1px solid #596271}.layer.selected{background:#28775f;color:#ffffff}.eye{margin-right:8px}#inspector row{height:31px;gap:5px;align-items:center}#inspector label{width:59px;color:#bfc7d2}#inspector input{flex:1;min-width:0;background:#20252d;color:#ffffff;border:1px solid #515b69}.hist{align-items:end;height:64px;padding:5px;background:#1d2229}.hist div{width:25px;background:#5aa78e}.h1{height:18px}.h2{height:39px}.h3{height:52px}.h4{height:31px}.h5{height:45px}#status{height:31px;padding:7px 10px;background:#303640;border-top:1px solid #080a0e;color:#cbd3df}";

#[cfg(target_arch = "m68k")]
struct EditorState {
    document: Document,
    undo: Vec<Pixel>,
    redo: Vec<Pixel>,
    title: &'static core::ffi::CStr,
    status: &'static core::ffi::CStr,
}

#[cfg(target_arch = "m68k")]
enum InitError {
    Background(ModelError),
    PaintLayer(ModelError),
    Pixels(ModelError),
}

#[cfg(target_arch = "m68k")]
impl EditorState {
    fn new() -> Result<Self, InitError> {
        Ok(Self {
            document: demo_document()?,
            undo: Vec::new(),
            redo: Vec::new(),
            title: c"PhotoCraft - Untitled (2 layers)",
            status: c"Ready - choose an image operation",
        })
    }

    fn reset(&mut self) {
        match demo_document() {
            Ok(document) => {
                self.document = document;
                self.undo.clear();
                self.redo.clear();
                self.title = c"PhotoCraft - New image";
                self.status = c"Created a new 6 x 4 RGBA image";
            }
            Err(_) => self.out_of_memory(),
        }
    }

    fn edit(&mut self, operation: fn(&mut Document) -> Result<(), ModelError>, title: &'static core::ffi::CStr, status: &'static core::ffi::CStr) {
        let Ok(before) = self.document.snapshot_active() else {
            self.out_of_memory();
            return;
        };
        if operation(&mut self.document).is_err() {
            self.title = c"PhotoCraft - Edit failed";
            self.status = c"The image operation could not be applied";
            return;
        }
        self.undo = before;
        self.redo.clear();
        self.title = title;
        self.status = status;
    }

    fn undo(&mut self) {
        if self.undo.is_empty() {
            return;
        }
        let Ok(current) = self.document.snapshot_active() else {
            self.out_of_memory();
            return;
        };
        let before = core::mem::take(&mut self.undo);
        if self.document.replace_active_pixels(&before).is_ok() {
            self.redo = current;
            self.title = c"PhotoCraft - Undo";
            self.status = c"Last pixel operation undone";
        }
    }

    fn redo(&mut self) {
        if self.redo.is_empty() {
            return;
        }
        let Ok(current) = self.document.snapshot_active() else {
            self.out_of_memory();
            return;
        };
        let after = core::mem::take(&mut self.redo);
        if self.document.replace_active_pixels(&after).is_ok() {
            self.undo = current;
            self.title = c"PhotoCraft - Redo";
            self.status = c"Pixel operation restored";
        }
    }

    fn add_layer(&mut self) {
        if self.document.layers().len() >= 3 {
            self.title = c"PhotoCraft - Layer limit reached";
            self.status = c"The compact editor view already has three layers";
            return;
        }
        if self.document.add_layer("Paint layer", Pixel::TRANSPARENT).is_err() {
            self.out_of_memory();
            return;
        }
        let _ = paint_accent(&mut self.document);
        self.undo.clear();
        self.redo.clear();
        self.title = c"PhotoCraft - Layer added";
        self.status = c"Added a new editable raster layer";
    }

    fn toggle_visibility(&mut self) {
        match self.document.toggle_active_visibility() {
            Ok(true) => {
                self.title = c"PhotoCraft - Layer visible";
                self.status = c"Active layer is visible";
            }
            Ok(false) => {
                self.title = c"PhotoCraft - Layer hidden";
                self.status = c"Active layer is hidden";
            }
            Err(_) => {
                self.title = c"PhotoCraft - Layer change failed";
                self.status = c"The active layer could not be changed";
            }
        }
    }

    fn out_of_memory(&mut self) {
        self.title = c"PhotoCraft - Not enough memory";
        self.status = c"The edit was refused without changing the image";
    }
}

#[cfg(target_arch = "m68k")]
impl amiga_gfopenlayout::EventSink for EditorState {
    fn on_event(&mut self, event: amiga_gfopenlayout::Event<'_>) -> bool {
        use amiga_gfopenlayout::EventKind;

        if event.kind != EventKind::Click {
            return false;
        }
        self.title = c"PhotoCraft - Click received";
        self.status = c"A native button click reached the editor";
        match event.registration {
            0 => self.reset(),
            1 => self.undo(),
            2 => self.redo(),
            3 => self.edit(Document::invert_active, c"PhotoCraft - Inverted", c"Inverted RGB channels on the active layer"),
            4 => self.edit(Document::grayscale_active, c"PhotoCraft - Grayscale", c"Converted the active layer to luminance"),
            5 => self.edit(brighten, c"PhotoCraft - Brighter", c"Raised active-layer brightness by 24"),
            6 => self.edit(darken, c"PhotoCraft - Darker", c"Lowered active-layer brightness by 24"),
            7 => self.add_layer(),
            8 => self.toggle_visibility(),
            9 => {
                self.title = c"PhotoCraft - Brush selected";
                self.status = c"Brush tool selected; pointer painting is the next native gate";
            }
            _ => return false,
        }
        true
    }

    fn click_ids(&self) -> &'static [&'static core::ffi::CStr] {
        &[c"new", c"undo", c"redo", c"invert", c"gray", c"brighter", c"darker", c"layer", c"visibility", c"brush"]
    }

    fn window_title(&self) -> Option<&core::ffi::CStr> {
        Some(self.title)
    }

    fn refresh(&mut self, ui: &mut amiga_gfopenlayout::Ui<'_, '_>) {
        let _ = ui.set_text(c"statusline", self.status);
        let _ = ui.set_disabled(c"undo", self.undo.is_empty());
        let _ = ui.set_disabled(c"redo", self.redo.is_empty());
        let layer_count = self.document.layers().len();
        let _ = ui.set_hidden(c"addedlayer", layer_count < 3);
        let _ = ui.set_disabled(c"layer", layer_count >= 3);
        let _ = ui.set_text(c"layercount", if layer_count >= 3 { c"3 layers" } else { c"2 layers" });
        let visible = self.document.active_layer().is_some_and(model::Layer::is_visible);
        let _ = ui.set_text(c"activelabel", if visible { c"Paint layer" } else { c"Paint layer (hidden)" });
        macro_rules! refresh_pixel {
            ($index:literal, $id:expr) => {
                if let Some(pixel) = self.document.composite_pixel($index) {
                    let bytes = style_bytes(pixel);
                    if let Ok(style) = core::ffi::CStr::from_bytes_with_nul(&bytes) {
                        let _ = ui.set_style($id, style);
                    }
                }
            };
        }
        refresh_pixel!(0, c"p0");
        refresh_pixel!(1, c"p1");
        refresh_pixel!(2, c"p2");
        refresh_pixel!(3, c"p3");
        refresh_pixel!(4, c"p4");
        refresh_pixel!(5, c"p5");
        refresh_pixel!(6, c"p6");
        refresh_pixel!(7, c"p7");
        refresh_pixel!(8, c"p8");
        refresh_pixel!(9, c"p9");
        refresh_pixel!(10, c"p10");
        refresh_pixel!(11, c"p11");
        refresh_pixel!(12, c"p12");
        refresh_pixel!(13, c"p13");
        refresh_pixel!(14, c"p14");
        refresh_pixel!(15, c"p15");
        refresh_pixel!(16, c"p16");
        refresh_pixel!(17, c"p17");
        refresh_pixel!(18, c"p18");
        refresh_pixel!(19, c"p19");
        refresh_pixel!(20, c"p20");
        refresh_pixel!(21, c"p21");
        refresh_pixel!(22, c"p22");
        refresh_pixel!(23, c"p23");
    }
}

#[cfg(target_arch = "m68k")]
fn demo_document() -> Result<Document, InitError> {
    let mut document = Document::new(6, 4, Pixel::rgb(242, 236, 216)).map_err(InitError::Background)?;
    document.add_layer("Paint layer", Pixel::TRANSPARENT).map_err(InitError::PaintLayer)?;
    let colours = [
        Pixel::rgb(28, 45, 74),
        Pixel::rgb(38, 76, 108),
        Pixel::rgb(45, 119, 119),
        Pixel::rgb(71, 151, 112),
        Pixel::rgb(151, 187, 111),
        Pixel::rgb(232, 206, 112),
        Pixel::rgb(45, 67, 93),
        Pixel::rgb(55, 107, 132),
        Pixel::rgb(66, 145, 139),
        Pixel::rgb(104, 180, 128),
        Pixel::rgb(204, 191, 104),
        Pixel::rgb(235, 150, 85),
        Pixel::rgb(69, 60, 95),
        Pixel::rgb(101, 78, 123),
        Pixel::rgb(151, 91, 126),
        Pixel::rgb(200, 99, 112),
        Pixel::rgb(232, 118, 91),
        Pixel::rgb(239, 169, 94),
        Pixel::rgb(83, 69, 78),
        Pixel::rgb(124, 82, 91),
        Pixel::rgb(174, 104, 102),
        Pixel::rgb(211, 134, 103),
        Pixel::rgb(227, 174, 118),
        Pixel::rgb(238, 211, 164),
    ];
    document.replace_active_pixels(&colours).map_err(InitError::Pixels)?;
    Ok(document)
}

#[cfg(target_arch = "m68k")]
fn paint_accent(document: &mut Document) -> Result<(), ModelError> {
    for y in 1..3 {
        for x in 2..5 {
            document.set_active_pixel(x, y, Pixel::rgb(238, 106, 69))?;
        }
    }
    Ok(())
}

#[cfg(target_arch = "m68k")]
fn brighten(document: &mut Document) -> Result<(), ModelError> {
    document.adjust_brightness_active(24)
}

#[cfg(target_arch = "m68k")]
fn darken(document: &mut Document) -> Result<(), ModelError> {
    document.adjust_brightness_active(-24)
}

#[cfg(target_arch = "m68k")]
fn style_bytes(pixel: Pixel) -> [u8; 19] {
    [
        b'b',
        b'a',
        b'c',
        b'k',
        b'g',
        b'r',
        b'o',
        b'u',
        b'n',
        b'd',
        b':',
        b'#',
        hex_digit(pixel.red >> 4),
        hex_digit(pixel.red),
        hex_digit(pixel.green >> 4),
        hex_digit(pixel.green),
        hex_digit(pixel.blue >> 4),
        hex_digit(pixel.blue),
        0,
    ]
}

#[cfg(target_arch = "m68k")]
fn hex_digit(value: u8) -> u8 {
    match value & 0x0f {
        0 => b'0',
        1 => b'1',
        2 => b'2',
        3 => b'3',
        4 => b'4',
        5 => b'5',
        6 => b'6',
        7 => b'7',
        8 => b'8',
        9 => b'9',
        10 => b'a',
        11 => b'b',
        12 => b'c',
        13 => b'd',
        14 => b'e',
        _ => b'f',
    }
}

#[cfg(target_arch = "m68k")]
#[allow(unsafe_code, reason = "the fixed AmigaDOS entry symbol is the port's isolated ABI boundary")]
#[unsafe(no_mangle)]
pub extern "C" fn gf_amiga_main(_args: *const u8, _length: i32) -> i32 {
    amiga_runtime::run(|runtime| {
        runtime.put_str(c"PhotoCraft: native entry\n");
        let native_bindings = platform::BINDINGS.len();
        if native_bindings != 3 {
            runtime.put_str(c"PhotoCraft: native service table invalid\n");
            return 21;
        }
        let mut editor = match EditorState::new() {
            Ok(editor) => editor,
            Err(InitError::Background(error)) => {
                let _ = error;
                runtime.put_str(c"PhotoCraft: background allocation failed\n");
                return 22;
            }
            Err(InitError::PaintLayer(error)) => {
                let _ = error;
                runtime.put_str(c"PhotoCraft: paint-layer allocation failed\n");
                return 23;
            }
            Err(InitError::Pixels(error)) => {
                let _ = error;
                runtime.put_str(c"PhotoCraft: demo-pixel creation failed\n");
                return 24;
            }
        };
        let Ok(window) = amiga_gfopenlayout::Window::open(runtime, c"PhotoCraft - Untitled", EDITOR_MARKUP, EDITOR_CSS, 940, 500) else {
            runtime.put_str(c"PhotoCraft: could not open GFOpenLayout editor window\n");
            return 25;
        };
        if !window.run_with(runtime, &mut editor) {
            runtime.put_str(c"PhotoCraft: could not register the GFOpenLayout event callback\n");
            return 26;
        }
        runtime.put_str(c"PhotoCraft: editor window closed\n");
        0
    })
}

#[cfg(target_arch = "m68k")]
#[allow(unsafe_code, reason = "compiler-builtins requires the C abort symbol at the executable ABI boundary")]
#[unsafe(no_mangle)]
pub extern "C" fn abort() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(target_arch = "m68k")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(not(target_arch = "m68k"))]
fn main() {}
