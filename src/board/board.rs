use crate::{moves::Direction, notation::FenError};

use crate::errors::OutOfBoundsError;
use crate::notation::fen;
use crate::notation::fen::parse as parse_fen;
use crate::piece::{Color, Piece};
use std::cmp;
use std::fmt::format;

use super::{BoardInfo, Coord, HasCoordinates};

const ROWS: u32 = 8;
const COLS: u32 = 8;

////////////////////////////////////////////////
// BOARD
////////////////////////////////////////////////

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

    pub fn default() -> Self {
        Self::from_fen(fen::INITIAL_BOARD).unwrap()
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
