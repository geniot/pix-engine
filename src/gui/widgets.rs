//! UI widget rendering methods.
//!
//! Provided [PixState] methods:
//!
//! - [PixState::button]
//! - [PixState::checkbox]
//! - [PixState::radio]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { checkbox: bool, radio: usize };
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     if s.button("Button")? {
//!         // was clicked
//!     }
//!     s.checkbox("Checkbox", &mut self.checkbox)?;
//!     s.radio("Radio 1", &mut self.radio, 0)?;
//!     s.radio("Radio 2", &mut self.radio, 1)?;
//!     s.radio("Radio 3", &mut self.radio, 2)?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::prelude::*;

pub mod field;
pub mod select;
pub mod text;
pub mod tooltip;

const CHECKBOX_SIZE: i32 = 16;
const RADIO_SIZE: i32 = 8;

impl PixState {
    /// Draw a button to the current canvas that returns `true` when clicked.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     if s.button("Button")? {
    ///         // was clicked
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn button<L>(&mut self, label: L) -> PixResult<bool>
    where
        L: AsRef<str>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let style = s.theme.style;
        let pad = style.item_pad;

        // Calculate button size
        let (mut width, height) = s.size_of(label)?;
        if let Some(next_width) = s.ui.next_width {
            width = next_width;
        }
        let mut button = rect![
            pos.x(),
            pos.y(),
            width as i32 + 2 * pad.x(),
            height as i32 + 2 * pad.y()
        ];

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, button);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();
        s.ui.push_cursor();

        // Render
        s.rect_mode(RectMode::Corner);

        // Button
        s.push();
        if focused {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(Cursor::hand())?;
            s.fill(s.highlight_color());
            if active {
                button.offset([1, 1]);
            }
        } else if disabled {
            s.fill(s.primary_color() / 2);
        } else {
            s.fill(s.primary_color());
        }
        s.rect(button)?;
        s.pop();

        // Button text
        s.rect_mode(RectMode::Center);
        s.clip(button)?;
        s.set_cursor_pos(button.center());
        s.text(label)?;
        s.no_clip()?;

        s.ui.pop_cursor();
        s.pop();

        // Process input
        s.ui.handle_events(id);
        s.advance_cursor(button);
        if !disabled {
            Ok(s.ui.was_clicked(id))
        } else {
            Ok(false)
        }
    }

    /// Draw a checkbox to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.checkbox("Checkbox", &mut self.checkbox)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn checkbox<S>(&mut self, label: S, checked: &mut bool) -> PixResult<bool>
    where
        S: AsRef<str>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();

        // Calculate checkbox rect
        let checkbox = square![pos, CHECKBOX_SIZE];

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, checkbox);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();

        // Render
        s.rect_mode(RectMode::Corner);

        // Checkbox
        s.push();
        if focused || active {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(Cursor::hand())?;
            s.fill(s.secondary_color());
        } else if disabled {
            s.fill(s.primary_color() / 2);
        } else {
            s.fill(s.primary_color());
        }
        s.rect(checkbox)?;

        if *checked {
            if disabled {
                s.stroke(s.highlight_color() / 2);
            } else {
                s.stroke(s.highlight_color());
            }
            s.stroke_weight(2);
            let half = CHECKBOX_SIZE / 2;
            let third = CHECKBOX_SIZE / 3;
            let x = checkbox.left() + half - 1;
            let y = checkbox.bottom() - third;
            let start = point![x - third + 2, y - third + 2];
            let mid = point![x, y];
            let end = point![x + third + 1, y - half + 2];
            s.line([start, mid])?;
            s.line([mid, end])?;
        }
        s.advance_cursor(checkbox);
        s.pop();

        // Label
        s.same_line(None);
        s.text(label)?;

        // Process input
        s.ui.handle_events(id);
        if !disabled {
            let clicked = s.ui.was_clicked(id);
            if clicked {
                *checked = !(*checked);
            }
            Ok(clicked)
        } else {
            Ok(false)
        }
    }

    /// Draw a set of radio buttons to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { radio: usize };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.radio("Radio 1", &mut self.radio, 0)?;
    ///     s.radio("Radio 2", &mut self.radio, 1)?;
    ///     s.radio("Radio 3", &mut self.radio, 2)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn radio<S>(&mut self, label: S, selected: &mut usize, index: usize) -> PixResult<bool>
    where
        S: AsRef<str>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();

        // Calculate radio rect
        let radio = circle![pos + RADIO_SIZE, RADIO_SIZE];

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, radio);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();

        // Render
        s.rect_mode(RectMode::Corner);
        s.ellipse_mode(EllipseMode::Corner);

        // Checkbox
        if focused || active {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(Cursor::hand())?;
            s.fill(s.secondary_color());
        } else if disabled {
            s.fill(s.primary_color() / 2);
        } else {
            s.fill(s.primary_color());
        }
        s.circle(radio)?;
        if *selected == index {
            s.no_stroke();
            if disabled {
                s.fill(s.highlight_color() / 2);
            } else {
                s.fill(s.highlight_color());
            }
            s.circle([radio.x(), radio.y(), radio.radius() - 2])?;
        }
        s.advance_cursor(radio.bounding_rect());
        s.pop();

        // Label
        s.same_line(None);
        s.text(label)?;

        // Process input
        s.ui.handle_events(id);
        if !disabled {
            let clicked = s.ui.was_clicked(id);
            if clicked {
                *selected = index;
            }
            Ok(clicked)
        } else {
            Ok(false)
        }
    }
}
