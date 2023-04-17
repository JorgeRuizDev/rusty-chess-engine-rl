use crate::{moves::Direction, notation::FenError};

use crate::errors::OutOfBoundsError;
use crate::piece::Piece;
use std::cmp;

use super::{Coord, HasCoordinates};

const ROWS: u32 = 8;
const COLS: u32 = 8;

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

    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        todo!();
    }
}
