pub mod board;
pub mod errors;
pub mod moves;
pub mod notation;
pub mod piece;

pub use board::{Board, Coord};
pub use piece::{Piece, PieceType};
use pyo3::prelude::*;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn chess_model(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Board>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
