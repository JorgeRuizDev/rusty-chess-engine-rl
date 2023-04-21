use std::collections::HashSet;

use crate::board::{Board, Coord};

use super::{parse_direction, Direction, Move};

pub struct Jump {
    pub first: u32,
    pub second: u32,
}

static FIRST_MOVE_MASK: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

impl Jump {
    pub fn default() -> Self {
        Self {
            first: 2,
            second: 1,
        }
    }

    fn is_jump_in_range(&self, from: &Coord, to: &Coord) -> bool {
        let n_rows = (to.row - from.row).abs() as u32;
        let n_cols = (to.col - from.col).abs() as u32;

        (n_rows == self.first && n_cols == self.second)
            || (n_rows == self.second && n_cols == self.first)
    }
}

impl Move for Jump {
    fn is_move_valid(&self, from: Coord, to: Coord, board: &Board) -> bool {
        let from_piece = match board.get_piece(&from) {
            Ok(Some(piece)) => piece,
            _ => return false,
        };

        if !self.is_jump_in_range(&from, &to) {
            return false;
        }

        let to_piece = match board.get_piece(&to) {
            Ok(Some(piece)) => piece,
            Ok(None) => return true,
            _ => return false,
        };
        from_piece.color != to_piece.color
    }

    fn allowed_moves(&self, from: Coord, board: &Board) -> Vec<Coord> {
        let from_piece = match board.get_piece(&from) {
            Ok(Some(piece)) => piece,
            _ => return vec![],
        };

        let mut to_coords = HashSet::new();

        for mask in FIRST_MOVE_MASK.iter() {
            // For each long* step, go N, S, E, W

            let masks_2 = if mask.0 == 0 {
                [(1, 0), (-1, 0)]
            } else {
                [(0, 1), (0, -1)]
            };

            for mask_2 in masks_2 {
                // After a long step, go Up or Down if we went E or W
                // or go Left or Right if we went N or S
                // That's the mask 2
                // i.e, if we went E, we go Up or Down

                let to = Coord {
                    row: from.row + mask.0 * self.first as i32 + mask_2.0 * self.second as i32,
                    col: from.col + mask.1 * self.first as i32 + mask_2.1 * self.second as i32,
                };

                match board.get_piece(&to) {
                    Ok(Some(piece)) => {
                        // If capturable piece
                        if piece.color != from_piece.color {
                            to_coords.insert(to);
                        }
                    }
                    Ok(None) => {
                        to_coords.insert(to);
                    }
                    Err(_) => {} // Out of bounds
                }
            }
        }

        return to_coords.into_iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::piece::{Color, Piece};

    use super::*;

    fn prepare() -> (Board, Coord, Coord) {
        let mut board = Board::new(None, None);
        let from = Coord { row: 0, col: 0 };
        let to = Coord { row: 2, col: 1 };

        let knight = Piece::new_knight(Color::White, from);
        board.set_piece(knight);
        (board, from, to)
    }

    #[test]
    fn test_is_jump_in_range() {
        let jump = Jump::default();
        let from = Coord { row: 0, col: 0 };
        let to = Coord { row: 2, col: 1 };

        assert!(jump.is_jump_in_range(&from, &to));

        let to = Coord { row: 1, col: 2 };
        assert!(jump.is_jump_in_range(&from, &to));
    }

    #[test]
    fn test_is_move_valid() {
        let jump = Jump::default();
        let from = Coord { row: 0, col: 0 };
        let to = Coord { row: 2, col: 1 };

        let board = Board::default();
        assert!(jump.is_move_valid(from, to, &board));
    }

    #[test]
    fn test_jump_to_enemy() {
        let (mut board, from, to) = prepare();

        let jump = Jump::default();

        assert!(jump.is_move_valid(from, to, &board));

        let black_piece = Piece::new_bishop(Color::Black, to);
        board.set_piece(black_piece);

        assert!(jump.is_move_valid(from, to, &board));
    }

    #[test]
    fn test_jump_to_ally() {
        let (mut board, from, to) = prepare();

        let piece = board.get_piece(&from).unwrap().unwrap();

        let jump = Jump::default();

        assert!(jump.is_move_valid(from, to, &board));

        let black_piece = Piece::new_bishop(Color::White, to);
        board.set_piece(black_piece);

        assert!(!jump.is_move_valid(from, to, &board));
    }

    #[test]
    fn invalid_jumps() {
        let (board, from, to) = prepare();

        let jump = Jump::default();

        let to = Coord { row: 1, col: 1 };

        assert!(!jump.is_move_valid(from, to, &board));

        let to = Coord { row: 1, col: 0 };

        assert!(!jump.is_move_valid(from, to, &board));

        let to = Coord { row: 0, col: 1 };

        assert!(!jump.is_move_valid(from, to, &board));

        let to = Coord { row: 3, col: 1 };

        assert!(!jump.is_move_valid(from, to, &board));
    }

    #[test]
    fn test_allowed_moves() {
        let (board, from, to) = prepare();

        let jump = Jump::default();

        let moves = jump.allowed_moves(from, &board);

        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_possible_jumps() {
        let (mut board, from, to) = prepare();

        let jump = Jump::default();

        let moves = jump.allowed_moves(from, &board);

        assert_eq!(moves.len(), 2);

        let to = Coord { row: 2, col: 1 };
        assert!(moves.contains(&to));

        let to = Coord { row: 1, col: 2 };
        assert!(moves.contains(&to));
    }

    #[test]
    fn test_init_white_knight() {
        let mut board = Board::default();

        let from = Coord { row: 0, col: 1 };

        let jump = Jump::default();
        let moves = jump.allowed_moves(from, &board);
        assert_eq!(moves.len(), 2);

        let to = Coord { row: 2, col: 2 };
        assert!(moves.contains(&to));

        let to = Coord { row: 2, col: 0 };
        assert!(moves.contains(&to));
    }

    #[test]
    fn test_in_black_zone() {
        let mut board = Board::default();

        let from = Coord { row: 2, col: 4 }; // In front of the black pawn row

        let knight = Piece::new_knight(Color::White, from.clone());
        board.set_piece(knight);

        let jump = Jump::default();
        let moves = jump.allowed_moves(from, &board);

        assert_eq!(moves.len(), 8);
    }
}
