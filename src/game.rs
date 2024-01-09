use std::process::exit;

use crate::{
    board::board::Board,
    io::{get_binary_input, get_key, get_num, wait_for_enter},
    util::{
        core::{
            Side, Turn,
            Turn::{Cpu, You},
        },
        random::calculate_random,
    },
};

use console::{
    Key::{Char, Enter},
    Term,
};

fn do_player_turn(term: &Term, board: &mut Board, side: &Side) {
    let msg: &str =
        "Choose a tile to place your piece (1 - 9). You cannot place a piece on an occupied tile.";

    let mut player_move: u8;

    loop {
        player_move = get_num(term, msg);

        if board.is_empty(player_move) {
            board.populate_tile(player_move, side);
            return;
        }
    }
}

fn start(term: &Term) {
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
            true => calculate_random(term, random_side_msgs),
            false => get_binary_input(term, query_side_msg, ['x', 'o'], false),
        };

        start_player = match get_binary_input(term, is_start_random_msg, ['y', 'n'], false) {
            true => calculate_random(term, random_start_player_msgs),
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

pub fn game_loop(term: &Term) -> ! {
    let mut start_config: (Side, Turn);

    start(term);

    loop {
        start_config = get_start_configuration(term);

        game_loop(term);
    }
}
