mod board;
mod game;
mod io;
mod logic;
mod util;

use crate::game::game_loop;
use console::Term;

pub fn main() -> ! {
    println!("Welcome to my Tic Tak Toe bot! Press enter to continue and q to quit.");

    game_loop(&Term::buffered_stdout());
}
