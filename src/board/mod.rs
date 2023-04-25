mod board;
mod board_info;

pub use board::Board;
pub use board_info::BoardInfo;
pub use board_info::CastlingRights;
use pyo3::prelude::*;
use std::ops::Add;
pub trait HasCoordinates {
    fn get_coordinates(&self) -> Coord;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[pyclass]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}

impl HasCoordinates for Coord {
    fn get_coordinates(&self) -> Coord {
        *self
    }
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Self::Output {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}
