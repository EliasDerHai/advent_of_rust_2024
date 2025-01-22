use std::fmt::{Display, Formatter};

/// a point (i16, i16) with some convenience
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub(crate) x: i16,
    pub(crate) y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Self {
        Point { x, y }
    }


    pub fn left(&self) -> Point {
        Point { x: self.x - 1, y: self.y }
    }

    pub fn up(&self) -> Point {
        Point { x: self.x, y: self.y - 1 }
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


impl From<&(i16, i16)> for Point {
    fn from(value: &(i16, i16)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

