#![allow(dead_code)]

use crate::util::traits::TwoOptions;
use console::{Key, Key::Enter, Term};
use core::panic;
use std::process::exit;

/// Clears the terminal.
///
/// ### Params
///     * term: A referance to the terminal you would like to clear
///
/// ### Panics
///     After five failed attempts to clear the terminal
pub fn cls(term: &Term) {
    let err_msg: &str = "Failed to clear the terminal after five attempts.";

    for _ in 0..5 {
        match term.clear_screen() {
            Ok(_) => return,
            Err(_) => (),
        }
    }

    panic!("{err_msg}");
}

/// Gets a number from the terminal
///
/// ### Params
///     * term: A referance to the terminal you are reading from
pub fn get_num(term: &Term, msg: &str) -> u8 {
    let mut user_input: char;

    cls(term);

    println!("{msg}");

    loop {
        user_input = get_char(term);

        if user_input.is_digit(10) {
            match user_input.to_digit(10) {
                Some(digit) => return digit as u8,
                None => (),
            }
        }
    }
}

/// Gets a character from the terminal
///
/// ### Params
///   * term: A referance to the terminal you are reading from
/// ### Panics
///
pub fn get_char(term: &Term) -> char {
    let err_msg: &str = "Failed to read terminal after 5 attempts.";

    for _ in 0..5 {
        match term.read_char() {
            Ok(char) => char,
            Err(_) => continue,
        };
    }

    panic!("{err_msg}");
}

pub fn get_key(term: &Term) -> Key {
    let err_msg: &str = "Failed to read key after 5 attempts";

    for _ in 0..5 {
        match term.read_key() {
            Ok(key) => key,
            Err(_) => continue,
        };
    }

    panic!("{err_msg}");
}

/// Waits for the enter key to be pressed on the keyboard.
/// This function is by nature blocking
///
/// ### Params
///     * term: A referance to the terminal you are getting input from
pub fn wait_for_enter(term: &Term) {
    loop {
        if get_key(term) == Enter {
            break;
        }
    }
}

/// Asks the user to confirm an input.
///
/// ### Params
///     * term: A referance to the terminal you are getting input from
fn confirm(term: &Term) -> bool {
    cls(term);

    println!("Are you sure (Y/N)?");

    loop {
        match get_char(term) {
            'q' => exit(0),
            'y' => return true,
            'n' => return false,
            _ => (),
        };
    }
}

/// Gets an input from the user that only has two possible choices
///
/// ### Parms
///     * stdout: A referance to the terminal you are getting input from
///     * msg: The message to diplay when obtaining user input
///     * keys: The keys to represented the choice
///     * confirm_input: Whether or not to confirm the user input
pub fn get_binary_input<T: TwoOptions<Output = T>>(
    term: &Term,
    msg: &str,
    keys: [char; 2],
    confirm_input: bool,
) -> T {
    cls(term);

    println!("{msg}");

    loop {
        match get_char(term) {
            'q' => exit(0),
            k if k == keys[0] => {
                if !confirm_input || confirm(term) {
                    return T::option_one();
                }
            }
            k if k == keys[1] => {
                if !confirm_input || confirm(term) {
                    return T::option_two();
                }
            }
            _ => (),
        }
    }
}
