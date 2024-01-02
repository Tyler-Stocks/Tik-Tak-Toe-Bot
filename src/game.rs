pub mod stages {
    use std::{
        collections::hash_map::RandomState,
        hash::{BuildHasher, Hasher},
        thread::sleep,
        time::Duration,
        process::exit
    };

    use console::Term;
    use console::Key::{Char, Enter};

    use crate::util::{Side, StartPlayer, TwoOptions};
    use crate::io::{wait_for_enter, get_binary_input};

    fn random_seed() -> u64 {
        RandomState::new().build_hasher().finish()
    }

    pub fn start(stdout: &Term) {
        loop {
            match stdout.read_key() {
                Ok(k) => {
                    match k {
                        Enter => break,
                        Char(c) => {
                            if c == 'q' { exit(0) }
                        }
                        _ => ()
                    }
                }
                Err(_) => {
                    println!("Failed to read input. The program will terminate in 3 seconds.");
                    sleep(Duration::from_secs(3));
                    exit(1)
                }
            }
        }
    }

    pub fn get_start_configuration(stdout: &Term) -> (Side, StartPlayer) {
        let is_side_random_msg: &str   = "Randomize starting side? (Y/N)";
        let is_start_random_msg: &str  = "Randomize starting player? (Y/N)";
        let query_side_msg: &str       = "Which side would you like to be (X/O)?";
        let query_start_player: &str   = "Would you like to start? (Y/N)";

        let random_side_msgs: [&str; 2] = [
            "You are X! Press enter to continue.",
            "You are O! Press enter to continue"
        ];

        let random_start_player_msgs: [&str; 2] = [
            "The computer starts! Uh oh. Press enter to continue",
            "You start! Now you many have a chance"
        ];

        let mut side: Side;
        let mut start_player: StartPlayer;

        loop {
            side = match get_binary_input(stdout, is_side_random_msg, ['y', 'n'], false) {
                true  => calculate_random(stdout, random_side_msgs),
                false => get_binary_input(stdout, query_side_msg, ['x', 'o'], false)
            };

            start_player = match get_binary_input::<bool>(stdout, is_start_random_msg, ['y', 'n'], false) {
                true  => calculate_random(stdout, random_start_player_msgs),
                false => get_binary_input(stdout, query_start_player, ['y', 'n'], false)
            };

            let confirm: String = format!("Are you happy with the start configuration (Y/N)? \nSide: {:?}, Start Player: {:?}", side, start_player);

            match get_binary_input::<bool>(&stdout, confirm.as_str(), ['y', 'n'], true) {
                true  => break,
                false => continue
            }
        }

        (side, start_player)
    }

    fn calculate_random<T: TwoOptions<Output=T> + PartialEq>(stdout: &Term, msg: [&str; 2]) -> T {
        let output: T = match random_seed() % 2 == 0 {
            true  => T::option_one(),
            false => T::option_two()
        };

        stdout.clear_screen().unwrap();

        println!("Calculating... ");
        sleep(Duration::from_secs_f64(1.5));

        if output == T::option_one() {
            println!("{}", msg[0])
        } else {
            println!("{}", msg[1])
        }

        output
    }

    fn calculate_random_side(stdout: &Term) -> Side {
        let output: Side = match random_seed() % 2 == 0 {
            true  => Side::X,
            false => Side::O
        };

        println!("Calculating random side...");
        sleep(Duration::from_secs_f64(1.5));

        stdout.clear_screen().unwrap();

        match output {
            Side::O => println!("You are O!"),
            Side::X => println!("You are X!")
        };

        wait_for_enter(stdout);

        output
    }

    fn calculate_random_start_player(stdout: &Term) -> StartPlayer {
        let output: StartPlayer = match random_seed() % 2 == 0 {
            true  => StartPlayer::You,
            false => StartPlayer::Cpu
        };

        println!("Calculating start player...");
        sleep(Duration::from_secs_f64(1.5));

        stdout.clear_screen().unwrap();

        match output {
            StartPlayer::Cpu => println!("CPU starts! Uh oh."),
            StartPlayer::You => println!("You start! You may have a chance.")
        };

        wait_for_enter(stdout);

        stdout.clear_screen().unwrap();

        output
    }
}