use anyhow::{Result, bail};
use hashbrown::HashMap;

use crate::{Coord, Grid, GridNum};

#[derive(Debug, Clone, Default)]
pub struct LinearGrid<T: GridNum, V: Copy> {
    _phantom: std::marker::PhantomData<T>,
    data: Vec<V>,
    width: usize,
    height: usize,
}

pub struct LinearGridIter<'a, T: GridNum, V: Copy> {
    grid: &'a LinearGrid<T, V>,
    index: usize,
}

impl<'a, T: GridNum + From<usize>, V: Copy> Iterator for LinearGridIter<'a, T, V> {
    type Item = (Coord<T>, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.data.len() {
            return None;
        }

        let idx = self.index;
        self.index += 1;

        let x = idx % self.grid.width;
        let y = idx / self.grid.width;

        Some((Coord(x.into(), y.into()), self.grid.data[idx]))
    }
}

impl<T: GridNum, V: Copy> LinearGrid<T, V> {
    pub fn iter(&self) -> LinearGridIter<'_, T, V> {
        LinearGridIter {
            grid: self,
            index: 0,
        }
    }
}

impl<T: GridNum, V: Copy> LinearGrid<T, V> {
    pub fn new(width: usize, height: usize, initial: V) -> Self {
        let capacity = width * height;
        Self {
            _phantom: std::marker::PhantomData,
            data: vec![initial; capacity],
            width,
            height,
        }
    }

    pub fn get_index_from_coord(&self, coord: &Coord<T>) -> Option<usize> {
        let x: usize = coord.x().try_into().ok()?;
        let y: usize = coord.y().try_into().ok()?;

        Some(y * self.width + x)
    }
}

impl<T: GridNum, V: Copy> Grid<T, V> for LinearGrid<T, V> {
    fn clear(&mut self) {
        self.data.clear();
    }

    fn check_bounds(&self, key: &Coord<T>) -> Result<()> {
        // no need to check since already constrained
        Ok(())
    }
    fn insert(&mut self, key: Coord<T>, value: V) -> Result<()> {
        self.check_bounds(&key)?;
        let index = self
            .get_index_from_coord(&key)
            .ok_or_else(|| anyhow::anyhow!("Coordinate out of bounds"))?;
        if let Some(v) = self.data.get_mut(index) {
            *v = value;
        }

        Ok(())
    }

    fn get(&self, key: &Coord<T>) -> Option<&V> {
        self.check_bounds(key).ok()?;
        let index = self
            .get_index_from_coord(key)
            .ok_or_else(|| anyhow::anyhow!("Coordinate out of bounds"))
            .ok()?;
        self.data.get(index)
    }

    // fn remove(&mut self, key: &Coord<T>) -> Option<V> {
    //     self.check_bounds(key).ok()?;
    //     self.data.remove(key)
    // }

    fn up_n(&self, coord: &Coord<T>, step: T) -> Option<V> {
        let new_coord = Coord::new(coord.x(), coord.y() - step);
        self.check_bounds(&new_coord).ok()?;

        let index = self
            .get_index_from_coord(&new_coord)
            .ok_or_else(|| anyhow::anyhow!("Coordinate out of bounds"))
            .ok()?;
        self.data.get(index).copied()
    }

    fn matches(&self, key: &Coord<T>, value: V) -> Result<bool>
    where
        V: PartialOrd,
    {
        self.check_bounds(key)?;
        let index = self
            .get_index_from_coord(key)
            .ok_or_else(|| anyhow::anyhow!("Coordinate out of bounds"))?;
        match self.data.get(index) {
            Some(grid_value) => Ok(*grid_value == value),
            None => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn coord(x: i32, y: i32) -> Coord<i32> {
        Coord::new(x, y)
    }

    #[test]
    fn test_insert_and_get() {
        let mut grid = LinearGrid::<i32, i32>::new(5, 5, 0);
        let c = coord(1, 2);
        grid.insert(c, 42).unwrap();
        assert_eq!(grid.get(&c), Some(&42));
    }

    // #[test]
    // fn test_remove() {
    //     let mut grid = LinearGrid::<i32, i32>::new(50, 50);
    //     let c = coord(5, 6);
    //     grid.insert(c, 7).unwrap();
    //     assert_eq!(grid.remove(&c), Some(7));
    //     assert_eq!(grid.get(&c), None);
    // }

    #[test]
    fn test_clear() {
        let mut grid = LinearGrid::<i32, i32>::new(50, 50, 0);
        let c = coord(9, 10);
        grid.insert(c, 5).unwrap();
        grid.clear();
        assert_eq!(grid.get(&c), None);
    }

    // To bounds checking in linear grid insert
    #[test]
    fn test_bounds() {
        let mut grid = LinearGrid::<i32, i32>::new(3, 3, 0);
        let in_bounds = coord(1, 1);
        let out_bounds = coord(3, 1);
        assert!(grid.insert(in_bounds, 1).is_ok());
        assert!(grid.insert(out_bounds, 2).is_ok());
    }

    #[test]
    fn test_up_n() {
        let mut grid = LinearGrid::<i32, i32>::new(5, 5, 0);
        let c = coord(2, 2);
        let up = coord(2, 1);
        grid.insert(up, 99).unwrap();
        assert_eq!(grid.up_n(&c, 1), Some(99));
        assert_eq!(grid.up_n(&c, 2), Some(0));
    }

    #[test]
    fn test_matches() {
        let mut grid = LinearGrid::<i32, i32>::new(5, 5, 0);
        let c = coord(0, 0);
        grid.insert(c, 123).unwrap();
        assert!(grid.matches(&c, 123).unwrap());
        assert!(!grid.matches(&c, 456).unwrap());
        let missing = coord(1, 1);
        assert!(!grid.matches(&missing, 123).unwrap());
    }
}
