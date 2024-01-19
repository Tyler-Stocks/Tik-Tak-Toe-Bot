use std::fmt::{Display, Formatter, Result};

use super::{side::Side, player::Player};

pub struct StartConfig {
    pub start_side: Side,
    pub start_player: Player
}

impl Display for StartConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}", self.start_side, self.start_player)
    }
}

impl StartConfig {

    pub fn new(side: Side, player: Player) -> StartConfig {
        StartConfig {
            start_side:   side,
            start_player: player
        }
    }
}


