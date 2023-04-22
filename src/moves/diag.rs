use crate::{
    board::{Board, Coord},
    moves::util::can_traverse,
};

use super::{parse_direction, Direction, Move};

pub struct Diagonal {
    max_range: Option<u32>,
}

impl Diagonal {
    pub fn new(max_range: Option<u32>) -> Diagonal {
        Diagonal { max_range }
    }
}

impl Move for Diagonal {
    fn is_move_valid(&self, from: Coord, to: Coord, board: &Board) -> bool {
        let from_piece = match board.get_piece(&from) {
            Ok(Some(piece)) => piece,
            _ => return false,
        };

        let direction = match parse_direction(&from, &to) {
            Ok(direction) => match direction {
                Direction::NorthEast
                | Direction::NorthWest
                | Direction::SouthEast
                | Direction::SouthWest => direction,
                _ => return false,
            },
            _ => return false,
        };
        println!("Direction: {:?}", direction);
        let step = direction.get_step();

        let mut current_coord = from;

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

        let mut legal_coords: Vec<Coord> = vec![];

        for direction in [
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
        ] {
            let step = direction.get_step();

            let mut current_coord = from;

            // for each coord in the direction
            for _ in 0..self
                .max_range
                .unwrap_or(board.max_cells_direction(&direction))
            {
                let next_coord = current_coord.clone() + step.clone();

                // Get the next cell
                let next_piece = match board.get_piece(&next_coord) {
                    Ok(piece) => piece,
                    Err(_) => break, // out of bounds -> false
                };

                match next_piece {
                    None => legal_coords.push(next_coord), // Empty cell
                    Some(piece) => {
                        // if the same color -> invalid
                        if piece.color != from_piece.color {
                            legal_coords.push(next_coord);
                        }
                        break; // Break -> There is a piece blocking the way (friendly & enemy)
                    }
                }
                current_coord = next_coord;
            }
        }

        return legal_coords;
    }
}

mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::board::{Board, Coord};
    use crate::piece::{Color, Piece, PieceType};

    fn prepare(coord: Coord) -> (Board, Rc<Diagonal>, Coord) {
        let mut board = Board::new(Some(3), Some(3));
        let movement = Rc::new(Diagonal::new(None));

        let piece = Piece::new(
            Color::Black,
            PieceType::Bishop,
            vec![movement.clone()],
            coord,
        );
        board.set_piece(piece);

        (board, movement, coord)
    }

    #[test]
    fn test_is_move_valid() {
        let (board, movement, from) = prepare(Coord { row: 1, col: 1 });

        println!("{:?}", &board);

        for (to, expect) in vec![
            (Coord { row: 0, col: 0 }, true),
            (Coord { row: 0, col: 1 }, false),
            (Coord { row: 0, col: 2 }, true),
            (Coord { row: 1, col: 0 }, false),
            (Coord { row: 1, col: 1 }, false),
            (Coord { row: 1, col: 2 }, false),
            (Coord { row: 2, col: 0 }, true),
            (Coord { row: 2, col: 1 }, false),
            (Coord { row: 2, col: 2 }, true),
        ] {
            println!("Testing {:?} -> {:?}", from, to);
            assert_eq!(movement.is_move_valid(from, to, &board), expect);
        }
    }

    #[test]
    fn test_max_range() {
        let (board, movement, from) = prepare(Coord { row: 0, col: 0 });

        for (to, expect) in vec![
            (Coord { row: 0, col: 0 }, false),
            (Coord { row: 0, col: 1 }, false),
            (Coord { row: 0, col: 2 }, false),
            (Coord { row: 1, col: 0 }, false),
            (Coord { row: 1, col: 1 }, true),
            (Coord { row: 1, col: 2 }, false),
            (Coord { row: 2, col: 0 }, false),
            (Coord { row: 2, col: 1 }, false),
            (Coord { row: 2, col: 2 }, true),
        ] {
            println!("Testing {:?} -> {:?}", from, to);
            assert_eq!(movement.is_move_valid(from, to, &board), expect);
        }

        println!("Limited Movement");
        let limit_movement = Diagonal::new(Some(1));
        for (to, expect) in vec![
            (Coord { row: 0, col: 0 }, false),
            (Coord { row: 0, col: 1 }, false),
            (Coord { row: 0, col: 2 }, false),
            (Coord { row: 1, col: 0 }, false),
            (Coord { row: 1, col: 1 }, true),
            (Coord { row: 1, col: 2 }, false),
            (Coord { row: 2, col: 0 }, false),
            (Coord { row: 2, col: 1 }, false),
            (Coord { row: 2, col: 2 }, false),
        ] {
            println!("Testing {:?} -> {:?}", from, to);
            assert_eq!(limit_movement.is_move_valid(from, to, &board), expect);
        }
    }

    #[test]
    fn test_bishop_center_board() {
        let mut board = Board::default();
        let movement = Rc::new(Diagonal::new(None));

        let bishop = Piece::new(
            Color::Black,
            PieceType::Bishop,
            vec![movement.clone()],
            Coord { row: 3, col: 3 },
        );
        board.set_piece(bishop.clone());

        let moves = movement.allowed_moves(bishop.coord, &board);
        println!("{:?}", moves);
        println!("{:?}", &board);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_white_bishop() {
        let board = Board::default();

        let from = Coord { row: 7, col: 2 };
        let movement = Rc::new(Diagonal::new(None));

        let moves = movement.allowed_moves(from, &board);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_1_step() {
        let mut board = Board::default();
        let movement = Rc::new(Diagonal::new(Some(1)));

        let king = Piece::new(
            Color::Black,
            PieceType::King,
            vec![movement.clone()],
            Coord { row: 4, col: 4 },
        );

        board.set_piece(king.clone());

        let moves = movement.allowed_moves(king.coord, &board);
        assert_eq!(moves.len(), 4);
    }
}
