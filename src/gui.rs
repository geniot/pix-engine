//! Graphical User Interface

use crate::{prelude::*, renderer::Rendering};

impl PixState {
    /// Draw text to the current canvas.
    pub fn text<P>(&mut self, p: P, text: &str) -> PixResult<()>
    where
        P: Into<Point<Primitive>>,
    {
        let s = &self.settings;
        let mut p = p.into();
        if let DrawMode::Center = s.rect_mode {
            let (width, height) = self.renderer.size_of(text)?;
            p = point!(p.x() - width / 2, p.y() - height / 2);
        };
        Ok(self.renderer.text(&p, text, s.fill, s.stroke)?)
    }

    /// Draw a [Button] to the current canvas.
    pub fn button<R>(&mut self, rect: R, label: &str) -> PixResult<bool>
    where
        R: Into<Rect>,
    {
        let rect = rect.into();
        self.push();
        self.stroke(WHITE);
        let hover = rect.contains_point(self.mouse.pos);
        if hover {
            self.fill(NAVY);
            self.frame_cursor(&Cursor::hand())?;
        } else {
            self.fill(GRAY);
        }
        self.rect(rect)?;

        self.rect_mode(DrawMode::Center);
        self.fill(WHITE);
        self.text(rect.center().as_::<i32>(), label)?;

        self.pop();
        if hover && self.mouse.was_clicked(&Mouse::Left) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
