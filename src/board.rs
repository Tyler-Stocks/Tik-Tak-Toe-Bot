#![allow(dead_code)]
use std::{
    fmt::{Display, Formatter, Result},
    u8,
};

use crate::{
    board::{TileState::Empty, TileState::Occupied},
    util::core::{Side, Side::O, Side::X, Turn},
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

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Tile {
    id: u8,
    state: TileState,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.id, self.state)
    }
}

impl Tile {
    pub fn new(id: u8) -> Tile {
        if id > 9 {
            panic!("Id {id} is too big. Must be less than 9.");
        }

        if id < 1 {
            panic!("Id {id} is too small. Must be greater than 0.");
        }

        Tile {
            id,
            state: TileState::Empty,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Board {
    pub start_player: Turn,
    pub top_left: Tile,
    pub top_middle: Tile,
    pub top_right: Tile,
    pub middle_left: Tile,
    pub middle_middle: Tile,
    pub middle_right: Tile,
    pub bottom_left: Tile,
    pub bottom_middle: Tile,
    pub bottom_right: Tile,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Start Player: {:?}\n{} {} {}\n{} {} {}\n{} {} {}",
            self.start_player,
            self.top_left,
            self.top_middle,
            self.top_right,
            self.middle_left,
            self.middle_middle,
            self.middle_right,
            self.bottom_left,
            self.bottom_middle,
            self.bottom_right
        )
    }
}

impl Board {
    /// Creates a new tick tack toe board
    pub fn new(start_player: Turn) -> Board {
        Board {
            start_player,
            top_left: Tile::new(1),
            top_middle: Tile::new(2),
            top_right: Tile::new(3),
            middle_left: Tile::new(4),
            middle_middle: Tile::new(5),
            middle_right: Tile::new(6),
            bottom_left: Tile::new(7),
            bottom_middle: Tile::new(8),
            bottom_right: Tile::new(9),
        }
    }

    pub fn do_turn(&mut self, id: u8, side: Side) {
        match side {
            X => self.modify_tile_state(id, Occupied(X)),
            O => self.modify_tile_state(id, Occupied(O)),
        }
    }

    pub fn is_empty(&mut self, id: u8) -> bool {
        self.get_tile_from_id(id).state == Empty
    }

    pub fn modify_tile_state(&mut self, id: u8, state: TileState) {
        match id {
            1 => self.top_left.state = state,
            2 => self.top_middle.state = state,
            3 => self.top_right.state = state,
            4 => self.middle_left.state = state,
            5 => self.middle_middle.state = state,
            6 => self.middle_right.state = state,
            7 => self.bottom_left.state = state,
            8 => self.bottom_middle.state = state,
            9 => self.bottom_right.state = state,
            _ => panic!("Tile with id {id} does not exist."),
        }
    }

    pub fn get_tile_from_id(&self, id: u8) -> Tile {
        match id {
            1 => self.top_left,
            2 => self.top_middle,
            3 => self.top_right,
            4 => self.middle_left,
            5 => self.middle_middle,
            6 => self.middle_right,
            7 => self.bottom_left,
            8 => self.bottom_middle,
            9 => self.bottom_right,
            _ => panic!("Tile with id {id} does not exist."),
        }
    }
}
