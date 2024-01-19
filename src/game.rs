use std::process::exit;

use crate::{
    board::Board,
    io::{cls, get_binary_input, get_key, get_num, wait_for_enter, confirm},
    logic::do_computer_move,
    random::get_random,
    util::{
        player::{
            Player,
            Player::{You, Cpu}
        },
        side::Side,
        game_result::{
            GameResult,
            GameResult::Unfinished
        }, start_config::StartConfig
    }
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
            _ => continue,
        }
    }
}

/// Gets the starting configuration from the user
///
/// ### Params
///     * term: The terminal you are reading from
fn get_start_configuration(term: &Term) -> StartConfig {
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

    let mut start_side: Side;
    let mut start_player: Player;

    loop {
        match get_binary_input::<bool>(term, is_side_random_msg, ['y', 'n']) {
            true => start_side = get_random(term, random_side_msgs),
            false => start_side = get_binary_input(term, query_side_msg, ['y', 'n']) 
        }

        match get_binary_input::<bool>(term, is_start_random_msg, ['y', 'n']) {
            true => start_player = get_random(term, random_side_msgs),
            false => start_player = get_binary_input(term, query_start_player, ['x', 'o'])
        }

        let confirm_msg: &str = 
            format!("Are you happy with the start configuration (Y/N)? \nSide: {}, Start Player: {}", side, start_player).as_str();

        if confirm(term, confirm_msg) {
            return StartConfig::new(start_side, start_player);
        }
    }
}

/// Main turn loop
///
/// ### Params
///     * term: The terminal you are reading from
///     * start_config: The starting player and turn
pub fn turn_loop(term: &Term, board: &mut Board, start_config: StartConfig) -> GameResult {
    let mut turn_counter: u8 = 1;

    // Turn loop
    loop {
        cls(term);

        match start_config.start_player {
            You => {
                if turn_counter % 2 != 0 {
                    do_player_turn(term, board, start_config.start_side);
                } else {
                    do_computer_move(term, board, start_config.start_side.other());
                }
            }
            Cpu => {
                if turn_counter % 2 != 0 {
                    do_computer_move(term, board, start_config.start_side.other());
                } else {
                    do_player_turn(term, board, start_config.start_side)
                }  
            }
        }

        match board.get_game_state() {
           Unfinished => (),
           _          => return board.get_game_state()
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
    let mut start_config: StartConfig; 

    start(&term);

    loop {
        start_config = get_start_configuration(&term);

        turn_loop(&term, &mut board, start_config);
    }
}
