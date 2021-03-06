//! Cardinal directions.

use std::fmt;
use ants::point::Point;

/// Represents the four cardinal directions in the game.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Convert the direction into a 'unit point'.
    ///
    /// The result can be added to a `Point` to offset it one unit in any
    /// direction.
    pub fn into_point(self) -> Point {
        match self {
            Direction::North => Point { row: -1, col: 0 },
            Direction::South => Point { row: 1, col: 0 },
            Direction::East => Point { row: 0, col: 1 },
            Direction::West => Point { row: 0, col: -1 },
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", match *self {
            Direction::North => "n",
            Direction::South => "s",
            Direction::East => "e",
            Direction::West => "w",
        })
    }
}
