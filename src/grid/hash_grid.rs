use anyhow::{Result, bail};
use hashbrown::HashMap;

use crate::{Coord, Grid, GridNum};

#[derive(Debug, Clone, Default)]
pub struct HashGrid<T: GridNum, V: Copy> {
    data: HashMap<Coord<T>, V>,
    min_x: Option<T>,
    max_x: Option<T>,
    min_y: Option<T>,
    max_y: Option<T>,
}

impl<T: GridNum, V: Copy> Grid<T, V> for HashGrid<T, V> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
        }
    }
    fn set_min_x(mut self, min_x: T) -> Self {
        self.min_x = Some(min_x);
        self
    }

    fn set_max_x(mut self, max_x: T) -> Self {
        self.max_x = Some(max_x);
        self
    }

    fn set_min_y(mut self, min_y: T) -> Self {
        self.min_y = Some(min_y);
        self
    }

    fn set_max_y(mut self, max_y: T) -> Self {
        self.max_y = Some(max_y);
        self
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn check_bounds(&self, key: &Coord<T>) -> Result<()> {
        // only check bounds in debug mode for performance
        #[cfg(debug_assertions)]
        {
            if let Some(min_x) = self.min_x
                && key.x() < min_x
            {
                bail!("Key x-coordinate is less than minimum x-coordinate");
            }
            if let Some(max_x) = self.max_x
                && key.x() > max_x
            {
                bail!("Key x-coordinate is greater than maximum x-coordinate");
            }
            if let Some(min_y) = self.min_y
                && key.y() < min_y
            {
                bail!("Key y-coordinate is less than minimum y-coordinate");
            }
            if let Some(max_y) = self.max_y
                && key.y() > max_y
            {
                bail!("Key y-coordinate is greater than maximum y-coordinate");
            }
        }
        Ok(())
    }
    fn insert(&mut self, key: Coord<T>, value: V) -> Result<()> {
        self.check_bounds(&key)?;
        self.data.insert(key, value);

        Ok(())
    }
    fn insert_or_ignore(&mut self, key: Coord<T>, value: V) -> Result<()> {
        self.check_bounds(&key)?;
        self.data.entry(key).or_insert(value);
        Ok(())
    }

    fn get(&self, key: &Coord<T>) -> Option<&V> {
        self.check_bounds(key).ok()?;
        self.data.get(key)
    }

    fn remove(&mut self, key: &Coord<T>) -> Option<V> {
        self.check_bounds(key).ok()?;
        self.data.remove(key)
    }

    fn contains_key(&self, key: &Coord<T>) -> bool {
        if self.check_bounds(key).is_err() {
            return false;
        }
        self.data.contains_key(key)
    }

    fn up_n(&self, coord: &Coord<T>, step: T) -> Option<V> {
        let new_coord = Coord::new(coord.x(), coord.y() - step);
        self.check_bounds(&new_coord).ok()?;

        self.data.get(&new_coord).copied()
    }

    fn matches(&self, key: &Coord<T>, value: V) -> Result<bool>
    where
        V: PartialOrd,
    {
        self.check_bounds(key)?;
        match self.data.get(key) {
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
        let mut grid = HashGrid::<i32, i32>::new();
        let c = coord(1, 2);
        grid.insert(c, 42).unwrap();
        assert_eq!(grid.get(&c), Some(&42));
    }

    #[test]
    fn test_insert_or_ignore() {
        let mut grid = HashGrid::<i32, i32>::new();
        let c = coord(3, 4);
        grid.insert_or_ignore(c, 10).unwrap();
        grid.insert_or_ignore(c, 99).unwrap();
        assert_eq!(grid.get(&c), Some(&10));
    }

    #[test]
    fn test_remove() {
        let mut grid = HashGrid::<i32, i32>::new();
        let c = coord(5, 6);
        grid.insert(c, 7).unwrap();
        assert_eq!(grid.remove(&c), Some(7));
        assert_eq!(grid.get(&c), None);
    }

    #[test]
    fn test_contains_key() {
        let mut grid = HashGrid::<i32, i32>::new();
        let c = coord(7, 8);
        assert!(!grid.contains_key(&c));
        grid.insert(c, 1).unwrap();
        assert!(grid.contains_key(&c));
    }

    #[test]
    fn test_clear() {
        let mut grid = HashGrid::<i32, i32>::new();
        let c = coord(9, 10);
        grid.insert(c, 5).unwrap();
        grid.clear();
        assert_eq!(grid.get(&c), None);
    }

    #[test]
    fn test_bounds() {
        let mut grid = HashGrid::<i32, i32>::new()
            .set_min_x(0)
            .set_max_x(2)
            .set_min_y(0)
            .set_max_y(2);
        let in_bounds = coord(1, 1);
        let out_bounds = coord(3, 1);
        assert!(grid.insert(in_bounds, 1).is_ok());
        assert!(grid.insert(out_bounds, 2).is_err());
    }

    #[test]
    fn test_up_n() {
        let mut grid = HashGrid::<i32, i32>::new();
        let c = coord(2, 2);
        let up = coord(2, 1);
        grid.insert(up, 99).unwrap();
        assert_eq!(grid.up_n(&c, 1), Some(99));
        assert_eq!(grid.up_n(&c, 2), None);
    }

    #[test]
    fn test_matches() {
        let mut grid = HashGrid::<i32, i32>::new();
        let c = coord(0, 0);
        grid.insert(c, 123).unwrap();
        assert!(grid.matches(&c, 123).unwrap());
        assert!(!grid.matches(&c, 456).unwrap());
        let missing = coord(1, 1);
        assert!(!grid.matches(&missing, 123).unwrap());
    }
}
