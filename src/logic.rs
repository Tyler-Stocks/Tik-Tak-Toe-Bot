use crate::{board::board::Board, util::core::Turn};

pub fn do_computer_turn(board: &mut Board, turn_number: u8) {
    match turn_number {
        2 => do_turn_two(board),
        3 => do_turn_three(board),
        4 => do_turn_four(board),
        5 => do_turn_five(board),
        6 => do_turn_six(board),
        7 => do_turn_seven(board),
        8 => do_turn_eight(board),
        9 => do_turn_nine(board),
        _ => panic!("Turn number out of bounds. Must be in range 1 - 9."),
    }
}

fn do_turn_one(board: &mut Board, player: &Turn) {
    match player {
        Turn::You => todo!(),
        Turn::Cpu => todo!(),
    }
}

fn do_turn_two(board: &mut Board) {
    todo!();
}

fn do_turn_three(board: &mut Board) {
    todo!();
}

fn do_turn_four(board: &mut Board) {
    todo!();
}

fn do_turn_five(board: &mut Board) {
    todo!();
}

fn do_turn_six(board: &mut Board) {
    todo!();
}

fn do_turn_seven(board: &mut Board) {
    todo!();
}

fn do_turn_eight(board: &mut Board) {
    todo!();
}

fn do_turn_nine(board: &mut Board) {
    todo!();
}
