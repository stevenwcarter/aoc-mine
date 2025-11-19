use crate::GridNum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord<T: GridNum>(pub T, pub T);

impl<T: GridNum> Coord<T> {
    pub fn new(x: T, y: T) -> Self {
        Coord(x, y)
    }
    pub fn x(&self) -> T {
        self.0
    }
    pub fn y(&self) -> T {
        self.1
    }
    pub fn range_contains(&self, top_left: &Coord<T>, bottom_right: &Coord<T>) -> bool {
        self.0 >= top_left.0
            && self.0 <= bottom_right.0
            && self.1 >= top_left.1
            && self.1 <= bottom_right.1
    }
    pub fn up_n(&self, n: T) -> Option<Self> {
        let new_y = self.1.checked_sub(&n)?;
        Some(Coord(self.0, new_y))
    }
    pub fn down_n(&self, n: T) -> Option<Self> {
        let new_y = self.1.checked_add(&n)?;
        Some(Coord(self.0, new_y))
    }
    pub fn left_n(&self, n: T) -> Option<Self> {
        let new_x = self.0.checked_sub(&n)?;
        Some(Coord(new_x, self.1))
    }
    pub fn right_n(&self, n: T) -> Option<Self> {
        let new_x = self.0.checked_add(&n)?;
        Some(Coord(new_x, self.1))
    }
    pub fn up(&self) -> Option<Self> {
        self.up_n(T::from(1u8))
    }
    pub fn down(&self) -> Option<Self> {
        self.down_n(T::from(1u8))
    }
    pub fn left(&self) -> Option<Self> {
        self.left_n(T::from(1u8))
    }
    pub fn right(&self) -> Option<Self> {
        self.right_n(T::from(1u8))
    }
}

impl<T: GridNum> From<(T, T)> for Coord<T> {
    fn from(tuple: (T, T)) -> Self {
        Coord(tuple.0, tuple.1)
    }
}

impl<T: GridNum> From<Coord<T>> for (T, T) {
    fn from(coord: Coord<T>) -> Self {
        (coord.0, coord.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_accessors() {
        let c = Coord::new(3u32, 4u32);
        assert_eq!(c.x(), 3u32);
        assert_eq!(c.y(), 4u32);
    }

    #[test]
    fn test_up_down_left_right() {
        let c = Coord::new(5u32, 5u32);
        assert_eq!(c.up(), Some(Coord::new(5, 4)));
        assert_eq!(c.down(), Some(Coord::new(5, 6)));
        assert_eq!(c.left(), Some(Coord::new(4, 5)));
        assert_eq!(c.right(), Some(Coord::new(6, 5)));
    }

    #[test]
    fn test_up_n_down_n_left_n_right_n() {
        let c = Coord::new(10u32, 10u32);
        assert_eq!(c.up_n(3), Some(Coord::new(10, 7)));
        assert_eq!(c.down_n(2), Some(Coord::new(10, 12)));
        assert_eq!(c.left_n(4), Some(Coord::new(6, 10)));
        assert_eq!(c.right_n(5), Some(Coord::new(15, 10)));
    }

    #[test]
    fn test_checked_bounds() {
        let c = Coord::new(0u32, 0u32);
        assert_eq!(c.up(), None);
        assert_eq!(c.left(), None);
        assert_eq!(c.up_n(1), None);
        assert_eq!(c.left_n(1), None);
    }

    #[test]
    fn test_tuple_conversion() {
        let c = Coord::from((7u32, 8u32));
        let t: (u32, u32) = c.into();
        assert_eq!(t, (7, 8));
    }
}
