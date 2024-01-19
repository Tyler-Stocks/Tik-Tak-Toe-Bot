use std::fmt::{Display, Formatter, Result};

use crate::util::{
    side::{Side, Side::{X, O}},
    game_result::GameResult::{Unfinished, Tie, Win}
};


#[derive(Copy, Clone, Eq, PartialEq)] 
pub enum GameResult {
    Win(Side),
    Tie,
    Unfinished
}

impl Display for GameResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Win(side) => {
                match side {
                    X => write!(f, "X Wins!"),
                    O => write!(f, "O Wins!")
                }
            },
            Tie => write!(f, "It's a tie!. Nobody wins :("),
            Unfinished => write!(f, "The game is not over yet, who knows what will happen next.")
        }
    }
}
