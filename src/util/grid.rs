use std::collections::hash_map::IntoIter;
use std::collections::HashMap;

use crate::util::point::Point;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    map: HashMap<Point, T>,
}

impl From<&str> for Grid<char> {
    fn from(value: &str) -> Self {
        let map: HashMap<Point, char> = value
            .lines()
            .enumerate()
            .into_iter()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    let p = Point::new(x as i32, y as i32);
                    (p, c)
                })
            })
            .collect();
        Grid { map }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (Point, T);
    type IntoIter = IntoIter<Point, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<T> Grid<T> {
    pub fn new(map: HashMap<Point, T>) -> Self {
        Grid { map }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Point, &T)> {
        self.map.iter()
    }

    pub fn get(&self, p: &Point) -> Option<&T> {
        self.map.get(p)
    }

    pub fn set(&mut self, p: Point, value: T) {
        self.map.insert(p, value);
    }

    pub fn neighbors(&self, p: &Point) -> impl Iterator<Item = (Point, &T)> {
        [p.left(), p.right(), p.up(), p.down()]
            .into_iter()
            .map(|n| (n, self.get(&n)))
            .filter_map(|(n, c)| c.map(|inner_c| (n, inner_c)))
    }

    pub fn map<F, U>(self, mut map_fn: F) -> Grid<U>
    where
        F: FnMut(T) -> U,
    {
        let next: HashMap<Point, U> = self.into_iter().map(|(p, t)| (p, map_fn(t))).collect();

        Grid { map: next }
    }

    pub fn filter_map<F, U>(self, mut map_fn: F) -> Grid<U>
    where
        F: FnMut(T) -> Option<U>,
    {
        let next: HashMap<Point, U> = self
            .into_iter()
            .filter_map(|(p, t)| map_fn(t).map(|u| (p, u)))
            .collect();

        Grid { map: next }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_map() {
        let chars = "
012
345
678"
        .trim();
        let original: Grid<char> = Grid::from(chars);
        let numeric: Grid<u8> = original.map(|v| v.to_string().parse::<u8>().unwrap());
        assert_eq!(36u8, numeric.map.iter().map(|(_, v)| v).sum::<u8>());
    }
}
