#![allow(dead_code)]
use std::fmt::{Display, Formatter};
use crate::board::TileId::{BottomLeft,
                           BottomMiddle,
                           BottomRight,
                           MiddleLeft,
                           MiddleMiddle,
                           MiddleRight,
                           TopLeft,
                           TopMiddle,
                           TopRight
};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum TileState {
    X,
    O,
    Empty
}

impl Display for TileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileState::X     => write!(f, "State = X"),
            TileState::O     => write!(f, "State = O"),
            TileState::Empty => write!(f, "State = Empty")
        }
    }
}

impl TileState {
    pub fn value(&self) -> u8 {
        match self {
            TileState::Empty => 0,
            TileState::X     => 1,
            TileState::O     => 2
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum TileId {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    MiddleMiddle,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight
}


impl Display for TileId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TopLeft      => write!(f, "Id = Top Left"),
            TopMiddle    => write!(f, "Id = Top Right"),
            TopRight     => write!(f, "Id = Top Right"),
            MiddleLeft   => write!(f, "Id = Middle Left"),
            MiddleMiddle => write!(f, "Id = Middle Middle"),
            MiddleRight  => write!(f, "Id = Middle Right"),
            BottomLeft   => write!(f, "Id = Bottom Left"),
            BottomMiddle => write!(f, "Id = Bottom Middle"),
            BottomRight  => write!(f, "Id = Bottom Right")
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Tile {
    pub id:    TileId,
    pub state: TileState
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.id, self.state)
    }
}

impl Tile {
    pub fn new(id: TileId) -> Tile {
        Tile {
            id,
            state: TileState::Empty
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Board {
    pub top_left:      Tile,
    pub top_middle:    Tile,
    pub top_right:     Tile,
    pub middle_left:   Tile,
    pub middle_middle: Tile,
    pub middle_right:  Tile,
    pub bottom_left:   Tile,
    pub bottom_middle: Tile,
    pub bottom_right:  Tile
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{} {} {}\n{} {} {}\n{} {} {}",
            self.top_left, self.top_middle, self.top_right,
            self.middle_left, self.middle_middle, self.middle_right,
            self.bottom_left, self.bottom_middle, self.bottom_right
        )
    }
}

impl Board {

    /// Creates a new tick tack toe board
    pub fn new() -> Board {
        Board {
            top_left:      Tile::new(TopLeft),
            top_middle:    Tile::new(TopMiddle),
            top_right:     Tile::new(TopRight),
            middle_left:   Tile::new(MiddleLeft),
            middle_middle: Tile::new(MiddleMiddle),
            middle_right:  Tile::new(MiddleRight),
            bottom_left:   Tile::new(BottomLeft),
            bottom_middle: Tile::new(BottomMiddle),
            bottom_right:  Tile::new(BottomRight)
        }
    }

    /// Changes the state of a single tile
    pub fn change_state(&mut self, id: TileId, state: TileState) {
        match id {
            TopLeft      => self.top_left.state      = state,
            TopMiddle    => self.top_middle.state    = state,
            TopRight     => self.top_right.state     = state,
            MiddleLeft   => self.middle_left.state   = state,
            MiddleMiddle => self.middle_middle.state = state,
            MiddleRight  => self.middle_right.state  = state,
            BottomLeft   => self.bottom_left.state   = state,
            BottomMiddle => self.bottom_middle.state = state,
            BottomRight  => self.bottom_right.state  = state
        }
    }

}