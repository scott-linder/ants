//! 2D coordinates.

use std::ops::Add;
use std::fmt;
use ants::direction::Direction;

/// An location or offset in the world's map.
///
/// The top left corner of the map is `Point { row: 0, col: 0 }`, with rows
/// incrementing down and cols incrementing to the right.
#[derive(Default, Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

fn wrap(n: i32, max: i32) -> i32 {
    if n < 0 {
        n % max + max
    } else {
        n % max
    }
}

impl Point {
    /// Calculate a point corrected for overflow or underflow.
    ///
    /// The map is a torus, so a point which overflows should start back at
    /// 0, and one that underflows should start back at the max for that
    /// dimension.
    pub fn wrap(&self, rows: i32, cols: i32) -> Point {
        Point {
            row: wrap(self.row, rows),
            col: wrap(self.col, cols),
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Point {
        self + rhs.into_point()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.row, self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap() {
        let p = Point{ row: 2, col: -1 };
        assert_eq!(p.wrap(2, 2), Point { row: 0, col: 1 });
    }
}
