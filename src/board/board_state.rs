use super::Board;

use crate::base::bitboard::BitBoard;
use crate::base::piece_move::{Move, SubMove};
use crate::base::square::Square;
use crate::base::Player;
use crate::base::Piece;
use crate::base::*;

use std::sync::Arc;

pub struct BoardState {
    pub prev_move: SubMove,
    pub prev: Option<Arc<BoardState>>,
    pub active_piece: Option<Square>,
    pub travelled_locations: Vec<Square>,
    pub captured_pieces: Vec<(Square, Piece)>,
}
