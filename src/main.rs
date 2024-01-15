mod board;
mod game;
mod io;
mod logic;
mod random;
mod util;

use console::Term;
use util::core::{Side, Turn};

use crate::{board::Board, game::turn_loop};

pub fn main() {
    let term: Term = Term::buffered_stdout();
    let mut board: Board = Board::new();

    let start_config: (Side, Turn) = (Side::X, Turn::You);

    let winner: Turn = turn_loop(&term, &mut board, start_config);

    println!("{winner}");
}
