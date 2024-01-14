use std::process::exit;

use crate::{
    board::Board,
    io::{get_binary_input, get_key, get_num},
    random::get_random,
    util::core::{
        Side, Turn,
        Turn::{Cpu, You},
    },
};

use console::{
    Key::{Char, Enter},
    Term,
};

/// Prompts the user to play a move, then plays the move
///
/// ### Params
///     * term: The terminal you are reading input from
///     * board: The tic tac toe board
///     * side: The player side
fn do_player_turn(term: &Term, board: &mut Board, side: &Side) {
    let msg: &str =
        "Choose a tile to place your piece (1 - 9). You cannot place a piece on an occupied tile.";

    let mut player_move: u8;

    loop {
        player_move = get_num(term, msg);

        if board.tile_is_empty(player_move) {
            board.make_move(player_move, *side);
            return;
        }
    }
}

/// Waits for the user to start
///
/// ### Params
///     * term: The terminal you are reading from
fn start(term: &Term) {
    let start_msg: &str = "Press enter to start or 'q' to quit";

    println!("{start_msg}");

    loop {
        match get_key(term) {
            Enter => break,
            Char(c) => {
                if c == 'q' {
                    exit(0)
                }
            }
            _ => (),
        }
    }
}

/// Gets the starting configuration from the user
///
/// ### Params
///     * term: The terminal you are reading from
fn get_start_configuration(term: &Term) -> (Side, Turn) {
    let is_side_random_msg: &str = "Randomize starting side? (Y/N)";
    let is_start_random_msg: &str = "Randomize starting player? (Y/N)";
    let query_side_msg: &str = "Which side would you like to be (X/O)?";
    let query_start_player: &str = "Would you like to start? (Y/N)";

    let random_side_msgs: [&str; 2] = [
        "You are X! Press enter to continue.",
        "You are O! Press enter to continue",
    ];

    let random_start_player_msgs: [&str; 2] = [
        "The computer starts! Uh oh. Press enter to continue",
        "You start! Now you many have a chance",
    ];

    let mut side: Side;
    let mut start_player: Turn;

    loop {
        side = match get_binary_input(term, is_side_random_msg, ['y', 'n'], false) {
            true => get_random(term, random_side_msgs),
            false => get_binary_input(term, query_side_msg, ['x', 'o'], false),
        };

        start_player = match get_binary_input(term, is_start_random_msg, ['y', 'n'], false) {
            true => get_random(term, random_start_player_msgs),
            false => get_binary_input(term, query_start_player, ['y', 'n'], false),
        };

        let confirm: String = format!(
            "Are you happy with the start configuration (Y/N)? \nSide: {:?}, Start Player: {:?}",
            side, start_player
        );

        match get_binary_input(term, confirm.as_str(), ['y', 'n'], true) {
            true => (side, start_player),
            false => continue,
        };
    }
}

/// Main turn loop
///
/// ### Params
///     * term: The terminal you are reading from
///     * start_config: The starting player and turn
fn turn_loop(term: &Term, start_config: (Side, Turn)) {
    // Turn loop
    loop {
        let mut turn_counter: u8 = 1;

        match start_config.1 {
            You => {
                if turn_counter % 2 != 0 {
                    todo!()
                } else {
                    todo!()
                }
            }
            Cpu => {
                if turn_counter % 2 != 0 {
                    todo!()
                } else {
                    todo!()
                }
            }
        }
    }
}

/// Main game loop
///
/// ### Params
///     * term: The terminal you are reading from
pub fn game_loop(term: &Term) -> ! {
    let mut start_config: (Side, Turn);

    start(term);

    loop {
        start_config = get_start_configuration(term);

        turn_loop(term, start_config);
    }
}
