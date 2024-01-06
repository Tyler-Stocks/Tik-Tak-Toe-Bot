pub mod stages {
    use std::hash::Hasher;
    use std::{
        collections::hash_map::RandomState, hash::BuildHasher, process::exit, thread::sleep,
        time::Duration,
    };

    use console::Key::{Char, Enter};
    use console::Term;

    use crate::board::Board;
    use crate::io::{clear_screen, get_binary_input, get_key, get_num, wait_for_enter};
    use crate::util::core::{Side, Turn, TwoOptions};

    fn do_player_turn(term: &Term, board: &mut Board, side: Side) {
        let msg: &str =
            "Choose a tile to place your piece (1 - 9). You cannot place a piece on an occupied tile.";

        let mut tile_move: u8;

        loop {
            tile_move = get_num(term, msg);

            if board.is_empty(tile_move) {
                board.do_turn(tile_move, side);
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

        // Game Loop
        loop {
            loop {
                side = match get_binary_input(term, is_side_random_msg, ['y', 'n'], false) {
                    true => calculate_random(term, random_side_msgs),
                    false => get_binary_input(term, query_side_msg, ['x', 'o'], false),
                };

                start_player = match get_binary_input(term, is_start_random_msg, ['y', 'n'], false)
                {
                    true => calculate_random(term, random_start_player_msgs),
                    false => get_binary_input(term, query_start_player, ['y', 'n'], false),
                };

                let confirm: String = format!("Are you happy with the start configuration (Y/N)? \nSide: {:?}, Start Player: {:?}", side, start_player);

                match get_binary_input(term, confirm.as_str(), ['y', 'n'], true) {
                    true => break,
                    false => continue,
                }
            }

            // Turn loop
            loop {
                let mut turn_counter: u8 = 1;

                match start_player {
                    Turn::You => {
                        if turn_counter % 2 != 0 {
                            todo!()
                        } else {
                            todo!()
                        }
                    }
                    Turn::Cpu => {
                        if turn_counter % 2 != 0 {
                            todo!()
                        } else {
                            todo!()
                        }
                    }
                }
            }
        }
    }

    fn calculate_random<T: TwoOptions<Output = T> + PartialEq>(term: &Term, msg: [&str; 2]) -> T {
        let random_seed: u64 = RandomState::new().build_hasher().finish();

        let output: T = match random_seed % 2 == 0 {
            true => T::option_one(),
            false => T::option_two(),
        };

        clear_screen(term);

        println!("Calculating... ");
        sleep(Duration::from_secs_f64(1.5));

        if output == T::option_one() {
            println!("{}", msg[0])
        } else {
            println!("{}", msg[1])
        }

        output
    }

    pub fn game_loop(term: &Term) -> ! {
        let mut start_config: (Side, Turn);

        start(term);

        loop {
            start_config = get_start_configuration(term);

            wait_for_enter(term);

            println!(
                "Start Side: {:?}, Start Player: {:?}",
                start_config.0, start_config.1
            );
        }
    }
}
