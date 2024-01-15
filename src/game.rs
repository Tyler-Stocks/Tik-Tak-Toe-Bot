use std::process::exit;

use crate::{
    board::Board,
    io::{cls, get_binary_input, get_key, get_num, wait_for_enter},
    logic::do_computer_move,
    random::get_random,
    util::core::{
        Player,
        Player::{Cpu, You},
        Side,
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
fn do_player_turn(term: &Term, board: &mut Board, side: Side) {
    cls(term);

    let msg: &str =
        "Choose a tile to place your piece (1 - 9). You cannot place a piece on an occupied tile.";

    let mut player_move: u8;

    println!("{}", board);

    loop {
        player_move = get_num(term, msg);

        if board.tile_is_empty(player_move) {
            board.make_move(player_move, side);
            break;
        }
    }

    cls(term);
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
fn get_start_configuration(term: &Term) -> (Side, Player) {
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
    let mut start_player: Player;

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
            "Are you happy with the start configuration (Y/N)? \nSide: {}, Start Player: {}",
            side, start_player
        );

        match get_binary_input(term, confirm.as_str(), ['y', 'n'], true) {
            true => return (side, start_player),
            false => continue,
        };
    }
}

/// Main turn loop
///
/// ### Params
///     * term: The terminal you are reading from
///     * start_config: The starting player and turn
pub fn turn_loop(term: &Term, board: &mut Board, start_config: (Side, Player)) -> Player {
    let mut turn_counter: u8 = 1;

    // Turn loop
    loop {
        cls(term);

        match start_config.1 {
            You => {
                if turn_counter % 2 != 0 {
                    do_player_turn(term, board, start_config.0);

                    if board.is_winning() {
                        cls(term);
                        return You;
                    }
                } else {
                    do_computer_move(board, start_config.0.other());

                    if board.is_winning() {
                        return Cpu;
                    }
                }
            }
            Cpu => {
                if turn_counter % 2 != 0 {
                    do_computer_move(board, start_config.0.other());

                    if board.is_winning() {
                        return You;
                    }
                } else {
                    if board.is_winning() {
                        return Cpu;
                    }
                }
            }
        }

        wait_for_enter(term);

        turn_counter += 1;
    }
}

/// Main game loop
///
/// ### Params
///     * term: The terminal you are reading from
pub fn game_loop(term: Term, mut board: Board) -> ! {
    let mut start_config: (Side, Player);

    start(&term);

    loop {
        start_config = get_start_configuration(&term);

        turn_loop(&term, &mut board, start_config);
    }
}
