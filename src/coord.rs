extern crate derive_more;
use derive_more::{Add, Sub};

#[derive(Clone, Copy, PartialEq, Add, Sub)]
pub struct Coord(pub f64, pub f64);
