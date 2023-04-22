use crate::board::Coord;

pub struct AlgebraicNotation {
    pub rows: u32,
    pub cols: u32,
}

#[derive(Debug, PartialEq)]
pub enum AlgebraicNotationError {
    InvalidString(String),
    InvalidCell(String),
}

impl AlgebraicNotation {
    pub fn cell_from_str(&self, cell: &str) -> Result<Coord, AlgebraicNotationError> {
        if cell.len() != 2 {
            return Err(AlgebraicNotationError::InvalidString(
                "Invalid cell".to_string(),
            ));
        }

        let mut chars = cell.chars();
        let (col, row) = (chars.next(), chars.next());

        if col.is_none() || row.is_none() {
            return Err(AlgebraicNotationError::InvalidCell(
                "Invalid cell".to_string(),
            ));
        }

        // notation 8 -> board row 0
        // Notation a -> board col 0

        let col = col.unwrap() as u32 - 'a' as u32;
        let row = row.unwrap() as u32 - '1' as u32;

        if col >= self.cols || row >= self.rows {
            return Err(AlgebraicNotationError::InvalidCell(
                "Invalid cell".to_string(),
            ));
        }

        Ok(Coord {
            row: ((row as i32) - (self.rows as i32 - 1)).abs(),
            col: col as i32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cell() {
        let algebraic_notation = AlgebraicNotation { rows: 8, cols: 8 };
        assert_eq!(
            algebraic_notation.cell_from_str("a1").unwrap(),
            Coord { row: 7, col: 0 }
        );
        assert_eq!(
            algebraic_notation.cell_from_str("h8").unwrap(),
            Coord { row: 0, col: 7 }
        );

        assert_eq!(
            algebraic_notation.cell_from_str("a8").unwrap(),
            Coord { row: 0, col: 0 }
        );
        assert_eq!(
            algebraic_notation.cell_from_str("h1").unwrap(),
            Coord { row: 7, col: 7 }
        );
    }

    #[test]
    fn test_invalid_string() {
        let algebraic_notation = AlgebraicNotation { rows: 8, cols: 8 };
        assert_eq!(
            algebraic_notation.cell_from_str("a"),
            Err(AlgebraicNotationError::InvalidString(
                "Invalid cell".to_string()
            ))
        );
        assert_eq!(
            algebraic_notation.cell_from_str("a12"),
            Err(AlgebraicNotationError::InvalidString(
                "Invalid cell".to_string()
            ))
        );
    }

    #[test]
    fn test_invalid_cell() {
        let algebraic_notation = AlgebraicNotation { rows: 8, cols: 8 };
        assert_eq!(
            algebraic_notation.cell_from_str("i1"),
            Err(AlgebraicNotationError::InvalidCell(
                "Invalid cell".to_string()
            ))
        );
        assert_eq!(
            algebraic_notation.cell_from_str("a9"),
            Err(AlgebraicNotationError::InvalidCell(
                "Invalid cell".to_string()
            ))
        );
    }

    #[test]
    fn test_row_equivalence() {
        let black_king = "e8";

        let algebraic_notation = AlgebraicNotation { rows: 8, cols: 8 };
        assert_eq!(
            algebraic_notation.cell_from_str(black_king).unwrap(),
            Coord { row: 0, col: 4 }
        );
    }
}
