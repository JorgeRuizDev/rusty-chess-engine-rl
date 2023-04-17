use crate::{
    board::Coord,
    piece::{Color, Piece},
};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::LinkedList;

/// Static methods for *FEN* notation
///
/// The FEN String represents the board state.
///
/// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation

#[derive(Debug, PartialEq)]
pub enum FenError {
    InvalidFen,
    InvalidPiece,
}

lazy_static! {
    static ref FEN_REGEX: Regex = Regex::new(
        r"^((([pnbrqkPNBRQK1-8]{1,8})/?){8})\s+(b|w)\s+(-|K?Q?k?q)\s+(-|[a-h][3-6])\s+(\d+)\s+(\d+)\s*",
    )
    .unwrap();
}

const OFFICIAL_BOARD_COLS: i32 = 8;

const INITIAl_BOARD: &str = r"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn is_valid(fen: &str) -> bool {
    FEN_REGEX.is_match(fen)
}

fn char_to_piece(c: char, row: i32, col: i32) -> Result<Piece, FenError> {
    if !c.is_alphabetic() {
        return Err(FenError::InvalidPiece);
    }

    let color = match c.is_uppercase() {
        true => Color::White,
        false => Color::Black,
    };

    let coord = Coord { row, col };

    let piece = match c.to_ascii_lowercase() {
        'p' => Piece::new_pawn(color, coord),
        'n' => Piece::new_knight(color, coord),
        'b' => Piece::new_bishop(color, coord),
        'r' => Piece::new_rook(color, coord),
        'q' => Piece::new_pawn(color, coord),
        'k' => Piece::new_king(color, coord),
        _ => return Err(FenError::InvalidPiece),
    };

    Ok(piece)
}

pub fn parse(fen: &str) -> Result<LinkedList<Piece>, FenError> {
    if !is_valid(fen) {
        return Err(FenError::InvalidFen);
    }

    let mut pieces = LinkedList::new();

    let mut rows: Vec<&str> = fen.split("/").collect();

    // Get last row
    let last_row = rows.pop().unwrap();
    rows.push(last_row.split_whitespace().next().unwrap()); // remove everything after the whitespace

    for (row_idx, row) in rows.iter().enumerate() {
        let mut col = 0;
        for c in row.chars() {
            // if c is a space, break
            if c.is_digit(10) {
                col += c.to_digit(10).unwrap() as i32;
            } else if c.is_alphabetic() {
                let piece = char_to_piece(c, row_idx as i32, col)?;
                pieces.push_back(piece);
                col += 1;
            } else {
                unreachable!("Invalid Fen that has passed the regex check");
            }
        }

        if col != OFFICIAL_BOARD_COLS {
            return Err(FenError::InvalidFen);
        }
    }

    Ok(pieces)
}
mod tests {

    use super::{is_valid, parse, INITIAl_BOARD};

    #[test]
    fn test_fen_regex() {
        let fen = INITIAl_BOARD;
        assert!(is_valid(fen), "Fen is valid");

        // invalid fen with a space instead of a p
        let fen = "rnbqkbnr/pppppxpp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(
            !is_valid(fen),
            "Fen is invalid, contains an x instead of a p"
        );

        // invalid fen with a space instead of a p
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(!is_valid(fen), "Fen is invalid, contains 9 rows");

        let fen = "rnbqkbnr/ppppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(
            !is_valid(fen),
            "Fen is invalid, contains a row with 9 pieces"
        );
    }

    #[test]
    fn test_piece_builder() {
        let fen = INITIAl_BOARD;
        let pieces = parse(&fen).unwrap();
        assert_eq!(pieces.len(), 32);
    }
}
