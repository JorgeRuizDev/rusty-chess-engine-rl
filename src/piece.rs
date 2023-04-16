use std::{fmt, rc::Rc};

use crate::{board::Coord, moves::Move};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Black => write!(f, "B"),
            Self::White => write!(f, "W"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]

pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Bishop => write!(f, "♝"),
            Self::King => write!(f, "♚"),
            Self::Queen => write!(f, "♛"),
            Self::Rook => write!(f, "♜"),
            Self::Knight => write!(f, "♞"),
            Self::Pawn => write!(f, "♟︎"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Piece {
    pub color: Color,
    pub piece: PieceType,
    pub has_moved: bool,
    // Mutable Cell reference:
    pub coord: Coord,
    pub moves: Vec<Rc<dyn Move>>,
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.color, self.piece)
    }
}

impl Piece {
    pub fn new(color: Color, piece: PieceType, moves: Vec<Rc<dyn Move>>, coord: Coord) -> Self {
        Self {
            color,
            piece,
            has_moved: false,
            moves: moves,
            coord,
        }
    }
}

#[cfg(test)]
mod piece_tests {
    use crate::board::Coord;

    use super::*;
}
