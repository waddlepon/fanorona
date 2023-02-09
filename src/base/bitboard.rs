use super::bit_twiddles::*;
use super::square::Square;
use super::*;

use std::fmt;
use std::ops::*;

#[derive(Copy, Clone, Default, Hash, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct BitBoard(pub u64);

impl_bit_ops!(BitBoard, u64);

impl BitBoard {
    pub fn to_square(self) -> Square {
        Square(bit_scan_forward(self.0))
    }

    pub fn bit_scan_forward(self) -> Square {
        Square(self.bit_scan_forward_u8())
    }

    pub fn bit_scan_forward_u8(self) -> u8 {
        bit_scan_forward(self.0)
    }

}
