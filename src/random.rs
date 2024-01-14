use rand::{thread_rng, Rng};

use std::{thread::sleep, time::Duration};

use crate::{io::wait_for_enter, util::traits::TwoOptions};

use console::Term;

pub fn get_random<T: TwoOptions<Output = T>>(term: &Term, msgs: [&str; 2]) -> T {
    let random_number: usize = thread_rng().gen_range(1..2);

    println!("Calculating...");

    sleep(Duration::from_secs(2));

    match random_number {
        1 => {
            println!("{}", msgs[0]);
            wait_for_enter(term);
            return T::option_one();
        }
        2 => {
            println!("{}", msgs[0]);
            wait_for_enter(term);
            return T::option_two();
        }
        _ => panic!("Random number out of range."),
    }
}
