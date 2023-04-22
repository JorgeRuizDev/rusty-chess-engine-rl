use crate::{
    board::{Board, Coord},
    piece::Piece,
};

use super::Direction;

pub fn can_traverse(
    board: &Board,
    from_piece: &Piece,
    to: &Coord,
    step: &Coord,
    max_range: u32,
) -> bool {
    let mut current_coord = from_piece.coord.clone();

    for _ in 0..max_range {
        let next_coord = current_coord.clone() + step.clone();

        // Get the next cell
        let next_piece = match board.get_piece(&next_coord) {
            Ok(piece) => piece,
            Err(_) => return false, // out of bounds -> false
        };

        // if the target cell
        if &next_coord == to {
            match board.get_piece(&next_coord) {
                Ok(Some(piece)) => {
                    // if the same color -> invalid
                    return piece.color != from_piece.color;
                }
                Ok(None) => return true, // empty cell -> valid
                _ => return false,       // Out of bounds -> should have been caught before
            }
        }

        // if there is a piece in the way -> invalid
        if !next_piece.is_none() {
            return false;
        }

        current_coord = next_coord;
    }

    // couldn't reach target cell in the given direction
    false
}

pub fn legal_coords_along_direction(
    from: &Coord,
    direction: &Direction,
    board: &Board,
    from_piece: &Piece,
    max_range: u32,
) -> Vec<Coord> {
    let step = direction.get_step();
    let mut current_coord = from.clone();
    let mut legal_coords = vec![];
    // for each coord in the direction
    for _ in 0..max_range {
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
    legal_coords
}
