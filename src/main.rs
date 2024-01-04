mod board;
mod game;
mod io;
mod util;

use crate::game::stages::game_loop;
use console::Term;

pub fn main() {
    println!("Welcome to my Tic Tak Toe bot! Press enter to continue and q to quit.");

    let term: Term = Term::buffered_stdout();

    game_loop(&term);
}

