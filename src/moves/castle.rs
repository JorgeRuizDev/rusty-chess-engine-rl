use crate::{
    board::{Board, Coord},
    piece::Piece,
};

use super::{parse_direction, Line, Move};
const MAX_RANGE: u32 = 2;
pub struct Castle<M: Move> {
    movement: M,
}

impl Castle<Line> {
    pub fn new(max_range: Option<u32>) -> Self {
        let max_range = max_range.unwrap_or(MAX_RANGE);
        let movement = Line::new(Some(max_range));

        Castle { movement }
    }

    /// Check that the king can safely traverse the path (there are not pieces in the way)
    fn can_safely_traverse(&self, king: &Coord, rook: &Coord, board: &Board) -> bool {
        let direction = match parse_direction(rook, king) {
            Ok(direction) => direction,
            Err(_) => return false,
        };

        let step = direction.get_step();

        let to = rook.clone() + step;

        // is move valid requires that the target (to) cell is empty
        // or has an enemy, that's why we get the previous cell to the rook
        self.movement.is_move_valid(*king, to, board)
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

        todo!()
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

        for right in rights {
            if self.can_safely_traverse(&right.new_king, &right.tower, board) {
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
    #[ignore]
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
    #[ignore]
    fn test_invalid_castle() {
        let board = Board::from_fen("4k3/8/8/8/8/8/8/Rn2K1NR w KQ - 0 1").unwrap();
    }

    #[test]
    #[ignore]
    fn test_no_fen_castle() {
        let board = Board::from_fen("4k3/8/8/8/8/8/8/Rn2K1NR w - - 0 1").unwrap();
    }

    #[test]
    #[ignore]
    fn test_castle_with_check() {
        let board = Board::from_fen("1k6/8/8/3q4/2p1p3/8/8/R3K2R w KQ - 0 1").unwrap();
    }
}
