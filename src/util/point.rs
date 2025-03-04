use crate::util::grid::Direction;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

/// a point (i128, i128) with some convenience
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub(crate) x: i128,
    pub(crate) y: i128,
}

impl Point {
    pub fn new(x: i128, y: i128) -> Self {
        Point { x, y }
    }

    pub fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", &self.x, &self.y)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::N => self.up(),
            Direction::NE => self.up().right(),
            Direction::E => self.right(),
            Direction::SE => self.down().right(),
            Direction::S => self.down(),
            Direction::SW => self.down().left(),
            Direction::W => self.left(),
            Direction::NW => self.up().left(),
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, rhs: usize) -> Self::Output {
        Point::new(self.x * rhs as i128, self.y * rhs as i128)
    }
}

impl From<&(u128, u128)> for Point {
    fn from(value: &(u128, u128)) -> Self {
        Point {
            x: value.0 as i128,
            y: value.1 as i128,
        }
    }
}
