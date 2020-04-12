extern crate derive_more;
use derive_more::{Add, Sub};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Add, Sub, Debug)]
pub struct Coord(pub f64, pub f64);

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
