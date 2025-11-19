#![allow(dead_code, unused_imports, unused_variables)]
use anyhow::{Result, bail};
use num_traits::{CheckedAdd, CheckedSub, Num};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::Mul,
};

use hashbrown::HashMap;

mod hash_grid;
mod linear_grid;
pub use hash_grid::*;
pub use linear_grid::*;

use crate::Coord;

pub trait GridNum:
    Display
    + Debug
    + Copy
    + Hash
    + Eq
    + PartialOrd
    + Ord
    + Num
    + TryInto<usize>
    + CheckedAdd
    + CheckedSub
    + Mul
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + From<u8>
{
}
impl<T> GridNum for T where
    T: Copy
        + Display
        + Debug
        + Hash
        + Eq
        + PartialOrd
        + Ord
        + Num
        + TryInto<usize>
        + CheckedAdd
        + CheckedSub
        + Mul
        + std::ops::Add<Output = Self>
        + std::ops::Sub<Output = Self>
        + From<u8>
{
}

// TODO: Implement methods for StaticGrid
pub struct StaticGrid<T: Copy, const W: usize, const H: usize> {
    _data: [[T; W]; H],
}

pub trait Grid<T: GridNum, V: Copy> {
    fn insert(&mut self, key: Coord<T>, value: V) -> Result<()>;
    fn get(&self, key: &Coord<T>) -> Option<&V>;
    // fn remove(&mut self, key: &Coord<T>) -> Option<V>;
    fn clear(&mut self);
    fn matches(&self, key: &Coord<T>, value: V) -> Result<bool>
    where
        V: PartialOrd;
    fn up_n(&self, coord: &Coord<T>, step: T) -> Option<V>;
    fn check_bounds(&self, key: &Coord<T>) -> Result<()>;
}
