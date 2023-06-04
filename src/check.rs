use crate::{piece::Color, Board, Coord, Piece};

/// This function checks if a move checks the king.
///
/// To use this function, a legal move should have been made
///
/// Example: https://lichess.org/analysis/4k3/8/8/b7/8/8/3R4/4K3_w_-_-_0_1?color=white
///     Rook can check the king but my king ends in check
///
/// Coord: Cell that is under check
/// Board: Board after the move
/// Enemy pieces: All enemy pieces - same color as the piece in coord
/// Ally pieces: All ally pieces - opposite color as the piece in coord
/// Returns: true if the move is check and that move does not cause an ally check
pub fn is_check(
    coord: &Coord,
    board: &mut Board,
    enemy_pieces: &Vec<&Piece>,
    ally_pieces: &Vec<&Piece>,
) -> bool {
    let enemy_color = match board.get_piece(coord) {
        Ok(Some(piece)) => piece.color,
        _ => return false, // no piece under check in that cell
    };

    let ally_king_coord = board.get_king(&enemy_color.opposite()).coord;

    // fn checks that after a move, the ally king is not under check
    let ally_in_check = |board: &mut Board| -> bool {
        for piece in enemy_pieces {
            if piece.can_move(ally_king_coord.clone(), &board) {
                return true;
            }
        }
        false
    };

    for piece in ally_pieces {
        // if an *ally* piece can move to the cell that might be under check.
        if piece.can_move(coord.clone(), &board) {
            let is_ally_in_check =
                board.temporal_move(&piece.coord, &ally_king_coord, ally_in_check);

            // if after moving an ally, the ally king is not under check, then the move is legal
            if !is_ally_in_check {
                return true;
            }
        }
    }
    false
}

/// The king is under check
/// +
/// The king cannot move into a non-check cell
/// +
/// The king cannot be protected by another piece
///    (can be protected if after an enemy move, the king is not under check)
///
/// coord: Cell that is under check
/// board: Board after the (enemy) move that might be check
/// enemy_pieces: All enemy pieces - same color as the piece in coord
/// ally_pieces: All ally pieces - opposite color as the piece in coord
///
/// Mate in 1: https://lichess.org/editor/r6k/qppppppp/8/8/8/8/PPPPPPPP/K7_b_-_-_0_1?color=white
/// Is mate: https://lichess.org/editor/r6k/1ppppppp/8/8/8/8/qPPPPPPP/K7_b_-_-_0_1?color=white
pub fn is_mate(
    coord: &Coord,
    board: &mut Board,
    enemy_pieces: &Vec<&Piece>,
    ally_pieces: &Vec<&Piece>,
) -> bool {
    let enemy_king = match board.get_piece(coord) {
        Ok(Some(piece)) => piece.clone(), // Clone because i cant borrow with an inmutable board and then use a mutable one.
        _ => return false,                // no piece under check in that cell
    };

    // Get all the enemy king possible moves
    let enemy_king_moves = enemy_king.get_moves(board);

    // The enemy king can move to a safe cell:
    for to_coord in enemy_king_moves.iter() {
        // fn checks that after a move, the ally king is not under check
        let is_tmp_move_safe = |new_board: &mut Board| -> bool {
            !is_check(&to_coord, new_board, enemy_pieces, ally_pieces)
        };

        let is_move_safe = board.temporal_move(&enemy_king.coord, to_coord, is_tmp_move_safe);

        if is_move_safe {
            return false;
        }
    }

    // Enemy piece can block or capture the piece that is checking the king
    for piece in enemy_pieces {
        // if the piece is the same we are checking, skip it
        if &piece.coord == coord {
            continue;
        }

        // check if any possible move blocks the mate
        for legal_move in piece.moves.iter() {
            for to_coord in legal_move.allowed_moves(piece.coord, board) {
                let tmp_block_capture = |new_board: &mut Board| -> bool {
                    is_check(&coord, new_board, enemy_pieces, ally_pieces)
                };

                let is_in_check = board.temporal_move(&piece.coord, &to_coord, tmp_block_capture);

                if !is_in_check {
                    return false;
                }
            }
        }
    }

    true // no possible move can avoid the mate, mate
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn test_is_check_not_mate() {
        // https://lichess.org/editor/7k/8/8/8/8/8/1p6/K7_w_-_-_0_1?color=white
        // Testing for white king
        // Ally pieces -> black
        // Enemy pieces -> white
        // We just moved the pawn to that position
        let board = Board::from_fen("k7/8/8/8/7R/8/1p6/K7 w - - 0 1").unwrap();

        let ally_pieces = &board.get_all_pieces(&Color::Black);
        let enemy_pieces = &board.get_all_pieces(&Color::White);
        let white_king = board.get_king(&Color::White);

        assert!(is_check(
            &white_king.coord,
            &mut board.clone(),
            enemy_pieces,
            ally_pieces
        ));

        assert!(!is_mate(
            &white_king.coord,
            &mut board.clone(),
            enemy_pieces,
            ally_pieces
        ));
    }

    #[test]
    fn not_a_real_check() {
        // Can't move the bishop as it both kings will be in check
        let board = Board::from_fen("8/8/8/8/R2b3k/8/8/K7 w - - 0 1").unwrap();

        let ally_pieces = &board.get_all_pieces(&Color::Black);
        let enemy_pieces = &board.get_all_pieces(&Color::White);
        let white_king = board.get_king(&Color::White);

        assert!(!is_check(
            &white_king.coord,
            &mut board.clone(),
            enemy_pieces,
            ally_pieces
        ));
    }

    #[test]
    fn test_default_board() {
        let board = Board::default();
        let ally_pieces = &board.get_all_pieces(&Color::Black);
        let enemy_pieces = &board.get_all_pieces(&Color::White);
        let white_king = board.get_king(&Color::White);

        assert!(!is_check(
            &white_king.coord,
            &mut board.clone(),
            enemy_pieces,
            ally_pieces
        ));

        assert!(!is_mate(
            &white_king.coord,
            &mut board.clone(),
            enemy_pieces,
            ally_pieces
        ));
    }

    // https://lichess.org/editor/1r6/r6k/8/8/4bR2/8/8/K7_w_-_-_0_1?color=white
    #[test]
    fn test_mate() {
        let board = Board::from_fen("1r6/r6k/8/8/4bR2/8/8/K7 w - - 0 1").unwrap();

        let ally_pieces = &board.get_all_pieces(&Color::Black);
        let enemy_pieces = &board.get_all_pieces(&Color::White);
        let white_king = board.get_king(&Color::White);

        assert!(is_check(
            &white_king.coord,
            &mut board.clone(),
            enemy_pieces,
            ally_pieces
        ));

        assert!(is_mate(
            &white_king.coord,
            &mut board.clone(),
            enemy_pieces,
            ally_pieces
        ));
    }

    // https://lichess.org/editor/1r6/r6k/8/8/4b3/8/5R2/K7_w_-_-_0_1?color=white
    #[test]
    fn test_block_mate() {
        let board = Board::from_fen("1r6/r6k/8/8/4b3/8/5R2/K7 w - - 0 1").unwrap();

        let ally_pieces = &board.get_all_pieces(&Color::Black);
        let enemy_pieces = &board.get_all_pieces(&Color::White);
        let white_king = board.get_king(&Color::White);

        assert!(is_check(
            &white_king.coord,
            &mut board.clone(),
            enemy_pieces,
            ally_pieces
        ));

        assert!(!is_mate(
            &white_king.coord,
            &mut board.clone(),
            enemy_pieces,
            ally_pieces
        ));
    }
}
