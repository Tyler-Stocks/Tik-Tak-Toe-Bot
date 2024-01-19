use std::fmt::{Display, Formatter, Result};

use Player::{You, Cpu};

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
