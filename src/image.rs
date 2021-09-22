//! [Image] and [PixelFormat] functions.

use crate::{color::Result as ColorResult, prelude::*, renderer::Rendering};
use png::{BitDepth, ColorType, Decoder};
use std::{
    borrow::Cow,
    cell::Cell,
    error,
    ffi::{OsStr, OsString},
    fmt,
    fs::File,
    io::{self, BufReader, BufWriter},
    path::Path,
    result,
};

/// The result type for [Image] operations.
pub type Result<T> = result::Result<T, Error>;

/// Format for interpreting bytes when using textures.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// 8-bit Red, Green, and Blue
    Rgb,
    /// 8-bit Red, Green, Blue, and Alpha
    Rgba,
}

impl PixelFormat {
    /// Returns the number of channels associated with the format.
    #[inline]
    pub fn channels(&self) -> usize {
        use PixelFormat::*;
        match self {
            Rgb => 3,
            Rgba => 4,
        }
    }
}

impl From<png::ColorType> for PixelFormat {
    fn from(color_type: png::ColorType) -> Self {
        use png::ColorType::*;
        match color_type {
            RGB => Self::Rgb,
            RGBA => Self::Rgba,
            _ => unimplemented!("{:?} is not supported.", color_type),
        }
    }
}

impl Default for PixelFormat {
    fn default() -> Self {
        Self::Rgba
    }
}

/// An `Image` representing a buffer of pixel color values.
#[non_exhaustive]
#[derive(Default, Clone)]
pub struct Image {
    /// `Image` width.
    width: u32,
    /// `Image` height.
    height: u32,
    /// Raw pixel data.
    data: Vec<u8>,
    /// Pixel Format.
    format: PixelFormat,
    /// Texture Identifier and whether texture requires updating.
    texture_cache: Cell<Option<(TextureId, bool)>>,
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("format", &self.format)
            .field("texture_cache", &self.texture_cache)
            .field("size", &self.data.len())
            .finish()
    }
}

impl Image {
    /// Constructs an empty RGBA `Image` with given `width` and `height`. Alias for
    /// [Image::with_rgba].
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self::with_rgba(width, height)
    }

    /// Constructs an empty RGBA `Image` with given `width` and `height`.
    pub fn with_rgba(width: u32, height: u32) -> Self {
        let format = PixelFormat::Rgba;
        let data = vec![0x00; format.channels() * (width * height) as usize];
        Self::from_vec(width, height, data, format)
    }

    /// Constructs an empty RGB `Image` with given `width` and `height`.
    pub fn with_rgb(width: u32, height: u32) -> Self {
        let format = PixelFormat::Rgb;
        let data = vec![0x00; format.channels() * (width * height) as usize];
        Self::from_vec(width, height, data, format)
    }

    /// Constructs an `Image` from a [u8] [slice] representing RGB/A values.
    pub fn from_bytes<B: AsRef<[u8]>>(
        width: u32,
        height: u32,
        bytes: B,
        format: PixelFormat,
    ) -> Result<Self> {
        let bytes = bytes.as_ref();
        if bytes.len() != (format.channels() * width as usize * height as usize) {
            return Err(Error::InvalidImage((width, height), bytes.len(), format));
        }
        Ok(Self::from_vec(width, height, bytes.to_vec(), format))
    }

    /// Constructs an `Image` from a [Vec<u8>] representing RGB/A values.
    pub fn from_vec(width: u32, height: u32, data: Vec<u8>, format: PixelFormat) -> Self {
        Self {
            width,
            height,
            data,
            format,
            texture_cache: Cell::new(None),
        }
    }

    /// Constructs an `Image` from a [png] file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let ext = path.extension();
        if ext != Some(OsStr::new("png")) {
            return Err(Error::InvalidFileType(ext.map(|e| e.to_os_string())));
        }

        let png_file = BufReader::new(File::open(&path)?);
        let png = Decoder::new(png_file);

        // TODO: Make this machine-dependent to best match display capabilities for texture
        // performance
        // EXPL: Switch RGBA32 (RGBA8888) format to ARGB8888 by swapping alpha
        // EXPL: Expand paletted to RGB and non-8-bit grayscale to 8-bits
        // png.set_transformations(Transformations::SWAP_ALPHA | Transformations::EXPAND);

        let (info, mut reader) = png.read_info()?;
        if info.bit_depth != BitDepth::Eight {
            return Err(Error::UnsupportedBitDepth(info.bit_depth));
        } else if !matches!(info.color_type, ColorType::RGB | ColorType::RGBA) {
            return Err(Error::UnsupportedColorType(info.color_type));
        }

        let mut data = vec![0x00; info.buffer_size()];
        reader.next_frame(&mut data)?;
        let format = info.color_type.into();
        Self::from_bytes(info.width, info.height, &data, format)
    }

    /// Returns the `Image` width.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the `Image` height.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the `Image` dimensions as `(width, height)`.
    #[inline]
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the center position as [Point].
    pub fn center(&self) -> PointI2 {
        point!(self.width() as i32 / 2, self.height() as i32 / 2)
    }

    /// Returns the `Image` pixel data as a [u8] [slice].
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    /// Returns the `Image` pixel data as a mutable [u8] [slice].
    #[inline]
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        self.set_updated(true);
        &mut self.data
    }

    /// Returns the color value at the given `(x, y)` position.
    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> ColorResult<'_, Color, u8> {
        let idx = self.idx(x, y);
        let channels = self.format.channels();
        Color::from_slice(ColorMode::Rgb, &self.data[idx..idx + channels])
    }

    /// Sets the color value at the given `(x, y)` position.
    #[inline]
    pub fn set_pixel<C: Into<Color>>(&mut self, x: u32, y: u32, color: C) {
        let color = color.into();
        let idx = self.idx(x, y);
        let channels = self.format.channels();
        self.data[idx..(idx + channels)].clone_from_slice(&color.channels()[..channels]);
        if let Some((_, ref mut needs_update)) = self.texture_cache.get() {
            *needs_update = true;
        }
    }

    /// Update the `Image` with a  [u8] [slice] representing RGB/A values.
    #[inline]
    pub fn update_bytes<B: AsRef<[u8]>>(&mut self, bytes: B) {
        self.set_updated(true);
        self.data.clone_from_slice(bytes.as_ref());
    }

    /// Returns the `Image` pixel format.
    #[inline]
    pub fn format(&self) -> PixelFormat {
        self.format
    }

    /// Save the `Image` to a [png] file.
    pub fn save<P>(&self, path: P) -> PixResult<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let png_file = BufWriter::new(std::fs::File::create(&path)?);
        let mut png = png::Encoder::new(png_file, self.width, self.height);
        png.set_color(png::ColorType::RGBA);
        let mut writer = png.write_header()?;
        Ok(writer.write_image_data(self.bytes())?)
    }

    /// Returns the `Image` [TextureId].
    #[inline]
    pub(crate) fn texture_cache(&self) -> Option<(TextureId, bool)> {
        self.texture_cache.get()
    }

    /// Set the `Image` [TextureId].
    #[inline]
    pub(crate) fn set_texture_id(&self, texture_id: TextureId) {
        self.texture_cache.set(Some((texture_id, false)));
    }

    /// Set the updated texture cache.
    #[inline]
    pub(crate) fn set_updated(&self, val: bool) {
        if let Some((texture_id, _)) = self.texture_cache.get() {
            self.texture_cache.set(Some((texture_id, val)));
        }
    }
}

