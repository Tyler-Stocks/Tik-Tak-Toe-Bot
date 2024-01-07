use crate::util::core::{
    Side,
    Side::{O, X},
    Turn,
    Turn::{Cpu, You},
};

pub trait TwoOptions {
    type Output;

    fn option_one() -> Self::Output;
    fn option_two() -> Self::Output;
}

impl TwoOptions for Turn {
    type Output = Turn;

    fn option_one() -> Self::Output {
        You
    }

    fn option_two() -> Self::Output {
        Cpu
    }
}

impl TwoOptions for Side {
    type Output = Side;

    fn option_one() -> Self::Output {
        X
    }

    fn option_two() -> Self::Output {
        O
    }
}

impl TwoOptions for bool {
    type Output = bool;

    fn option_one() -> Self::Output {
        true
    }

    fn option_two() -> Self::Output {
        false
    }
}
