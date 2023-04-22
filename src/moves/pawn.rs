use super::{Direction, Move};
use crate::board::{Board, Coord};
use crate::moves::parse_direction;
use crate::piece::{Color, Piece};

pub struct PawnMove {}

impl PawnMove {
    pub fn new() -> PawnMove {
        PawnMove {}
    }

    fn check_one_step(&self, to: &Coord, board: &Board) -> bool {
        match board.get_piece(to) {
            Ok(Some(_)) => false,
            Ok(None) => true,
            _ => unreachable!(), // outside of board
        }
    }

    fn check_two_steps(&self, from_piece: &Piece, step: &Coord, board: &Board) -> bool {
        if !board.is_pawn_row(from_piece.coord.row, from_piece.color) {
            return false;
        }

        for _ in 0..2 {
            let next_coord = from_piece.coord.clone() + step.clone();
            if !self.check_one_step(&next_coord, &board) {
                return false;
            }
        }

        true
    }

    fn check_en_passant(&self, to: &Coord, board: &Board) -> bool {
        match board.info.en_passant {
            Some(coord) => &coord == to,
            None => false,
        }
    }

    fn check_capture(&self, from_piece: &Piece, to: &Coord, board: &Board) -> bool {
        let to_piece = match board.get_piece(to) {
            Ok(Some(piece)) => piece,
            Ok(None) => return self.check_en_passant(&to, &board), // Empty cell
            _ => return false,
        };
        to_piece.color != from_piece.color
    }
}

impl Move for PawnMove {
    fn is_move_valid(&self, from: Coord, to: Coord, board: &Board) -> bool {
        let from_piece = match board.get_piece(&from) {
            Ok(Some(piece)) => piece,
            _ => return false,
        };

        let direction = match parse_direction(&from, &to) {
            Ok(dir) => dir,
            Err(_) => return false, // Invalid direction
        };

        let direction = match from_piece.color {
            Color::Black => match direction {
                Direction::South | Direction::SouthEast | Direction::SouthWest => direction,
                _ => return false,
            },
            Color::White => match direction {
                Direction::North | Direction::NorthEast | Direction::NorthWest => direction,
                _ => return false,
            },
        };

        let row_dis = (from.row as i32 - to.row as i32).abs();
        let col_dis = (from.col as i32 - to.col as i32).abs();

        if col_dis == 1 && row_dis == 1 {
            return self.check_capture(&from_piece, &to, &board);
        }

        if col_dis != 0 {
            return false; // Can't capture
        }

        if row_dis == 1 {
            return self.check_one_step(&to, &board);
        } else if row_dis == 2 {
            return self.check_two_steps(&from_piece, &direction.get_step(), &board);
        }
        false
    }

    fn allowed_moves(&self, from: Coord, board: &Board) -> Vec<Coord> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, Coord},
        moves::Move,
    };

    use super::PawnMove;

    #[test]
    pub fn check_opening() {
        let board = Board::default();
        let pawn = PawnMove::new();

        assert!(pawn.is_move_valid(Coord { row: 1, col: 0 }, Coord { row: 3, col: 0 }, &board));
        assert!(pawn.is_move_valid(Coord { row: 6, col: 0 }, Coord { row: 5, col: 0 }, &board));

        assert!(!pawn.is_move_valid(Coord { row: 1, col: 0 }, Coord { row: 4, col: 0 }, &board));
        assert!(!pawn.is_move_valid(Coord { row: 6, col: 0 }, Coord { row: 4, col: 1 }, &board));

        assert!(!pawn.is_move_valid(Coord { row: 1, col: 0 }, Coord { row: 2, col: 1 }, &board));
    }

    #[test]
    pub fn test_en_passant() {
        let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/8/6Pp/p7/PPPPPP1P/RNBQKBNR b KQkq g3 0 1")
            .unwrap();
        let pawn = PawnMove::new();

        assert!(pawn.is_move_valid(Coord { row: 4, col: 7 }, Coord { row: 5, col: 6 }, &board));
        assert!(pawn.is_move_valid(Coord { row: 4, col: 7 }, Coord { row: 5, col: 7 }, &board))
    }

    #[test]
    pub fn test_capture() {
        let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/8/6Pp/p7/PPPPPP1P/RNBQKBNR b KQkq g3 0 1")
            .unwrap();
        let pawn = PawnMove::new();

        let black = Coord { row: 5, col: 0 };
        let white = Coord { row: 6, col: 1 };

        assert!(pawn.is_move_valid(black, white, &board));
        assert!(pawn.is_move_valid(white, black, &board));

        // Extra black moves
        assert!(!pawn.is_move_valid(black, black, &board));
        assert!(!pawn.is_move_valid(black, Coord { row: 6, col: 0 }, &board));
    }

    #[test]
    pub fn test_pawn_cant_backward() {
        let mut board = Board::default();
        let pawn = PawnMove::new();

        let from = Coord { row: 1, col: 0 };
        let to = Coord { row: 2, col: 0 };

        assert!(pawn.is_move_valid(from, to, &board));

        pawn.move_piece(from.clone(), to.clone(), &mut board);

        println!("{}", board);

        assert!(!pawn.is_move_valid(to, from, &board)); // Go back
    }
}
