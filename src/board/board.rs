use crate::PieceType;
use crate::{moves::Direction, notation::FenError};

use super::{BoardInfo, Coord, HasCoordinates};
use crate::errors::OutOfBoundsError;
use crate::notation::fen;
use crate::notation::fen::parse as parse_fen;
use crate::piece::{Color, Piece};
use pyo3::prelude::*;
use std::cmp;

const ROWS: u32 = 8;
const COLS: u32 = 8;

////////////////////////////////////////////////
// BOARD
////////////////////////////////////////////////

#[pyclass]
#[derive(Clone)]
pub struct Board {
    board: Vec<Vec<Option<Piece>>>,
    pub info: BoardInfo,

    n_rows: u32,
    n_cols: u32,
}

impl Board {
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
            info: BoardInfo::default(),
        }
    }

    pub fn in_bounds(&self, coords: &Coord) -> bool {
        let Coord { row, col } = coords.get_coordinates();
        row >= 0 && row < self.n_rows as i32 && col >= 0 && col < self.n_cols as i32
    }

    pub fn set_piece(&mut self, piece: Piece) {
        let Coord { row, col } = piece.coord;
        self.board[row as usize][col as usize] = Some(piece);
    }

    pub fn remove_piece(&mut self, coord: &Coord) {
        self.board[coord.row as usize][coord.col as usize] = None;
    }

    pub fn move_to_coord(&mut self, from: &Coord, to: &Coord) -> Option<Piece> {
        let mut piece = self.board[from.row as usize][from.col as usize].take();

        if piece.is_some() {
            // update the piece's coordinates
            piece.as_mut().unwrap().coord = *to;
        }

        let old_piece = self.board[to.row as usize][to.col as usize].take();
        self.board[to.row as usize][to.col as usize] = piece;
        return old_piece;
    }

    pub fn get_piece_mut(
        &mut self,
        coords: &Coord,
    ) -> Result<&mut Option<Piece>, OutOfBoundsError> {
        let Coord { row, col } = coords.get_coordinates();

        if !self.in_bounds(coords) {
            return Err(OutOfBoundsError);
        }

        Ok(&mut self.board[row as usize][col as usize])
    }

    pub fn get_rows(&self) -> u32 {
        self.n_rows
    }

    pub fn get_cols(&self) -> u32 {
        self.n_cols
    }

    /// Returns the number of cells in the given direction
    pub fn max_cells_direction(&self, direction: &Direction) -> u32 {
        match direction {
            Direction::North | Direction::South => self.n_rows,
            Direction::East | Direction::West => self.n_cols,
            _ => cmp::max(self.n_rows, self.n_cols),
        }
    }

    pub fn is_promotion_row(&self, row: i32, color: Color) -> bool {
        match color {
            Color::White => row == 0,
            Color::Black => row == self.n_rows as i32 - 1,
        }
    }

    pub fn is_pawn_row(&self, row: i32, color: Color) -> bool {
        match color {
            Color::White => row == self.n_rows as i32 - 2,
            Color::Black => row == 1,
        }
    }

    pub fn get_piece(&self, coords: &Coord) -> Result<Option<&Piece>, OutOfBoundsError> {
        let Coord { row, col } = coords.get_coordinates();

        if !self.in_bounds(coords) {
            return Err(OutOfBoundsError);
        }

        Ok(self.board[row as usize][col as usize].as_ref())
    }

    pub fn get_all_pieces(&self, color: &Color) -> Vec<&Piece> {
        let mut pieces = Vec::new();
        for row in self.board.iter() {
            for cell in row.iter() {
                if let Some(piece) = cell {
                    if &piece.color == color {
                        pieces.push(piece);
                    }
                }
            }
        }
        pieces
    }

    pub fn temporal_move<F, T>(&mut self, from: &Coord, to: &Coord, mut on_board_change: F) -> T
    where
        F: FnMut(&mut Board) -> T,
    {
        let to_piece = self.move_to_coord(from, to);

        let res = on_board_change(self);

        self.move_to_coord(to, from);

        if to_piece.is_some() {
            self.board[to.row as usize][to.col as usize] = to_piece;
        }

        res
    }

    pub fn get_king(&self, color: &Color) -> &Piece {
        for row in self.board.iter() {
            for cell in row.iter() {
                if let Some(piece) = cell {
                    if piece.color == *color && piece.piece == PieceType::King {
                        return piece;
                    }
                }
            }
        }
        unreachable!("There should be a king on the board")
    }
}

#[pymethods]
impl Board {
    #[staticmethod]
    pub fn default() -> Self {
        Self::from_fen(fen::INITIAL_BOARD).unwrap()
    }

    #[staticmethod]
    #[args(fen = "fen::INITIAL_BOARD")]
    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        let (pieces, info) = parse_fen(fen)?;

        let mut board = Self::new(None, None);
        for piece in pieces {
            board.set_piece(piece);
        }
        board.info = info;

        Ok(board)
    }

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

    pub fn move_piece(&mut self, from: &Coord, to: &Coord, promote: Option<Piece>) {
        let piece = match self.get_piece(from) {
            Ok(Some(piece)) => piece,
            _ => return,
        };
        // TODO
    }

    fn __str__(&self) -> String {
        String::from(self.to_string())
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // First Row

        let mut s = String::from("");

        for (i, row) in self.board.iter().enumerate() {
            // row index
            s.push_str(format!("{} ", (i as i32 - self.n_rows as i32).abs()).as_str());

            for piece in row.iter() {
                match piece {
                    Some(piece) => s.push_str(&format!("{} ", piece)),
                    None => s.push_str("· "),
                };
            }
            s.push_str("\n");
        }

        s.push_str("  ");
        for i in 0..self.n_cols {
            s.push_str(&format!("{} ", ('a' as u8 + i as u8) as char));
        }
        s.push_str("\n");

        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("");

        for (i, row) in self.board.iter().enumerate() {
            // row index
            s.push_str(format!("{} ", i).as_str());

            for piece in row.iter() {
                match piece {
                    Some(piece) => s.push_str(&format!("{} ", piece)),
                    None => s.push_str("· "),
                };
            }
            s.push_str("\n");
        }

        s.push_str("  ");
        for i in 0..self.n_cols {
            s.push_str(&format!("{} ", i));
        }
        s.push_str("\n");

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let board = Board::default();
        println!("{}", board);
    }

    #[test]
    fn test_pawn_row() {
        let board = Board::default();
        assert!(board.is_pawn_row(1, Color::Black));
        assert!(board.is_pawn_row(6, Color::White));
    }

    #[test]
    fn test_prom_row() {
        let board = Board::default();
        assert!(board.is_promotion_row(0, Color::White));
        assert!(board.is_promotion_row(7, Color::Black));
    }
}
