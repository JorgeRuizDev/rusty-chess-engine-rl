use crate::{
    board::{BoardInfo, Coord},
    piece::{Color, Piece},
};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, LinkedList};

use super::AlgebraicNotation;

#[derive(Debug, PartialEq)]
pub enum FenError {
    InvalidFen(String),
    InvalidPiece(String),
    InvalidGameInfo(String),
}

lazy_static! {
    static ref FEN_REGEX: Regex = Regex::new(
        r"^((([pnbrqkPNBRQK1-8]{1,8})/?){8})\s+(b|w)\s+(-|K?Q?k?q)\s+(-|[a-h][3-6])\s+(\d+)\s+(\d+)\s*",
    )
    .unwrap();
}

const OFFICIAL_BOARD_COLS: i32 = 8;

pub const INITIAL_BOARD: &str = r"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn is_valid(fen: &str) -> bool {
    FEN_REGEX.is_match(fen)
}

fn char_to_piece(c: char, row: i32, col: i32) -> Result<Piece, FenError> {
    if !c.is_alphabetic() {
        return Err(FenError::InvalidPiece(format!("Invalid piece {}", c)));
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
        _ => return Err(FenError::InvalidPiece(format!("Invalid piece {}", c))),
    };

    Ok(piece)
}

/// Parses the `w KQkq - 0 1` part of a Fen String
///
/// The input should be the splited string with 5 elements
/// 1. Turn
/// 2. Castling rights
/// 3. En passant target cell
/// 4. Halfmove clock
/// 5. Fullmove number
///  
fn parse_board_info(last_row: Vec<&str>) -> Result<BoardInfo, FenError> {
    if last_row.len() != 5 {
        return Err(FenError::InvalidGameInfo(format!(
            "Incorrect number of game info, expected 5, got {}",
            last_row.len()
        )));
    }

    let turn = match last_row[0] {
        "w" => Color::White,
        "b" => Color::Black,
        _ => {
            return Err(FenError::InvalidGameInfo(format!(
                "Invalid turn {}",
                last_row[0]
            )))
        }
    };
    let mut castling_rights: HashMap<Color, Vec<Coord>> = HashMap::new();
    for c in last_row[1].chars() {
        let (color, coord) = match c {
            'K' => (Color::White, Coord { row: 0, col: 6 }),
            'Q' => (Color::White, Coord { row: 0, col: 2 }),
            'k' => (Color::Black, Coord { row: 7, col: 6 }),
            'q' => (Color::Black, Coord { row: 7, col: 2 }),
            '-' => break,
            _ => {
                return Err(FenError::InvalidGameInfo(format!(
                    "Invalid castling right {}",
                    c
                )))
            }
        };
        castling_rights.entry(color).or_insert(vec![]).push(coord);
    }

    let alg_parser = AlgebraicNotation { rows: 8, cols: 8 };

    let en_passant = match last_row[2] {
        "-" => None,
        s => match alg_parser.cell_from_str(s) {
            Ok(coord) => Some(coord),
            Err(_) => {
                return Err(FenError::InvalidGameInfo(format!(
                    "Invalid en passant {}",
                    last_row[3]
                )))
            }
        },
    };

    let halfmove_clock = match last_row[3].parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            return Err(FenError::InvalidGameInfo(format!(
                "Invalid halfmove clock {}",
                last_row[4]
            )))
        }
    };

    let fullmove_number = match last_row[4].parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            return Err(FenError::InvalidGameInfo(format!(
                "Invalid fullmove number {}",
                last_row[5]
            )))
        }
    };

    Ok(BoardInfo {
        turn,
        castling: castling_rights,
        en_passant,
        halfmove_clock,
        fullmove_number,
    })
}

/// Parse function for *FEN* notation
///
/// The FEN String represents the board state.
///
/// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
pub fn parse(fen: &str) -> Result<(LinkedList<Piece>, BoardInfo), FenError> {
    if !is_valid(fen) {
        return Err(FenError::InvalidFen(format!("Invalid FEN: {}", fen)));
    }

    let mut pieces = LinkedList::new();

    let mut rows: Vec<&str> = fen.split("/").collect();

    // Get last row
    let info_row = rows.pop().unwrap();
    let mut info_row = info_row.split_whitespace(); // remove everything after the whitespace

    // Removes the firest item from the iterator -> the last row
    rows.push(info_row.next().unwrap()); // remove everything after the whitespace

    let board_info = parse_board_info(info_row.collect())?;

    // For each row
    for (row_idx, row) in rows.iter().enumerate() {
        let mut col = 0;
        // For each element in the row
        for c in row.chars() {
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
            return Err(FenError::InvalidFen(format!(
                "Invalid Fen, row {} has {} columns instead of {}",
                row_idx, col, OFFICIAL_BOARD_COLS
            )));
        }
    }

    Ok((pieces, board_info))
}

mod tests {

    use crate::{board::Coord, piece::Color};

    use super::{is_valid, parse, INITIAL_BOARD};

    #[test]
    fn test_fen_regex() {
        let fen = INITIAL_BOARD;
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
        let fen = INITIAL_BOARD;
        let (pieces, _) = parse(&fen).unwrap();
        assert_eq!(pieces.len(), 32);
    }

    #[test]
    fn test_board_info() {
        let fen = INITIAL_BOARD;
        let (_, board_info) = parse(&fen).unwrap();
        assert_eq!(board_info.turn, Color::White);
        assert_eq!(board_info.castling.len(), 2);
        assert_eq!(board_info.castling.get(&Color::White).unwrap().len(), 2);
        assert_eq!(board_info.castling.get(&Color::Black).unwrap().len(), 2);
        assert_eq!(board_info.en_passant, None);
        assert_eq!(board_info.halfmove_clock, 0);
        assert_eq!(board_info.fullmove_number, 1);
    }

    #[test]
    fn test_en_passant() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e3 0 1";
        let (_, board_info) = parse(&fen).unwrap();
        assert_eq!(board_info.en_passant, Some(Coord { row: 5, col: 4 }));
    }

    #[test]
    fn test_castling_rights() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let (_, board_info) = parse(&fen).unwrap();

        let black_rights = board_info.castling.get(&Color::Black).unwrap();
        assert_eq!(black_rights.len(), 2);

        assert!(black_rights.contains(&Coord { row: 7, col: 2 }));
        assert!(black_rights.contains(&Coord { row: 7, col: 6 }));

        let white_rights = board_info.castling.get(&Color::White).unwrap();
        assert_eq!(white_rights.len(), 2);
        assert!(white_rights.contains(&Coord { row: 0, col: 2 }));
        assert!(white_rights.contains(&Coord { row: 0, col: 6 }));
    }

    #[test]
    fn test_row_color() {
        // Tests that row 0 is black and row 7 is black

        let (pieces, board_info) = parse(INITIAL_BOARD).unwrap();
        const color: Color = Color::White;

        // White Castling
        assert_eq!(
            board_info
                .castling
                .get(&color)
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .row,
            0
        );

        assert_eq!(
            pieces
                .iter()
                .filter(|p| p.color == color && p.coord.row == 7)
                .count(),
            8
        )
    }
}
