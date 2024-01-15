use console::Term;

use crate::{board::Board, io::cls, util::core::Side};
use rand::{thread_rng, Rng};

pub fn do_computer_move(term: &Term, board: &mut Board, side: Side) {
    let msg: &str = "Doing Computer Move...";

    let legal_moves: Vec<u8> = board.legal_moves();

    cls(term);

    for legal_move in &legal_moves {
        let mut board_copy: Board = board.clone();

        board_copy.make_move(*legal_move, side);

        if board_copy.is_winning() {
            board.make_move(*legal_move, side);
            return;
        }
    }

    let random_move: u8 = thread_rng().gen_range(0..legal_moves.len() - 1) as u8;

    board.make_move(random_move, side)
}
