#![allow(dead_code, unused_imports, unused_variables)]
use anyhow::{Result, bail};
use num_traits::{CheckedAdd, CheckedSub};
use std::hash::Hash;

use hashbrown::HashMap;

mod hash_grid;
pub use hash_grid::*;

use crate::Coord;

pub trait GridNum:
    Copy
    + Hash
    + Eq
    + PartialOrd
    + Ord
    + CheckedAdd
    + CheckedSub
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + From<u8>
{
}
impl<T> GridNum for T where
    T: Copy
        + Hash
        + Eq
        + PartialOrd
        + Ord
        + CheckedAdd
        + CheckedSub
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
    fn new() -> Self;
    fn insert(&mut self, key: Coord<T>, value: V) -> Result<()>;
    fn get(&self, key: &Coord<T>) -> Option<&V>;
    fn remove(&mut self, key: &Coord<T>) -> Option<V>;
    fn contains_key(&self, key: &Coord<T>) -> bool;
    fn clear(&mut self);
    fn matches(&self, key: &Coord<T>, value: V) -> Result<bool>
    where
        V: PartialOrd;
    fn up_n(&self, coord: &Coord<T>, step: T) -> Option<V>;
    fn insert_or_ignore(&mut self, key: Coord<T>, value: V) -> Result<()>;
    fn check_bounds(&self, key: &Coord<T>) -> Result<()>;
    fn set_min_x(self, min: T) -> Self;
    fn set_max_x(self, min: T) -> Self;
    fn set_min_y(self, min: T) -> Self;
    fn set_max_y(self, min: T) -> Self;
}
