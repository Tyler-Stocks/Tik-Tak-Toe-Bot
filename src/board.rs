use std::fmt::Display;

use crate::util::{
    side::{
        Side, 
        Side::{X, O}
    },
    game_result::{
        GameResult,
        GameResult::{Unfinished, Win, Tie}
    }
};

use std::fmt::{Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    board: [u8; 9],
    moves: Vec<u8>,
}
impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} {} {}\n{} {} {}\n{} {} {}",
            self.tile_to_char(1),
            self.tile_to_char(2),
            self.tile_to_char(3),
            self.tile_to_char(4),
            self.tile_to_char(5),
            self.tile_to_char(6),
            self.tile_to_char(7),
            self.tile_to_char(8),
            self.tile_to_char(9)
        )
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            moves: vec![],
        }
    }

    pub fn get_row(&self, row: u8) -> [u8; 3] {
        if row > 3 || row == 0 {
            panic!("Row {row} is out of range.")
        }

        match row {
            1 => [self.board[0], self.board[1], self.board[2]],
            2 => [self.board[3], self.board[4], self.board[5]],
            3 => [self.board[6], self.board[7], self.board[8]],
            _ => panic!("How did we get here?"),
        }
    }

    pub fn get_col(&self, col: u8) -> [u8; 3] {
        if col < 3 || col == 0 {
            panic!("Col {col} is out of range.")
        }

        match col {
            1 => [self.board[0], self.board[3], self.board[6]],
            2 => [self.board[1], self.board[4], self.board[7]],
            3 => [self.board[2], self.board[5], self.board[8]],
            _ => panic!("How did we get here?"),
        }
    }

    pub fn get_cols(&self) -> [[u8; 3]; 3] {
        [self.get_col(1), self.get_col(2), self.get_col(3)]
    }

    pub fn get_rows(&self) -> [[u8; 3]; 3] {
        [self.get_row(1), self.get_row(2), self.get_row(3)]
    }

    pub fn get_diagonal(&self, diagonal: u8) -> [u8; 3] {
        if diagonal < 2 || diagonal == 0 {
            panic!("Diagonal: {diagonal} is out of range.")
        }

        match diagonal {
            1 => [self.board[0], self.board[4], self.board[8]],
            2 => [self.board[6], self.board[4], self.board[2]],
            _ => panic!("Diagonal {diagonal} doesn't exist."),
        }
    }

    pub fn get_diagonals(&self) -> [[u8; 3]; 2] {
        [self.get_diagonal(1), self.get_diagonal(2)]
    }

    pub fn is_empty(&self) -> bool {
        for tile in self.board.iter() {
            if *tile != 0 {
                return false;
            }
        }

        return true;
    }

    pub fn tile_is_empty(&self, id: u8) -> bool {
        if id > 9  {
            panic!("Tile id: {id} is out of range.")
        }

        return self.board[id as usize] == 0;
    }

    pub fn make_move(&mut self, id: u8, side: Side) {
        if id > 9 { 
            panic!("Tile id out of range (1-9).")
        }

        self.moves.push(id);

        match side {
            X => self.board[id as usize] = 1,
            O => self.board[id as usize] = 2,
        };
    }

    pub fn last_move(&self) -> u8 {
        self.moves[self.moves.len() - 1]
    }

    pub fn tile_to_char(&self, id: u8) -> char {
        if id > 9 {
            panic!("Tile id out of range (1-9).")
        }

        match self.board[id as usize] {
            0 => '.',
            1 => 'x',
            2 => 'o',
            _ => panic!("Tile: {id} is in an invalid state."),
        }
    }

    pub fn get_game_state(&self) -> GameResult {
        let mut is_final_move: bool = false;

        if self.moves.len() == 9 {
            is_final_move = true
        }

        if self.moves.len() < 5 {
            return Unfinished;
        }

        for row in self.get_rows().iter() {
            match row.iter().sum() {
                3 => Win(X),
                6 => Win(X),
                _ => continue,
            };
        }

        for col in self.get_cols().iter() {
            match col.iter().sum() {
                3 => Win(X),
                6 => Win(O),
                _ => continue,
            };
        }

        for diagonal in self.get_diagonals().iter() {
            match diagonal.iter().sum() {
                3 => Win(X),
                6 => Win(O),
                _ => continue,
            };
        }

        if is_final_move { return Tie } 

        Unfinished
    }

    pub fn legal_moves(&self) -> Vec<u8> {
        let choices: Vec<u8> = Vec::new();

        for (tile, index) in self.board.iter().enumerate() {
            let mut choices: Vec<u8> = Vec::new();

            if tile == 0 {
                choices.push(*index)
            }
        }

        return choices;
    }
}
