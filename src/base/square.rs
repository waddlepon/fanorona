//!
//! Maps u8 to these spots on the board
//!
//! 5 | 36 37 38 39 40 41 42 43 44 
//! 4 | 27 28 29 30 31 32 33 34 35 
//! 3 | 18 19 20 21 22 23 24 25 26
//! 2 | 9  10 11 12 13 14 15 16 17
//! 1 | 0  1  2  3  4  5  6  7  8
//!   ----------------------------
//!     a  b  c  d  e  f  g  h  i
//!

use super::bitboard::BitBoard;
use super::*;

use std::fmt;
use std::mem::transmute;
use std::ops::*;

#[derive(Copy, Clone, Default, Hash, PartialEq, PartialOrd, Eq, Debug)]
#[repr(transparent)]
pub struct Square(pub u8);

impl_bit_ops!(Square, u8);

impl Square {
    #[inline(always)]
    pub const fn is_okay(self) -> bool {
        self.0 < 45
    }

    /// Strong square has diagonals
    #[inline(always)]
    pub const fn is_strong(self) -> bool {
       self.0 % 2 == 0
    }

    pub fn to_bb(self) -> BitBoard {
        assert!(self.is_okay());
        BitBoard(1) << self.0.into()
    }

    pub fn rank(self) -> Rank {
        unsafe { transmute::<u8, Rank>(self.0 / 9) }
    }

    pub fn file(self) -> File {
        unsafe { transmute::<u8, File>(self.0 % 9) }
    }

    pub fn make(file: File, rank: Rank) -> Square {
        Square(((rank as u8) * 9 + (file as u8)) as u8)
    }
}

pub static SQUARE_DISPLAY: [&str; 45] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "i1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "i2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "i3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "i4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "i5",
];

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", SQUARE_DISPLAY[self.0 as usize])
    }
}
