use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct TileAlreadyOccupiedError;

impl Display for TileAlreadyOccupiedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile Already Occupied")
    }
}

impl Error for TileAlreadyOccupiedError {}
