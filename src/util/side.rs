use std::fmt::{Display, Formatter, Result};

use Side::{X, O};

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
