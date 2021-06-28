//! # Getting Started
//!
//! `pix_engine` is a cross-platform graphics/UI engine framework for simple games, visualizations,
//! and graphics demos.
//!
//! The goal of this library is to be simpler to setup and use for graphics or algorithm
//! exploration than larger graphics libraries.
//!
//! This is intended to be more than just a toy crate, however, and can be used to drive real
//! applications. For example, the
//! [`Tetanes`](https://crates.io/crates/tetanes) NES emulator is driven by this crate.
//!
//! ```no_run
//! # #![allow(unused_variables)]
//! use pix_engine::prelude::*;
//!
//! struct App;
//!
//! impl AppState for App {
//!     fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Setup App state. PixState contains engine specific state and
//!         // utility functions for things like getting mouse coordinates,
//!         // drawing shapes, etc.
//!         Ok(())
//!     }
//!     fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Main render loop. Called roughly every 16ms.
//!         Ok(())
//!     }
//!     fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Teardown any state or resources before exiting.
//!         Ok(())
//!     }
//! }
//!
//! pub fn main() {
//!     let mut engine = PixEngine::builder()
//!       .with_dimensions(800, 600)
//!       .with_title("App Title")
//!       .position_centered()
//!       .vsync_enabled()
//!       .build();
//!     let mut app = App;
//!     engine.run(&mut app).expect("engine run");
//! }
//! ```

#![deny(missing_docs, missing_doc_code_examples)]
#![warn(
    unused,
    deprecated_in_future,
    unreachable_pub,
    unused_crate_dependencies,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    future_incompatible,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    variant_size_differences,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]
#![doc(
    html_root_url = "https://docs.rs/pix-engine/0.3.5",
    html_favicon_url = "",
    html_logo_url = "",
    test(attr(deny(warnings)))
)]

#[cfg(target_arch = "wasm32")]
use getrandom as _; // Used to set "js" feature for rand
use include_dir::{include_dir, Dir};

// Bundles static binary assets with crate
const _STATIC_DIR: Dir<'_> = include_dir!("./static");

#[macro_use]
pub mod color;
pub mod audio;
pub mod draw;
pub mod engine;
pub mod event;
pub mod image;
pub mod lighting;
#[macro_use]
pub mod math;
pub mod renderer;
#[macro_use]
pub mod shape;
pub mod state;
#[macro_use]
pub mod vector;
pub mod texture;
pub mod window;

mod common;
mod utils;

/// Exports most commonly used types, traits, and functions for 2D.
pub mod prelude {
    use super::*;
    pub use color::{constants::*, Color, ColorError, ColorMode};
    pub use common::{Error as PixError, Result as PixResult};
    pub use draw::Draw;
    pub use engine::PixEngine;
    pub use event::{Axis, Button, Event, Key, KeyEvent, KeyMod, Mouse, WindowEvent};
    pub use image::{Image, PixelFormat};
    pub use math::{constants, map, Scalar};
    pub use shape::{Circle, Ellipse, Line, Point, Rect, Shape, ShapeNum, Triangle};
    pub use state::{
        settings::{AngleMode, ArcMode, BlendMode, DrawMode, TextStyle::*},
        AppState, PixState,
    };
    pub use texture::TextureId;
    pub use vector::Vector;
    pub use window::{Position, WindowBuilder, WindowId};
    // Color macros
    pub use {color, hsb, hsl, rgb};
    // Math macros
    pub use {noise, random, vector};
    // Shape macros
    pub use {circle, ellipse, point, rect, sphere, square};
}

/// Exports most commonly used types, traits, and functions for 3D.
pub mod prelude_3d {
    pub use super::*;
    pub use lighting::{Light, LightSource};
    pub use prelude::*;
    pub use shape::Sphere;
}
