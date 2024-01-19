mod board;
mod game;
mod io;
mod logic;
mod random;
mod util;

use console::Term;

use crate::{
    board::Board,
    game::turn_loop,
    util::{
        player::{
            Player, 
            Player::You
        },
        side::{
            Side,
            Side::X
        }
    }
};

pub fn main() {
    let term: Term = Term::buffered_stdout();
    let mut board: Board = Board::new();

    let start_config: (Side, Player) = (X, You);

    let winner: Player = turn_loop(&term, &mut board, start_config);

    println!("{winner}");
}
