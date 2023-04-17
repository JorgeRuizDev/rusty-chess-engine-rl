use crate::board::Coord;

pub struct AlgebraicNotation {
    rows: u32,
    cols: u32,
}

pub enum AlgebraicNotationError {}

impl AlgebraicNotation {
    pub fn from_str() -> Result<Coord, AlgebraicNotationError> {
        todo!();
    }
}
