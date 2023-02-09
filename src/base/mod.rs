#[macro_use]
mod macros;

pub mod bit_twiddles;
pub mod bitboard;
pub mod square;
pub mod piece_move;

use std::fmt;
use std::mem;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Player {
    White = 0,
    Black = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    None = 0b0000,
    WhitePiece = 0b0001,
    BlackPiece = 0b1001,
}

impl Piece {
    #[inline(always)]
    pub fn player(self) -> Option<Player> {
        if self as u8 & 0b0111 == 0 {
            None
        } else {
            Some(unsafe { mem::transmute((self as u8 >> 3) & 0b1) })
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[repr(u8)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Rank {
    R1 = 0,
    R2 = 1,
    R3 = 2,
    R4 = 3,
    R5 = 4,
}
