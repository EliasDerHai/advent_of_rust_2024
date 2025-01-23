use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

/// a point (i32, i32) with some convenience
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
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

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}


impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<&(i32, i32)> for Point {
    fn from(value: &(i32, i32)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

