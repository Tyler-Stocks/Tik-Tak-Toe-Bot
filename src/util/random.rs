use console::Term;

use crate::{io::cls, util::traits::TwoOptions};

use std::{
    collections::hash_map::RandomState, hash::BuildHasher, hash::Hasher, thread::sleep,
    time::Duration,
};

pub fn calculate_random<T: TwoOptions<Output = T> + PartialEq>(term: &Term, msg: [&str; 2]) -> T {
    let random_seed: u64 = RandomState::new().build_hasher().finish();

    let output: T = match random_seed % 2 == 0 {
        true => T::option_one(),
        false => T::option_two(),
    };

    cls(term);

    println!("Calculating... ");
    sleep(Duration::from_secs_f64(1.5));

    if output == T::option_one() {
        println!("{}", msg[0])
    } else {
        println!("{}", msg[1])
    }

    output
}
