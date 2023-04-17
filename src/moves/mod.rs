use super::board::{Board, Coord, HasCoordinates};
pub mod avoid_capture;
pub mod castle;
pub mod diag;
pub mod en_passant;
pub mod jump;
pub mod line;
pub mod prom;

// Re-export the modules:
pub use diag::Diagonal;
pub use line::Line;

pub trait Move {
    fn is_move_valid(&self, from: Coord, to: Coord, board: &Board) -> bool;
    /// Moves a piece from one cell to another
    /// Does not check if the move is valid
    ///
    /// # Arguments
    /// * `from` - The starting cell
    /// * `to` - The target cell
    /// * `board` - The board on which the piece is moved
    /// # Returns
    /// Nothing
    fn move_piece(&self, from: Coord, to: Coord, board: &mut Board) {
        let mut from_piece = match board.get_piece(&from) {
            Ok(Some(piece)) => piece.clone(),
            _ => return,
        };

        from_piece.coord = to;
        from_piece.has_moved = true;

        board.set_piece(from_piece);
        board.remove_piece(&from)
    }
    fn allowed_moves(&self, from: Coord, board: &Board) -> Vec<Coord>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Direction {
    pub fn get_step(&self) -> Coord {
        match self {
            Direction::North => Coord { row: 1, col: 0 },
            Direction::South => Coord { row: -1, col: 0 },
            Direction::East => Coord { row: 0, col: 1 },
            Direction::West => Coord { row: 0, col: -1 },
            Direction::NorthWest => Coord { row: 1, col: -1 },
            Direction::NorthEast => Coord { row: 1, col: 1 },
            Direction::SouthWest => Coord { row: -1, col: -1 },
            Direction::SouthEast => Coord { row: -1, col: 1 },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DirectionError {
    SameOriginDestiny,
    InvalidDirection,
}

pub fn parse_direction<T: HasCoordinates>(from: &T, to: &T) -> Result<Direction, DirectionError> {
    let from_coord = from.get_coordinates();
    let to_coord = to.get_coordinates();

    let row_diff = (to_coord.row as i32) - (from_coord.row as i32);
    let col_diff = (to_coord.col as i32) - (from_coord.col as i32);

    if row_diff == 0 && col_diff == 0 {
        return Err(DirectionError::SameOriginDestiny);
    }

    if row_diff == 0 {
        if col_diff > 0 {
            return Ok(Direction::East);
        } else {
            return Ok(Direction::West);
        }
    }

    if col_diff == 0 {
        if row_diff > 0 {
            return Ok(Direction::North);
        } else {
            return Ok(Direction::South);
        }
    }

    if row_diff == col_diff {
        if row_diff > 0 {
            return Ok(Direction::NorthEast);
        } else {
            return Ok(Direction::SouthWest);
        }
    }

    if row_diff == -col_diff {
        if row_diff > 0 {
            return Ok(Direction::NorthWest);
        } else {
            return Ok(Direction::SouthEast);
        }
    }

    Err(DirectionError::InvalidDirection)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Coord;

    #[test]
    fn parse_direction_test() {
        // Test same origin and destiny
        let from = Coord { row: 0, col: 0 };
        let to = Coord { row: 0, col: 0 };
        assert!(matches!(
            parse_direction(&from, &to),
            Err(DirectionError::SameOriginDestiny)
        ));

        // Test invalid direction
        let from = Coord { row: 0, col: 0 };
        let to = Coord { row: 2, col: 3 };
        assert!(matches!(
            parse_direction(&from, &to),
            Err(DirectionError::InvalidDirection)
        ));

        // Test valid directions
        let from = Coord { row: 0, col: 0 };
        let to = Coord { row: 0, col: 1 };
        assert_eq!(parse_direction(&from, &to), Ok(Direction::East));

        let from = Coord { row: 0, col: 1 };
        let to = Coord { row: 0, col: 0 };
        assert_eq!(parse_direction(&from, &to), Ok(Direction::West));

        let from = Coord { row: 0, col: 0 };
        let to = Coord { row: 1, col: 0 };
        assert_eq!(parse_direction(&from, &to), Ok(Direction::North));

        let from = Coord { row: 1, col: 0 };
        let to = Coord { row: 0, col: 0 };
        assert_eq!(parse_direction(&from, &to), Ok(Direction::South));

        let from = Coord { row: 0, col: 0 };
        let to = Coord { row: 1, col: 1 };
        assert_eq!(parse_direction(&from, &to), Ok(Direction::NorthEast));

        let from = Coord { row: 1, col: 1 };
        let to = Coord { row: 0, col: 0 };
        assert_eq!(parse_direction(&from, &to), Ok(Direction::SouthWest));

        let from = Coord { row: 0, col: 1 };
        let to = Coord { row: 1, col: 0 };
        assert_eq!(parse_direction(&from, &to), Ok(Direction::NorthWest));

        let from = Coord { row: 1, col: 0 };
        let to = Coord { row: 0, col: 1 };
        assert_eq!(parse_direction(&from, &to), Ok(Direction::SouthEast));
    }
}
