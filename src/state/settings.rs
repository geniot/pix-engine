//! Settings for the current [`PixEngine`] [`PixState`].
//!
//! [`PixEngine`]: crate::prelude::PixEngine

use crate::{
    prelude::{Color, PixState, Rect, Scalar},
    renderer::{self, Rendering},
    window::Window,
};
use num_traits::AsPrimitive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Drawing mode which changes how `(x, y)` coordinates are interpreted.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DrawMode {
    /// Use `(x, y)` as the top-left corner. Default.
    Corner,
    /// Use `(x, y)` as the center.
    Center,
}

/// Drawing mode which changes how arcs are drawn.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ArcMode {
    /// Draws arc with fill as an open pie segment.
    None,
    /// Draws arc with fill as an closed pie segment.
    Pie,
    /// Draws arc with fill as an open semi-circle.
    Open,
    /// Draws arc with fill as a closed semi-circle.
    Chord,
}

/// Drawing mode which changes how textures are blended together.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BlendMode {
    /// Disable blending.
    None,
    /// Alpha blending.
    Blend,
    /// Additive blending.
    Add,
    /// Color modulate.
    Mod,
}

/// Angle mode which changes how math functions interpreted.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AngleMode {
    /// Radians.
    Radians,
    /// Degrees.
    Degrees,
}

/// Text style for Bold/Italic rendering of fonts.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextStyle {
    /// Normal.
    Normal,
    /// Italic.
    Italic,
    /// Bold.
    Bold,
    /// Bold & Italic.
    BoldItalic,
}

/// Several settings used to change various functionality of the engine.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct Settings {
    pub(crate) background: Color,
    pub(crate) fill: Option<Color>,
    pub(crate) stroke: Option<Color>,
    pub(crate) text_size: u32,
    pub(crate) text_style: TextStyle,
    pub(crate) text_font: String,
    pub(crate) paused: bool,
    pub(crate) show_frame_rate: bool,
    pub(crate) rect_mode: DrawMode,
    pub(crate) ellipse_mode: DrawMode,
    pub(crate) blend_mode: BlendMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: Color::default(),
            fill: None,
            stroke: None,
            text_size: 16,
            text_style: TextStyle::Normal,
            text_font: "Emulogic".to_string(),
            paused: false,
            show_frame_rate: false,
            rect_mode: DrawMode::Corner,
            ellipse_mode: DrawMode::Corner,
            blend_mode: BlendMode::None,
        }
    }
}

impl PixState {
    /// Sets the `Color` value used to clear the canvas.
    pub fn background<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.settings.background = color.into();
    }

    /// Sets the `Color` value used to fill shapes drawn on the canvas.
    pub fn fill<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.settings.fill = Some(color.into());
    }

    /// Disables filling shapes drawn on the canvas.
    pub fn no_fill(&mut self) {
        self.settings.fill = None;
    }

    /// Sets the `Color` value used to outline shapes drawn on the canvas.
    pub fn stroke<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.settings.stroke = Some(color.into());
    }

    /// Disables outlining shapes drawn on the canvas.
    pub fn no_stroke(&mut self) {
        self.settings.stroke = None;
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    pub fn clip(&mut self, rect: impl Into<Rect<Scalar>>) {
        self.renderer.clip(rect.into());
    }

    /// Clears the clip rect used by the renderer to draw to the current canvas.
    pub fn no_clip(&mut self) {
        self.renderer.clip::<Scalar, _>(None);
    }

    /// Returns whether the application is fullscreen or not.
    pub fn fullscreen(&mut self) -> bool {
        self.renderer.fullscreen()
    }

    /// Set the application to fullscreen or not.
    pub fn set_fullscreen(&mut self, val: bool) {
        self.renderer.set_fullscreen(val)
    }

    /// Set whether the cursor is shown or not.
    pub fn cursor(&mut self, show: bool) {
        self.renderer.cursor(show);
    }

    /// Set the cursor icon.
    pub fn cursor_icon<P>(&mut self, _path: P)
    where
        P: AsRef<Path>,
    {
        todo!("cursor_icon");
    }

    /// Whether the render loop is paused or not.
    pub fn paused(&mut self) -> bool {
        self.settings.paused
    }

    /// Pause or unpause the render loop.
    pub fn pause(&mut self, paused: bool) {
        self.settings.paused = paused;
    }

    /// Set whether to show the current frame rate per second in the title or not.
    pub fn show_frame_rate(&mut self, show: bool) {
        self.settings.show_frame_rate = show;
    }

    /// Set the rendering scale of the current canvas.
    pub fn scale<T: AsPrimitive<f32>>(&mut self, x: T, y: T) -> renderer::Result<()> {
        self.renderer.scale(x, y)
    }

    /// Set the text size for drawing to the current canvas.
    pub fn text_size(&mut self, size: u32) {
        self.settings.text_size = size;
        todo!(); // self.renderer.text_size
    }

    /// Set the text style for drawing to the current canvas.
    pub fn text_style(&mut self, style: TextStyle) {
        self.settings.text_style = style;
        todo!(); // self.renderer.text_style
    }

    /// Set the text font-family for drawing to the current canvas.
    pub fn text_font(&mut self, font: impl Into<String>) {
        self.settings.text_font = font.into();
        todo!(); // self.renderer.text_style
    }

    /// Change the way parameters are interpreted for drawing squares and rectangles.
    pub fn rect_mode(&mut self, mode: DrawMode) {
        self.settings.rect_mode = mode;
    }

    /// Change the way parameters are interpreted for drawing circles and ellipses.
    pub fn ellipse_mode(&mut self, mode: DrawMode) {
        self.settings.ellipse_mode = mode;
    }

    /// Change the way textures are blended together.
    pub fn blend_mode(&mut self, mode: BlendMode) {
        self.settings.blend_mode = mode;
        self.renderer.blend_mode(mode);
    }

    /// Saves the current draw settings and transforms.
    pub fn push(&mut self) {
        todo!("push");
    }

    /// Restores the current draw settings and transforms.
    pub fn pop(&mut self) {
        todo!("pop");
    }
}
