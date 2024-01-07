use crate::util::traits::TwoOptions;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Side {
    X,
    O,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Turn {
    You,
    Cpu,
}
