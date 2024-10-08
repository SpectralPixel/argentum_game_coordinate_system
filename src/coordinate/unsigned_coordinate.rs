use std::ops::{BitAnd, BitOr, BitXor, Not};

use argentum_game_coordinate_system_macros::Coordinate;
use min_max_traits::{Max, Min};
use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, Unsigned};
use quickcheck::Arbitrary;

use super::coordinate_trait::CoordinateTrait;

#[cfg(test)]
mod tests;

#[derive(Coordinate, Clone, Debug, PartialEq)]
pub struct UnsignedCoordinate<T>
where
    T: Integer
        + Unsigned
        + Copy
        + CheckedAdd
        + CheckedSub
        + CheckedMul
        + CheckedDiv
        + Display
        + Max
        + Min
        + Arbitrary
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
        + Not<Output = T>,
{
    pub x: T,
    pub y: T,
    pub z: T,
}
