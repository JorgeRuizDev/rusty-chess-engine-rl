use crate::moves::Direction;
use crate::piece::Color;

use super::errors::OutOfBoundsError;
use super::piece::Piece;
use std::cmp;
use std::collections::HashMap;
use std::ops::Add;

const ROWS: u32 = 8;
const COLS: u32 = 8;

pub trait HasCoordinates {
    fn get_coordinates(&self) -> Coord;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}

impl HasCoordinates for Coord {
    fn get_coordinates(&self) -> Coord {
        *self
    }
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Self::Output {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

////////////////////////////////////////////////
// BOARD
////////////////////////////////////////////////
#[derive(Debug)]
pub struct Board {
    board: Vec<Vec<Option<Piece>>>,
    n_rows: u32,
    n_cols: u32,
}

impl Board {
    pub fn can_move(&self, from: &Coord, to: &Coord) -> bool {
        let piece = match self.get_piece(from) {
            Ok(Some(piece)) => piece,
            _ => return false,
        };

        for move_ in piece.moves.iter() {
            if move_.is_move_valid(*from, *to, self) {
                return true;
            }
        }

        return false;
    }

    pub fn new(n_rows: Option<u32>, n_cols: Option<u32>) -> Self {
        // Get the default values
        let n_rows = n_rows.unwrap_or(ROWS);
        let n_cols = n_cols.unwrap_or(COLS);

        let mut board: Vec<Vec<Option<Piece>>> = Vec::new();
        // Fill the matrix with cells
        for _ in 0..n_rows {
            let row = (0..n_cols).map(|_| None).collect();
            board.push(row);
        }

        Self {
            board,
            n_rows,
            n_cols,
        }
    }

    pub fn in_bounds<T: HasCoordinates>(&self, coords: &T) -> bool {
        let Coord { row, col } = coords.get_coordinates();
        row >= 0 && row < self.n_rows as i32 && col >= 0 && col < self.n_cols as i32
    }

    pub fn set_piece(&mut self, piece: Piece) {
        let Coord { row, col } = piece.coord;
        self.board[row as usize][col as usize] = Some(piece);
    }

    pub fn remove_piece<T: HasCoordinates>(&mut self, coords: &T) {
        let Coord { row, col } = coords.get_coordinates();
        self.board[row as usize][col as usize] = None;
    }

    pub fn get_piece_mut<T: HasCoordinates>(
        &mut self,
        coords: &T,
    ) -> Result<&mut Option<Piece>, OutOfBoundsError> {
        let Coord { row, col } = coords.get_coordinates();

        if !self.in_bounds(coords) {
            return Err(OutOfBoundsError);
        }

        Ok(&mut self.board[row as usize][col as usize])
    }

    pub fn get_piece<T: HasCoordinates>(
        &self,
        coords: &T,
    ) -> Result<Option<&Piece>, OutOfBoundsError> {
        let Coord { row, col } = coords.get_coordinates();

        if !self.in_bounds(coords) {
            return Err(OutOfBoundsError);
        }

        Ok(self.board[row as usize][col as usize].as_ref())
    }

    pub fn max_cells_direction(&self, direction: &Direction) -> u32 {
        match direction {
            Direction::North | Direction::South => self.n_rows,
            Direction::East | Direction::West => self.n_cols,
            _ => cmp::max(self.n_rows, self.n_cols),
        }
    }
}

////////////////////////////////////////////////
/// BOARD INFO
////////////////////////////////////////////////

#[derive(Debug)]
pub struct BoardInfo {
    /// Current Turn Color
    pub turn: Color,

    /// Castling rights, where the king can move
    pub castling: HashMap<Color, Vec<Coord>>,

    /// En passant target square
    pub en_passant: Option<Coord>,

    /// Halfmove clock - number of halfmoves since the last capture or pawn advance
    pub halfmove_clock: i32,

    /// Fullmove number - the number of the full move. It starts at 1, and is incremented after Black's move.
    pub fullmove_number: i32,
}

impl BoardInfo {
    pub fn new_default() -> Self {
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

#[derive(Debug)]
pub struct FenError;

impl Board {
    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        todo!();
    }
}
