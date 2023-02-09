use std::fmt;

use super::square::Square;
use super::*;

pub struct Move(pub Vec<SubMove>);

pub enum MoveType {
    Withdraw,
    Attach,
    Quiet,
}

pub struct SubMove {
    pub move_type: MoveType,
    pub src: Square,
    pub dst: Square,
}

impl fmt::Display for SubMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "broke")
    }
}
