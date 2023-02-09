use crate::base::square::Square;
use crate::base::Player;
use crate::base::*;

use self::board_state::BoardState;

use std::sync::Arc;

pub mod board_state;

pub struct Board {
    turn: Player,
    moves_made: u16,
    active_piece: Option<Square>,
    travelled_locations: Vec<Square>,
    state: Arc<BoardState>,
}
