use crate::{
    board::{Board, Coord},
    check::is_check,
    piece::Piece,
};

use super::{parse_direction, Line, Move};
const MAX_RANGE: u32 = 2; // In a FIDE castle, the king can move 2 cells
pub struct Castle<M: Move> {
    movement: M,
}

impl Castle<Line> {
    pub fn new(max_range: Option<u32>) -> Self {
        let max_range = max_range.unwrap_or(MAX_RANGE);
        let movement = Line::new(Some(max_range));

        Castle { movement }
    }

    fn is_line_clear(&self, king: &Coord, rook: &Coord, board: &Board) -> bool {
        let direction = match parse_direction(king, rook) {
            Ok(direction) => direction,
            Err(_) => return false,
        };

        let step = direction.get_step();

        let mut inter_cell = *king + step; // intermediate cell that the king will traverse

        loop {
            if inter_cell == *rook {
                return true;
            }

            match board.get_piece(&inter_cell) {
                Ok(Some(_)) => return false,
                Ok(None) => (),
                Err(_) => return true, //
            }

            inter_cell = inter_cell + step;
        }
    }

    /// Check that the king can safely traverse the path (there are not pieces in the way)
    fn can_safely_traverse(
        &self,
        king: &Coord,
        new_king: &Coord,
        rook: &Coord,
        board: &mut Board,
    ) -> bool {
        let direction = match parse_direction(new_king, king) {
            Ok(direction) => direction,
            Err(_) => return false,
        };

        // check that the king can move
        // It uses by default the line move as an abstraction... But it traverses
        // those cells twice, one for the movement and the other for the check.
        // Can be optimized? Maybe, but it will be more coupled and nastier.
        if !self.is_line_clear(king, rook, board) {
            return false;
        }

        let ally_color = match board.get_piece(king) {
            Ok(Some(piece)) => piece.color,
            _ => return false,
        };

        // if can move, check that the king is not under check in any of the
        // inter cells
        let step = direction.get_step();

        let mut inter_cell = *king; // intermediate cell that the king will traverse

        for _ in 0..MAX_RANGE + 1 {
            // + 1 because the king cannot move under check in the initial cell

            let under_check = board.temporal_move(king, &inter_cell, |board| {
                is_check(&inter_cell, board, false)
            });

            if under_check {
                return false;
            }

            inter_cell = inter_cell.add(&step);
        }
        true
    }
}

impl Move for Castle<Line> {
    fn is_move_valid(&self, from: Coord, to: Coord, board: &Board) -> bool {
        let from_piece = match board.get_piece(&from) {
            Ok(Some(piece)) => piece,
            _ => return false,
        };

        // Check that you can move to that cell (Because the king or rook have not moved)
        let rights = match board.info.castling.get(&from_piece.color) {
            Some(castling) => castling,
            None => return false,
        };

        for right in rights {
            if right.new_king != to {
                continue;
            }
            return self.can_safely_traverse(&from, &to, &right.rook, &mut board.clone());
        }
        false // move not in rights
    }

    fn allowed_moves(&self, from: Coord, board: &Board) -> Vec<Coord> {
        let from_piece = match board.get_piece(&from) {
            Ok(Some(piece)) => piece,
            _ => return Vec::new(),
        };

        let rights = match board.info.castling.get(&from_piece.color) {
            Some(castling) => castling,
            None => return Vec::new(),
        };

        let mut allowed_moves = vec![];
        let mut board = board.clone(); // FIXME: board.clone

        for right in rights {
            if self.can_safely_traverse(&from, &right.new_king, &right.rook, &mut board) {
                allowed_moves.push(right.new_king);
            }
        }

        allowed_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn test_is_valid_castle() {
        let board = Board::from_fen("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();

        let castle = Castle::new(None);

        let from = Coord { row: 7, col: 4 };

        // Long Castle
        let to = Coord { row: 7, col: 2 };
        assert!(castle.is_move_valid(from, to, &board));

        // Short Castle
        let to = Coord { row: 7, col: 6 };
        assert!(castle.is_move_valid(from, to, &board));
    }

    #[test]
    fn test_invalid_castle() {
        let board = Board::from_fen("4k3/8/8/8/8/8/8/Rn2K1NR w KQ - 0 1").unwrap();
        let castle = Castle::new(None);

        let from = Coord { row: 7, col: 4 };

        // Long Castle
        let to = Coord { row: 7, col: 2 };
        assert!(!castle.is_move_valid(from, to, &board));

        // Short Castle
        let to = Coord { row: 7, col: 6 };
        assert!(!castle.is_move_valid(from, to, &board));
    }

    #[test]
    fn test_invalid_fen_castle() {
        let board = Board::from_fen("4k3/8/8/8/8/8/8/Rn2K1NR w - - 0 1").unwrap();

        let castle = Castle::new(None);

        let from = Coord { row: 7, col: 4 };

        // Long Castle
        let to = Coord { row: 7, col: 2 };
        assert!(!castle.is_move_valid(from, to, &board));

        // Short Castle
        let to = Coord { row: 7, col: 6 };
        assert!(!castle.is_move_valid(from, to, &board));
    }

    #[test]
    fn test_castle_with_check() {
        let board = Board::from_fen("1k6/8/8/8/2pqp3/4q3/8/R3K2R w KQ - 0 1").unwrap();

        let castle = Castle::new(None);

        let from = Coord { row: 7, col: 4 };

        // Long Castle
        let to = Coord { row: 7, col: 2 };
        assert!(!castle.is_move_valid(from, to, &board));

        // Short Castle
        let to = Coord { row: 7, col: 6 };
        assert!(!castle.is_move_valid(from, to, &board));
    }

    #[test]
    fn test_generate_valid() {
        let board = Board::from_fen("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
        let castle = Castle::new(None);
        let from = Coord { row: 7, col: 4 };

        assert_eq!(castle.allowed_moves(from, &board).len(), 2);
    }

    #[test]
    fn test_generate_invalid() {
        // pieces between king and rook
        let board = Board::from_fen("4k3/8/8/8/8/8/8/Rn2K1NR w KQ - 0 1").unwrap();
        let castle = Castle::new(None);
        let from = Coord { row: 7, col: 4 };

        assert_eq!(castle.allowed_moves(from, &board).len(), 0);

        // Just one right queen side:
        let board = Board::from_fen("4k3/8/8/8/8/8/8/R3K2R w Q - 0 1").unwrap();
        let castle = Castle::new(None);
        let from = Coord { row: 7, col: 4 };

        assert_eq!(castle.allowed_moves(from, &board).len(), 1);

        // king under check

        let board = Board::from_fen("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
        let castle = Castle::new(None);
        let from = Coord { row: 7, col: 4 };

        assert_eq!(castle.allowed_moves(from, &board).len(), 2);
    }
}
