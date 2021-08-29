use pix_engine::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North = 0,
    East,
    South,
    West,
}
use Direction::*;

use crate::SIZE;

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cell {
    pub id: usize,
    col: Primitive,
    row: Primitive,
    rect: Rect,
    pub walls: [bool; 4],
}

impl Cell {
    pub fn new(id: usize, col: Primitive, row: Primitive) -> Self {
        Self {
            id,
            col,
            row,
            rect: square!(col * SIZE, row * SIZE, SIZE).as_(),
            walls: [true; 4],
        }
    }

    pub fn col(&self) -> Primitive {
        self.col
    }

    pub fn row(&self) -> Primitive {
        self.row
    }

    pub fn remove_wall(&mut self, direction: Direction) {
        self.walls[direction as usize] = false;
    }

    pub fn draw<C: Into<Color>>(&self, s: &mut PixState, color: C) -> PixResult<()> {
        let color = color.into();
        s.fill(color);
        s.no_stroke();
        s.rect(self.rect)?;
        s.no_fill();
        s.stroke(WHITE);
        let top = self.rect.top();
        let right = self.rect.right();
        let bottom = self.rect.bottom();
        let left = self.rect.left();
        for (i, _) in self.walls.iter().enumerate().filter(|(_, n)| **n) {
            match i {
                0 => s.line([left, top, right, top])?,
                1 => s.line([right, top, right, bottom])?,
                2 => s.line([left, bottom, right, bottom])?,
                3 => s.line([left, top, left, bottom])?,
                _ => (),
            }
        }
        Ok(())
    }
}
