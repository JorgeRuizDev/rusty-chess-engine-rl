use crate::piece::Color;
use std::collections::HashMap;

use super::Coord;
////////////////////////////////////////////////
/// BOARD INFO
////////////////////////////////////////////////

#[derive(Debug)]
pub struct BoardInfo {
    /// Current Turn Color
    pub turn: Color,

    /// Castling rights, where the king can move
    pub castling: HashMap<Color, Vec<CastlingRights>>,

    /// En passant target square
    pub en_passant: Option<Coord>,

    /// Halfmove clock - number of halfmoves since the last capture or pawn advance
    pub halfmove_clock: i32,

    /// Fullmove number - the number of the full move. It starts at 1, and is incremented after Black's move.
    pub fullmove_number: i32,
}

impl BoardInfo {
    pub fn default() -> Self {
        Self {
            turn: Color::White,
            castling: HashMap::new(),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn next_turn(&mut self) {
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        if self.turn == Color::White {
            self.fullmove_number += 1;
        }

        self.halfmove_clock += 1;
    }

    pub fn reset_halfmove_clock(&mut self) {
        self.halfmove_clock = 0;
    }
}

#[derive(Debug, PartialEq)]
pub struct CastlingRights {
    /// Cell where the king will move to
    pub new_king: Coord,

    /// Cell where the rook is at
    pub tower: Coord,
}
