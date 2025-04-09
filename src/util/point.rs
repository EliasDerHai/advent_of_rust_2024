use crate::util::grid::Direction;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

/// a point (i128, i128) with some convenience - has evolved to be more of a Vec2 but often semantically a Point
/// ... could probably be refactored into a proper distinction between Point and Vec2,
/// but I guess as long as I can solve the aoc I will just keep on going
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Point<T = i32> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T> Point<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + From<i32>,
{
    pub fn left(&self) -> Point<T> {
        Point::new(self.x - T::from(1), self.y)
    }

    pub fn up(&self) -> Point<T> {
        Point::new(self.x, self.y - T::from(1))
    }

    pub fn right(&self) -> Point<T> {
        Point::new(self.x + T::from(1), self.y)
    }

    pub fn down(&self) -> Point<T> {
        Point::new(self.x, self.y + T::from(1))
    }

    pub fn neighbors(&self) -> [Point<T>; 4] {
        [self.up(), self.right(), self.down(), self.left()]
    }
}

impl Point<i32> {
    /// Returns all points within Manhattan distance `n` from `self`
    pub fn proximity_manhattan(&self, n: i32) -> HashSet<Point> {
        (-n..=n)
            .flat_map(|dx| {
                let max_dy = n - dx.abs();
                (-max_dy..=max_dy).map(move |dy| Point::new(self.x + dx, self.y + dy))
            })
            .collect()
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

impl Add<(u8, u8)> for Point<u8> {
    type Output = Point<u8>;

    fn add(self, rhs: (u8, u8)) -> Self::Output {
        Point::new(self.x + rhs.0, self.y + rhs.1)
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point::new(self.x + rhs.0, self.y + rhs.1)
    }
}

impl Add<(i128, i128)> for Point<i128> {
    type Output = Self;

    fn add(self, rhs: (i128, i128)) -> Self::Output {
        Point::new(self.x + rhs.0, self.y + rhs.1)
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

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<(i32, i32)> for Point {
    type Output = Point;

    fn mul(self, rhs: (i32, i32)) -> Self::Output {
        Point::new(self.x * rhs.0, self.y * rhs.1)
    }
}

impl From<&(u128, u128)> for Point<i128> {
    fn from(value: &(u128, u128)) -> Self {
        Point {
            x: value.0 as i128,
            y: value.1 as i128,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn should_give_proximity() {
        let p = Point::new(0, 0);
        let actual = p.proximity_manhattan(1);
        let expected: HashSet<Point> = vec![(0, 0), (1, 0), (0, 1), (0, -1), (-1, 0)]
            .into_iter()
            .map(|(x, y)| Point::new(x, y))
            .collect();

        assert_eq!(expected, actual);
    }
}
