use console::Term;

pub trait TwoOptions {
    type Output;

    fn option_one() -> Self::Output;
    fn option_two() -> Self::Output;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Side {
    X,
    O
}

impl TwoOptions for Side {
    type Output = Side;

    /// Returns Side::X
    fn option_one() -> Self::Output {
        Side::X
    }

    /// Returns Side::O
    fn option_two() -> Self::Output {
        Side::O
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StartPlayer {
    You,
    Cpu
}

impl TwoOptions for StartPlayer {
    type Output = StartPlayer;

    /// Returns Self::You
    fn option_one() -> Self::Output {
        Self::You
    }

    /// Returns Self::Cpu
    fn option_two() -> Self::Output {
        Self::Cpu
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

