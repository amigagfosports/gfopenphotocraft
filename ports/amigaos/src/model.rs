// Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
//! Bounded native raster and layer model for the PhotoCraft AmigaOS port.

use alloc::string::String;
use alloc::vec::Vec;

pub const MAX_DIMENSION: u16 = 4_096;
pub const MAX_PIXELS: usize = 1_048_576;
pub const MAX_LAYERS: usize = 64;
pub const MAX_NAME_BYTES: usize = 256;
pub const MAX_FILE_BYTES: usize = 262_144;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormatStage {
    Magic,
    Width,
    Height,
    LayerCount,
    ActiveLayer,
    LayerName,
    Visibility,
    Opacity,
    Pixels,
    TrailingData,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Pixel {
    pub const TRANSPARENT: Self = Self { red: 0, green: 0, blue: 0, alpha: 0 };
    pub const WHITE: Self = Self { red: 255, green: 255, blue: 255, alpha: 255 };

    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue, alpha: 255 }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelError {
    EmptyName,
    NameTooLong,
    InvalidDimensions,
    PixelLimit,
    LayerLimit,
    MissingLayer,
    PixelOutOfBounds,
    PixelCountMismatch,
    OutOfMemory,
    BufferTooSmall,
    InvalidFormat(FormatStage),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Layer {
    name: String,
    visible: bool,
    opacity: u8,
    pixels: Vec<Pixel>,
}

impl Layer {
    fn new(name: &str, pixel_count: usize, fill: Pixel) -> Result<Self, ModelError> {
        let name = copy_name(name)?;
        let mut pixels = Vec::new();
        pixels.try_reserve_exact(pixel_count).map_err(|_| ModelError::OutOfMemory)?;
        // Push into already-reserved storage instead of using Vec::resize.
        // The freestanding m68k alloc build has native coverage for exact
        // reservation and push, while bulk resize still depends on a generic
        // extend path that has not yet been qualified on AmigaOS.
        for _ in 0..pixel_count {
            pixels.push(fill);
        }
        Ok(Self { name, visible: true, opacity: 255, pixels })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn is_visible(&self) -> bool {
        self.visible
    }

    pub const fn opacity(&self) -> u8 {
        self.opacity
    }

    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Document {
    width: u16,
    height: u16,
    layers: Vec<Layer>,
    active: usize,
}

impl Document {
    pub fn new(width: u16, height: u16, background: Pixel) -> Result<Self, ModelError> {
        let pixel_count = checked_pixel_count(width, height)?;
        let background = Layer::new("Background", pixel_count, background)?;
        let mut layers = Vec::new();
        layers.try_reserve_exact(1).map_err(|_| ModelError::OutOfMemory)?;
        layers.push(background);
        Ok(Self { width, height, layers, active: 0 })
    }

    pub const fn width(&self) -> u16 {
        self.width
    }

    pub const fn height(&self) -> u16 {
        self.height
    }

    pub fn pixel_count(&self) -> usize {
        usize::from(self.width) * usize::from(self.height)
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }

    pub const fn active_layer_index(&self) -> usize {
        self.active
    }

    pub fn active_layer(&self) -> Option<&Layer> {
        self.layers.get(self.active)
    }

    fn active_layer_mut(&mut self) -> Result<&mut Layer, ModelError> {
        self.layers.get_mut(self.active).ok_or(ModelError::MissingLayer)
    }

    pub fn add_layer(&mut self, name: &str, fill: Pixel) -> Result<usize, ModelError> {
        if self.layers.len() >= MAX_LAYERS {
            return Err(ModelError::LayerLimit);
        }
        let layer = Layer::new(name, self.pixel_count(), fill)?;
        self.layers.try_reserve_exact(1).map_err(|_| ModelError::OutOfMemory)?;
        self.layers.push(layer);
        self.active = self.layers.len() - 1;
        Ok(self.active)
    }

    pub fn set_active_pixel(&mut self, x: u16, y: u16, pixel: Pixel) -> Result<(), ModelError> {
        if x >= self.width || y >= self.height {
            return Err(ModelError::PixelOutOfBounds);
        }
        let index = usize::from(y) * usize::from(self.width) + usize::from(x);
        let layer = self.active_layer_mut()?;
        let destination = layer.pixels.get_mut(index).ok_or(ModelError::PixelOutOfBounds)?;
        *destination = pixel;
        Ok(())
    }

    pub fn fill_active(&mut self, pixel: Pixel) -> Result<(), ModelError> {
        self.active_layer_mut()?.pixels.fill(pixel);
        Ok(())
    }

    pub fn invert_active(&mut self) -> Result<(), ModelError> {
        for pixel in &mut self.active_layer_mut()?.pixels {
            pixel.red = 255 - pixel.red;
            pixel.green = 255 - pixel.green;
            pixel.blue = 255 - pixel.blue;
        }
        Ok(())
    }

    pub fn grayscale_active(&mut self) -> Result<(), ModelError> {
        for pixel in &mut self.active_layer_mut()?.pixels {
            let luminance = (77 * u32::from(pixel.red) + 150 * u32::from(pixel.green) + 29 * u32::from(pixel.blue) + 128) >> 8;
            let gray = luminance as u8;
            pixel.red = gray;
            pixel.green = gray;
            pixel.blue = gray;
        }
        Ok(())
    }

    pub fn adjust_brightness_active(&mut self, amount: i16) -> Result<(), ModelError> {
        for pixel in &mut self.active_layer_mut()?.pixels {
            pixel.red = adjust_channel(pixel.red, amount);
            pixel.green = adjust_channel(pixel.green, amount);
            pixel.blue = adjust_channel(pixel.blue, amount);
        }
        Ok(())
    }

    pub fn toggle_active_visibility(&mut self) -> Result<bool, ModelError> {
        let layer = self.active_layer_mut()?;
        layer.visible = !layer.visible;
        Ok(layer.visible)
    }

    pub fn snapshot_active(&self) -> Result<Vec<Pixel>, ModelError> {
        let layer = self.active_layer().ok_or(ModelError::MissingLayer)?;
        let mut snapshot = Vec::new();
        snapshot.try_reserve_exact(layer.pixels.len()).map_err(|_| ModelError::OutOfMemory)?;
        snapshot.extend_from_slice(&layer.pixels);
        Ok(snapshot)
    }

    pub fn replace_active_pixels(&mut self, pixels: &[Pixel]) -> Result<(), ModelError> {
        if pixels.len() != self.pixel_count() {
            return Err(ModelError::PixelCountMismatch);
        }
        self.active_layer_mut()?.pixels.copy_from_slice(pixels);
        Ok(())
    }

    pub fn composite_pixel(&self, index: usize) -> Option<Pixel> {
        if index >= self.pixel_count() {
            return None;
        }
        let mut output = Pixel::TRANSPARENT;
        for layer in &self.layers {
            if !layer.visible {
                continue;
            }
            let source = *layer.pixels.get(index)?;
            output = source_over(source, layer.opacity, output);
        }
        Some(output)
    }

    pub fn encode(&self, output: &mut [u8]) -> Result<usize, ModelError> {
        let mut writer = Writer::new(output);
        writer.bytes(b"PCF1")?;
        writer.u16(self.width)?;
        writer.u16(self.height)?;
        writer.u8(u8::try_from(self.layers.len()).map_err(|_| ModelError::LayerLimit)?)?;
        writer.u8(u8::try_from(self.active).map_err(|_| ModelError::LayerLimit)?)?;
        for layer in &self.layers {
            writer.u16(u16::try_from(layer.name.len()).map_err(|_| ModelError::NameTooLong)?)?;
            writer.bytes(layer.name.as_bytes())?;
            writer.u8(u8::from(layer.visible))?;
            writer.u8(layer.opacity)?;
            for pixel in &layer.pixels {
                writer.u8(pixel.red)?;
                writer.u8(pixel.green)?;
                writer.u8(pixel.blue)?;
                writer.u8(pixel.alpha)?;
            }
        }
        Ok(writer.len())
    }

    pub fn decode(input: &[u8]) -> Result<Self, ModelError> {
        let mut reader = Reader::new(input);
        if reader.bytes(4, FormatStage::Magic)? != b"PCF1" {
            return Err(ModelError::InvalidFormat(FormatStage::Magic));
        }
        let width = reader.u16(FormatStage::Width)?;
        let height = reader.u16(FormatStage::Height)?;
        let pixel_count = checked_pixel_count(width, height)?;
        let layer_count = usize::from(reader.u8(FormatStage::LayerCount)?);
        if layer_count == 0 || layer_count > MAX_LAYERS {
            return Err(ModelError::InvalidFormat(FormatStage::LayerCount));
        }
        let active = usize::from(reader.u8(FormatStage::ActiveLayer)?);
        if active >= layer_count {
            return Err(ModelError::InvalidFormat(FormatStage::ActiveLayer));
        }
        let mut layers = Vec::new();
        layers.try_reserve_exact(layer_count).map_err(|_| ModelError::OutOfMemory)?;
        for _ in 0..layer_count {
            let name_length = usize::from(reader.u16(FormatStage::LayerName)?);
            if name_length == 0 || name_length > MAX_NAME_BYTES {
                return Err(ModelError::InvalidFormat(FormatStage::LayerName));
            }
            let name_bytes = reader.bytes(name_length, FormatStage::LayerName)?;
            let name = core::str::from_utf8(name_bytes).map_err(|_| ModelError::InvalidFormat(FormatStage::LayerName))?;
            let visible = match reader.u8(FormatStage::Visibility)? {
                0 => false,
                1 => true,
                _ => return Err(ModelError::InvalidFormat(FormatStage::Visibility)),
            };
            let opacity = reader.u8(FormatStage::Opacity)?;
            let mut pixels = Vec::new();
            pixels.try_reserve_exact(pixel_count).map_err(|_| ModelError::OutOfMemory)?;
            for _ in 0..pixel_count {
                pixels.push(Pixel {
                    red: reader.u8(FormatStage::Pixels)?,
                    green: reader.u8(FormatStage::Pixels)?,
                    blue: reader.u8(FormatStage::Pixels)?,
                    alpha: reader.u8(FormatStage::Pixels)?,
                });
            }
            layers.push(Layer { name: copy_name(name)?, visible, opacity, pixels });
        }
        if !reader.is_finished() {
            return Err(ModelError::InvalidFormat(FormatStage::TrailingData));
        }
        Ok(Self { width, height, layers, active })
    }
}

struct Writer<'a> {
    output: &'a mut [u8],
    cursor: usize,
}

impl<'a> Writer<'a> {
    const fn new(output: &'a mut [u8]) -> Self {
        Self { output, cursor: 0 }
    }

    const fn len(&self) -> usize {
        self.cursor
    }

    fn bytes(&mut self, bytes: &[u8]) -> Result<(), ModelError> {
        let end = self.cursor.checked_add(bytes.len()).ok_or(ModelError::BufferTooSmall)?;
        let destination = self.output.get_mut(self.cursor..end).ok_or(ModelError::BufferTooSmall)?;
        destination.copy_from_slice(bytes);
        self.cursor = end;
        Ok(())
    }

    fn u8(&mut self, value: u8) -> Result<(), ModelError> {
        self.bytes(&[value])
    }

    fn u16(&mut self, value: u16) -> Result<(), ModelError> {
        self.bytes(&value.to_be_bytes())
    }
}

struct Reader<'a> {
    input: &'a [u8],
    cursor: usize,
}

impl<'a> Reader<'a> {
    const fn new(input: &'a [u8]) -> Self {
        Self { input, cursor: 0 }
    }

    fn bytes(&mut self, length: usize, stage: FormatStage) -> Result<&'a [u8], ModelError> {
        let end = self.cursor.checked_add(length).ok_or(ModelError::InvalidFormat(stage))?;
        let bytes = self.input.get(self.cursor..end).ok_or(ModelError::InvalidFormat(stage))?;
        self.cursor = end;
        Ok(bytes)
    }

    fn u8(&mut self, stage: FormatStage) -> Result<u8, ModelError> {
        self.bytes(1, stage)?.first().copied().ok_or(ModelError::InvalidFormat(stage))
    }

    fn u16(&mut self, stage: FormatStage) -> Result<u16, ModelError> {
        let bytes = self.bytes(2, stage)?;
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    fn is_finished(&self) -> bool {
        self.cursor == self.input.len()
    }
}

fn checked_pixel_count(width: u16, height: u16) -> Result<usize, ModelError> {
    if width == 0 || height == 0 || width > MAX_DIMENSION || height > MAX_DIMENSION {
        return Err(ModelError::InvalidDimensions);
    }
    let count = usize::from(width).checked_mul(usize::from(height)).ok_or(ModelError::PixelLimit)?;
    if count > MAX_PIXELS {
        return Err(ModelError::PixelLimit);
    }
    Ok(count)
}

fn copy_name(name: &str) -> Result<String, ModelError> {
    if name.is_empty() {
        return Err(ModelError::EmptyName);
    }
    if name.len() > MAX_NAME_BYTES {
        return Err(ModelError::NameTooLong);
    }
    let mut owned = String::new();
    owned.try_reserve_exact(name.len()).map_err(|_| ModelError::OutOfMemory)?;
    owned.push_str(name);
    Ok(owned)
}

fn adjust_channel(channel: u8, amount: i16) -> u8 {
    (i16::from(channel) + amount).clamp(0, 255) as u8
}

fn source_over(source: Pixel, opacity: u8, destination: Pixel) -> Pixel {
    // Keep the two overwhelmingly common editor cases on a small, exact path.
    // Besides avoiding needless division, this is friendlier to the current
    // m68k LLVM backend than lowering the full alpha expression for every
    // opaque or transparent pixel during a native refresh.
    if source.alpha == 0 || opacity == 0 {
        return destination;
    }
    if source.alpha == 255 && opacity == 255 {
        return source;
    }
    let source_alpha = u32::from(source.alpha) * u32::from(opacity) / 255;
    let destination_alpha = u32::from(destination.alpha);
    let inverse = 255 - source_alpha;
    let output_alpha = source_alpha + destination_alpha * inverse / 255;
    if output_alpha == 0 {
        return Pixel::TRANSPARENT;
    }
    let blend = |source_channel: u8, destination_channel: u8| {
        let source_part = u32::from(source_channel) * source_alpha;
        let destination_part = u32::from(destination_channel) * destination_alpha * inverse / 255;
        ((source_part + destination_part) / output_alpha).min(255) as u8
    };
    Pixel {
        red: blend(source.red, destination.red),
        green: blend(source.green, destination.green),
        blue: blend(source.blue, destination.blue),
        alpha: output_alpha.min(255) as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_composites_bounded_layers() -> Result<(), ModelError> {
        let mut document = Document::new(2, 2, Pixel::WHITE)?;
        assert_eq!(document.add_layer("Paint", Pixel::TRANSPARENT), Ok(1));
        document.set_active_pixel(1, 0, Pixel::rgb(20, 40, 60))?;
        assert_eq!(document.composite_pixel(0), Some(Pixel::WHITE));
        assert_eq!(document.composite_pixel(1), Some(Pixel::rgb(20, 40, 60)));
        assert_eq!(document.layers()[1].name(), "Paint");
        assert_eq!(document.active_layer_index(), 1);
        Ok(())
    }

    #[test]
    fn image_operations_change_rgb_but_preserve_alpha() -> Result<(), ModelError> {
        let mut document = Document::new(1, 1, Pixel::rgb(10, 20, 30))?;
        document.invert_active()?;
        assert_eq!(document.active_layer().and_then(|layer| layer.pixels().first()).copied(), Some(Pixel::rgb(245, 235, 225)));
        document.grayscale_active()?;
        let pixel = document.active_layer().and_then(|layer| layer.pixels().first()).copied();
        assert!(pixel.is_some_and(|value| value.red == value.green && value.green == value.blue && value.alpha == 255));
        document.adjust_brightness_active(40)?;
        assert!(document.active_layer().and_then(|layer| layer.pixels().first()).is_some_and(|value| value.red > 225));
        Ok(())
    }

    #[test]
    fn snapshot_restores_a_real_edit() -> Result<(), ModelError> {
        let mut document = Document::new(2, 1, Pixel::rgb(1, 2, 3))?;
        let before = document.snapshot_active()?;
        document.fill_active(Pixel::rgb(90, 80, 70))?;
        assert_ne!(document.snapshot_active()?, before);
        document.replace_active_pixels(&before)?;
        assert_eq!(document.snapshot_active()?, before);
        Ok(())
    }

    #[test]
    fn hostile_sizes_and_coordinates_fail_closed() -> Result<(), ModelError> {
        assert_eq!(Document::new(0, 1, Pixel::WHITE), Err(ModelError::InvalidDimensions));
        assert_eq!(Document::new(MAX_DIMENSION, MAX_DIMENSION, Pixel::WHITE), Err(ModelError::PixelLimit));
        let mut document = Document::new(1, 1, Pixel::WHITE)?;
        assert_eq!(document.set_active_pixel(1, 0, Pixel::WHITE), Err(ModelError::PixelOutOfBounds));
        assert_eq!(document.replace_active_pixels(&[]), Err(ModelError::PixelCountMismatch));
        Ok(())
    }

    #[test]
    fn pcf1_round_trips_layers_pixels_and_active_state() -> Result<(), ModelError> {
        let mut document = Document::new(2, 1, Pixel::rgb(1, 2, 3))?;
        document.add_layer("Paint", Pixel::TRANSPARENT)?;
        document.set_active_pixel(1, 0, Pixel { red: 9, green: 8, blue: 7, alpha: 6 })?;
        let mut bytes = [0_u8; 128];
        let length = document.encode(&mut bytes)?;
        let decoded = Document::decode(&bytes[..length])?;
        assert_eq!(decoded, document);
        Ok(())
    }

    #[test]
    fn pcf1_rejects_truncation_trailing_data_and_hostile_counts() -> Result<(), ModelError> {
        let document = Document::new(1, 1, Pixel::WHITE)?;
        let mut bytes = [0_u8; 64];
        let length = document.encode(&mut bytes)?;
        assert_eq!(Document::decode(&bytes[..3]), Err(ModelError::InvalidFormat(FormatStage::Magic)));
        assert_eq!(Document::decode(&bytes[..length + 1]), Err(ModelError::InvalidFormat(FormatStage::TrailingData)));
        let mut hostile = bytes;
        hostile[8] = 65;
        assert_eq!(Document::decode(&hostile[..length]), Err(ModelError::InvalidFormat(FormatStage::LayerCount)));
        Ok(())
    }
}
