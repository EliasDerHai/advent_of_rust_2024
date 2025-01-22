use std::collections::hash_map::IntoIter;
use std::collections::HashMap;

use crate::util::grid::Direction::*;
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

#[derive(Debug)]
pub struct CharGrid {
    map: HashMap<Point, char>,
}

impl From<&str> for CharGrid {
    fn from(value: &str) -> Self {
        let map: HashMap<Point, char> =
            value
                .lines()
                .enumerate()
                .into_iter()
                .flat_map(|(y, line)| line
                    .chars()
                    .enumerate()
                    .map(move |(x, c)|
                        {
                            let p = Point::new(x as i16, y as i16);
                            (p, c)
                        }
                    )
                )
                .collect();
        CharGrid { map }
    }
}

impl IntoIterator for CharGrid {
    type Item = (Point, char);
    type IntoIter = IntoIter<Point, char>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl CharGrid {
    pub fn iter(&self) -> impl Iterator<Item=(&Point, &char)> {
        self.map.iter()
    }

    pub fn get(&self, p: &Point) -> Option<&char> {
        self.map.get(p)
    }

    pub fn neighbors<'a>(&'a self, p: &'a Point)
                         -> impl Iterator<Item=(Point, char)> + 'a {
        [p.left(), p.right(), p.up(), p.down()]
            .into_iter()
            .map(|n| (n, self.get(&n)))
            .filter_map(|(n, c)|
                c.map(|inner_c| (n, *inner_c))
            )
    }
}
