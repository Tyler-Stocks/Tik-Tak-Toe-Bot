use std::fmt::{Display, Formatter, Result};

use Player::{Cpu, You};
use Side::{O, X};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Side {
    X,
    O,
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            X => write!(f, "Side: X"),
            O => write!(f, "Side: O"),
        }
    }
}

impl Side {
    pub fn other(&self) -> Side {
        match self {
            X => O,
            O => X,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Player {
    You,
    Cpu,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            You => write!(f, "Turn: You"),
            Cpu => write!(f, "Turn: Cpu"),
        }
    }
}
