#![allow(dead_code)]
use std::{
    fmt::{Display, Formatter, Result},
    u8,
};

use crate::board::tile_state::{TileState, TileState::Empty};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Tile {
    id: u8,
    pub state: TileState,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}", self.id, self.state)
    }
}

impl Tile {
    /// Constructs a new tile.
    ///
    /// ### Params
    ///     * id: The tile Id. Tiles are arranged in order and the id will be clipped to be between
    ///     1 and 9
    pub fn new(mut id: u8) -> Tile {
        if id == 0 || id > 9 {
            panic!("Id {id} is out of range. Id must be in range 1 - 9.");
        }

        Tile { id, state: Empty }
    }

    /// Returns the id of the tile
    pub fn id(&self) -> u8 {
        self.id
    }
}
