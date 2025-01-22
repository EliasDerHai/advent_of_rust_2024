use std::fmt::{Display, Formatter};

/// a point (u8, u8) with some convenience
/// `left` and `up` might return (u8::max, u8::max) point if overflow
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: u8,
    y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        Point { x, y }
    }


    pub fn left(&self) -> Point {
        let next_x = if self.x == 0 {
            u8::MAX
        } else {
            &self.x - 1
        };
        Point { x: next_x, y: self.y }
    }

    pub fn up(&self) -> Point {
        let next_y = if self.y == 0 {
            u8::MAX
        } else {
            &self.y - 1
        };
        Point { x: self.x, y: next_y }
    }

    pub fn right(&self) -> Point {
        Point { x: self.x + 1, y: self.y }
    }

    pub fn down(&self) -> Point {
        Point { x: self.x, y: self.y + 1 }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", &self.x, &self.y)
    }
}


impl From<&(u8, u8)> for Point{
    fn from(value: &(u8, u8)) -> Self {
        Point {
            x: value.0,
            y: value.1
        }
    }
}

