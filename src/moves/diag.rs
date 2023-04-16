use crate::board::{Board, Coord};

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

        // for each coord in the direction
        for _ in 0..self
            .max_range
            .unwrap_or(board.max_cells_direction(&direction))
        {
            println!("Current coord: {:?}", current_coord);
            let next_coord = current_coord.clone() + step.clone();

            // Get the next cell
            let next_piece = match board.get_piece(&next_coord) {
                Ok(piece) => piece,
                Err(_) => return false, // out of bounds -> false
            };

            // if the target cell
            if next_coord == to {
                match board.get_piece(&next_coord) {
                    Ok(Some(piece)) => {
                        // if the same color -> invalid
                        return piece.color != from_piece.color;
                    }
                    Ok(None) => return true, // empty cell -> valid
                    _ => unreachable!(),     // Out of bounds -> should have been caught before
                }
            }

            // if there is a pice in the way -> invalid
            if !next_piece.is_none() {
                return false;
            }
            current_coord = next_coord;
        }

        return false;
    }

    fn move_piece(
        &self,
        from: crate::board::Coord,
        to: crate::board::Coord,
        board: &mut crate::board::Board,
    ) {
        todo!()
    }

    fn allowed_moves(&self, from: Coord, board: &Board) -> Vec<Coord> {
        todo!()
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
}