impl Image {
    fn idx(&self, x: u32, y: u32) -> usize {
        self.format.channels() * (y * self.width + x) as usize
    }
}

impl PixState {
    /// Draw an [Image] to the current canvas.
    pub fn image<P>(&mut self, position: P, img: &Image) -> PixResult<()>
    where
        P: Into<PointI2>,
    {
        let s = &self.settings;
        let mut pos = position.into();
        if let DrawMode::Center = s.image_mode {
            pos -= img.center();
        };
        self.image_transformed(
            rect![pos.x(), pos.y(), img.width() as i32, img.height() as i32],
            img,
            0.0,
            None,
            None,
        )
    }

    /// Draw a rotated [Image] to the current canvas.
    pub fn image_transformed<R, T, C, F>(
        &mut self,
        rect: R,
        img: &Image,
        angle: T,
        center: C,
        flipped: F,
    ) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
        T: Into<Scalar>,
        C: Into<Option<PointI2>>,
        F: Into<Option<Flipped>>,
    {
        let s = &self.settings;
        let mut rect = rect.into();
        if let DrawMode::Center = s.image_mode {
            rect.center_on(rect.center());
        };
        Ok(self.renderer.image(
            rect,
            img,
            angle.into(),
            center.into(),
            flipped.into(),
            s.image_tint,
        )?)
    }
}

/// The error type for [Image] operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Invalid image.
    InvalidImage((u32, u32), usize, PixelFormat),
    /// Invalid file type.
    InvalidFileType(Option<OsString>),
    /// Invalid color type.
    UnsupportedColorType(png::ColorType),
    /// Invalid bit depth.
    UnsupportedBitDepth(png::BitDepth),
    /// I/O errors.
    IoError(io::Error),
    /// [png] decoding errors.
    DecodingError(png::DecodingError),
    /// [png] encoding errors.
    EncodingError(png::EncodingError),
    /// Unknown error.
    Other(Cow<'static, str>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidImage(dimensions, len, format) => write!(
                f,
                "invalid image. dimensions: {:?}, bytes: {}, format: {:?}",
                dimensions, len, format
            ),
            InvalidFileType(ext) => write!(f, "invalid file type: {:?}", ext),
            UnsupportedColorType(color_type) => write!(f, "invalid color type: {:?}", color_type),
            UnsupportedBitDepth(depth) => write!(f, "invalid bit depth: {:?}", depth),
            Other(err) => write!(f, "renderer error: {}", err),
            err => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            IoError(err) => err.source(),
            DecodingError(err) => err.source(),
            _ => None,
        }
    }
}

impl From<Error> for PixError {
    fn from(err: Error) -> Self {
        Self::ImageError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<png::DecodingError> for Error {
    fn from(err: png::DecodingError) -> Self {
        Error::DecodingError(err)
    }
}

impl From<png::EncodingError> for Error {
    fn from(err: png::EncodingError) -> Self {
        Error::EncodingError(err)
    }
}

impl From<png::EncodingError> for PixError {
    fn from(err: png::EncodingError) -> Self {
        PixError::ImageError(Error::EncodingError(err))
    }
}
