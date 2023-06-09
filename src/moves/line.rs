use super::{
    parse_direction,
    util::{can_traverse, legal_coords_along_direction},
    Direction, Move,
};
use crate::board::{Board, Coord};
pub struct Line {
    max_range: Option<u32>,
}

impl Line {
    pub fn new(max_range: Option<u32>) -> Line {
        Line { max_range }
    }
}

impl Move for Line {
    fn is_move_valid(&self, from: Coord, to: Coord, board: &Board) -> bool {
        let from_piece = match board.get_piece(&from) {
            Ok(Some(piece)) => piece,
            _ => return false,
        };

        let direction = match parse_direction(&from, &to) {
            Ok(direction) => direction,
            Err(_) => return false,
        };

        let step = match direction {
            Direction::North | Direction::South | Direction::East | Direction::West => {
                direction.get_step()
            }
            _ => return false,
        };

        let max_range = self
            .max_range
            .unwrap_or(board.max_cells_direction(&direction));

        return can_traverse(board, from_piece, &to, &step, max_range);
    }

    fn allowed_moves(&self, from: Coord, board: &Board) -> Vec<Coord> {
        let from_piece = match board.get_piece(&from) {
            Ok(Some(piece)) => piece,
            _ => return vec![],
        };

        let mut allowed_moves = vec![];

        for direction in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            let max_range = self
                .max_range
                .unwrap_or(board.max_cells_direction(&direction));

            allowed_moves.append(&mut legal_coords_along_direction(
                &from,
                &direction,
                &board,
                &from_piece,
                max_range,
            ));
        }
        allowed_moves
    }
}

mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::board::{Board, Coord};
    use crate::piece::{Color, Piece, PieceType};

    fn prepare() -> (Board, Coord, Rc<dyn Move>) {
        let mut board = Board::new(Some(3), Some(3));
        let from = Coord { row: 0, col: 0 };

        let line: Rc<dyn Move> = Rc::new(Line::new(None));
        let rook = Piece::new(Color::White, PieceType::Rook, vec![Rc::clone(&line)], from);

        board.set_piece(rook);

        return (board, from, Rc::clone(&line));
    }

    #[test]
    fn test_line_move() {
        // create a 3x3 board
        let (board, from, line) = prepare();
        for (to, result) in [
            (Coord { row: 0, col: 1 }, true),
            (Coord { row: 0, col: 2 }, true),
            (Coord { row: 1, col: 0 }, true),
            (Coord { row: 2, col: 0 }, true),
            (Coord { row: 1, col: 1 }, false),
            (Coord { row: 2, col: 2 }, false),
            (Coord { row: -1, col: 0 }, false),
        ] {
            println!("Testing {:?} -> {:?}", from, to);
            assert!(line.is_move_valid(from, to, &board) == result);
            assert!(board.can_move(&from, &to) == result);
        }
    }

    #[test]
    fn test_line_move_with_limits() {
        let (board, from, _) = prepare();
        let line = Line::new(Some(1));

        for (to, result) in [
            (Coord { row: 0, col: 1 }, true),
            (Coord { row: 0, col: 2 }, false),
            (Coord { row: 1, col: 0 }, true),
            (Coord { row: 2, col: 0 }, false),
            (Coord { row: 1, col: 1 }, false),
            (Coord { row: 2, col: 2 }, false),
            (Coord { row: -1, col: 0 }, false),
        ] {
            println!("Testing {:?} -> {:?}", from, to);
            assert!(line.is_move_valid(from, to, &board) == result);
        }
    }

    #[test]
    fn test_move_empty() {
        let (mut board, from, line) = prepare();
        let to = Coord { row: 0, col: 1 };

        line.move_piece(from, to, &mut board);

        assert!(board.get_piece(&from).unwrap().is_none());
        assert!(board.get_piece(&to).unwrap().is_some());
    }

    #[test]
    fn test_move_piece() {
        let (mut board, from, line) = prepare();
        let to = Coord { row: 0, col: 1 };

        let piece = Piece::new(Color::White, PieceType::Pawn, vec![], to);
        board.set_piece(piece);

        line.move_piece(from, to, &mut board);

        assert!(board.get_piece(&from).unwrap().is_none());
        assert!(board.get_piece(&to).unwrap().unwrap().piece == PieceType::Rook);
    }

    #[test]
    fn test_move_piece_capture() {
        let (mut board, from, line) = prepare();
        let to = Coord { row: 0, col: 1 };

        let piece = Piece::new(Color::Black, PieceType::Pawn, vec![], to);
        board.set_piece(piece);

        line.move_piece(from, to, &mut board);

        assert!(board.get_piece(&from).unwrap().is_none());
        assert!(board.get_piece(&to).unwrap().unwrap().piece == PieceType::Rook);
    }

    #[test]
    fn test_king_line_mov() {
        let mut board = Board::default();
        let king = Piece::new_king(Color::White, Coord { row: 3, col: 3 });

        board.set_piece(king);

        let line = Rc::new(Line::new(Some(1)));

        let moves = line.allowed_moves(Coord { row: 3, col: 3 }, &board);

        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn test_initial_rook_moves() {
        let board = Board::default();

        let line = Rc::new(Line::new(None));

        let moves = line.allowed_moves(Coord { row: 0, col: 0 }, &board);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_rook_moves() {
        let mut board = Board::default();

        let rook = Piece::new_rook(Color::White, Coord { row: 3, col: 3 });
        board.set_piece(rook.clone());

        let line = Rc::new(Line::new(None));
        let moves = line.allowed_moves(rook.coord, &board);
        println!("{:?}", board);
        assert_eq!(moves.len(), 11);
    }
}
