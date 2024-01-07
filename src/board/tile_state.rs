use std::fmt::{Display, Formatter, Result};

use crate::{
    board::tile_state::TileState::{Empty, Occupied},
    util::core::Side,
};

/// The current state of the tile on the board
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum TileState {
    Occupied(Side),
    Empty,
}

impl Display for TileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Occupied(X) => write!(f, "State = Occupied(X)"),
            Occupied(O) => write!(f, "State = Occupied(O)"),
            Empty => write!(f, "State = Empty"),
        }
    }
}

impl TileState {
    pub fn value(&self) -> u8 {
        match self {
            Empty => 0,
            Occupied(X) => 1,
            Occupied(O) => 2,
        }
    }
}
