use anyhow::{Result, bail};
use hashbrown::HashMap;

use crate::{Coord, Grid, GridNum};

#[derive(Debug, Clone, Default)]
pub struct LinearGrid<T: GridNum, V: Copy> {
    data: Vec<V>,
    width: T,
    height: T,
}

impl<T: GridNum, V: Copy> LinearGrid<T, V> {
    pub fn new(width: T, height: T, initial: V) -> Self {
        let capacity = width * height;
        let capacity: usize = capacity.try_into().unwrap_or(0);
        Self {
            data: vec![initial; capacity],
            width,
            height,
        }
    }

    pub fn get_index_from_coord(&self, coord: &Coord<T>) -> Option<usize> {
        let x: usize = coord.x().try_into().ok()?;
        let y: usize = coord.y().try_into().ok()?;
        let width: usize = self.width.try_into().ok()?;

        Some(y * width + x)
    }
}

impl<T: GridNum, V: Copy> Grid<T, V> for LinearGrid<T, V> {
    fn clear(&mut self) {
        self.data.clear();
    }

    fn check_bounds(&self, key: &Coord<T>) -> Result<()> {
        // only check bounds in debug mode for performance
        #[cfg(debug_assertions)]
        {
            if key.x() < 0u8.into() {
                bail!("Key x-coordinate is less than minimum x-coordinate");
            }
            if key.x() >= self.width {
                bail!("Key x-coordinate is greater than maximum x-coordinate");
            }
            if key.y() < 0u8.into() {
                bail!("Key y-coordinate is less than minimum y-coordinate");
            }
            if key.y() >= self.height {
                bail!("Key y-coordinate is greater than maximum y-coordinate");
            }
        }
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

    #[test]
    fn test_bounds() {
        let mut grid = LinearGrid::<i32, i32>::new(3, 3, 0);
        let in_bounds = coord(1, 1);
        let out_bounds = coord(3, 1);
        assert!(grid.insert(in_bounds, 1).is_ok());
        assert!(grid.insert(out_bounds, 2).is_err());
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
