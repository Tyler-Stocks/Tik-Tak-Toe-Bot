mod board;
mod game;
mod io;
mod logic;
mod random;
mod util;

use crate::{board::Board, util::core::Side::X};

pub fn main() {
    let mut board: Board = Board::new();

    println!("{board}");

    board.make_move(8, X);

    println!("{board}");
}
