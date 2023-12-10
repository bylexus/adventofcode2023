pub mod coord_2d;
pub use coord_2d::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

