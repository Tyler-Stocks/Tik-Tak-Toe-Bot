mod board;
mod io;
mod game;
mod util;

use console::Term;

use crate::game::stages::{get_start_configuration, start};
use crate::util::{Side, StartPlayer};

pub fn main() {
    println!("Welcome to my Tic Tak Toe bot! Press enter to continue and q to quit.");

    let stdout: Term = Term::buffered_stdout();

    start(&stdout);

    let start_configuration: (Side, StartPlayer) = get_start_configuration(&stdout);

    println!("Side: {:?}, Start Player: {:?}", start_configuration.0, start_configuration.1)
}