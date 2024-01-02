#![allow(dead_code)]

use std::process::exit;
use console::{Key, Term};
use crate::util::TwoOptions;

pub fn wait_for_enter(term: &Term) {
    'input_loop: loop {
        match term.read_key() {
            Ok(key)    => if key == Key::Enter { break 'input_loop; },
            Err(err) => panic!("Failed to read key [{}]", err)
        }
    }
}

fn confirm(stdout: &Term) -> bool {
    stdout.clear_screen().unwrap();

    println!("Are you sure (Y/N)?");

    loop {
        match stdout.read_char().unwrap() {
            'q' => exit(0),
            'y' => return true,
            'n' => return false,
            _   => stdout.clear_screen().unwrap(),

        };
    }
}

pub fn get_binary_input<T: TwoOptions<Output = T>>(stdout: &Term, msg: &str, keys: [char; 2], confirm_input: bool) -> T {
    stdout.clear_screen().unwrap();

    println!("{msg}");

    let mut output: T;

    loop {
        match stdout.read_char().unwrap() {
            'q' => exit(0),
            k if k == keys[0] => {
                output = T::option_one();

                if !confirm_input { break; }

                match confirm(stdout) {
                    true  => break,
                    false => continue
                }
            },
            k if k == keys[1] => {
                output = T::option_two();

                if !confirm_input { break; }

                match confirm(stdout) {
                    true  => break,
                    false => continue
                }
            }
            _ => ()
        }
    }

    stdout.clear_screen().unwrap();

    output
}


