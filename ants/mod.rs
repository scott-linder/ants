//! Rust library for the [ants ai challenge](http://ants.aichallenge.org/).
//!
//! Most users will only need to implement `bot::Bot`'s `do_turn` method.
//!
//! # Example
//!
//! ```
//! extern crate ants;
//!
//! use ants::ant::Ant;
//! use ants::bot::Bot;
//! use ants::direction::Direction;
//! use ants::error::Result;
//! use ants::player::Player;
//! use ants::tile::Tile;
//! use ants::world::World;
//!
//! struct MyBot;
//!
//! impl Bot for MyBot {
//!     fn do_turn(&mut self, world: &mut World) -> Result<()> {
//!         let direction = Direction::North;
//!         for point in world.map.tiles().filter_map(|(point, &tile)| {
//!                     if tile == Some(Tile::Ant(Ant{ alive: true, owner: Player::Me })) {
//!                         Some(point)
//!                     } else {
//!                         None
//!                     }
//!                 }) {
//!             if world.map[point + direction].unwrap_or(Tile::Land).is_passable() {
//!                 world.order(point, direction);
//!             }
//!         }
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     let mut bot = MyBot;
//!     bot.run().unwrap();
//! }
//! ```

#![allow(unused)]

pub mod error;
pub mod params;
pub mod world;
pub mod tile;
pub mod point;
pub mod player;
pub mod ant;
pub mod direction;
pub mod map;
pub mod bot;
