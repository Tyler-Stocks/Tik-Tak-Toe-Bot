// ---------- Modules ---------- //
mod board;
mod game;
mod io;
mod logic;
mod util;

// -------- Imports ---------- //
use crate::game::stages::game_loop;
use console::Term;

// ---------- Main ---------- //
pub fn main() -> ! {
    println!("Welcome to my Tic Tak Toe bot! Press enter to continue and q to quit.");

    let term: Term = Term::buffered_stdout();

    game_loop(&term);
}
