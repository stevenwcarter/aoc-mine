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
    pub fn up_n(&self, n: Option<T>, min_y: Option<T>) -> Option<Self> {
        let n = n.unwrap_or(T::from(1u8));

        if let Some(min) = min_y
            && (self.1 < n || self.1 - n < min)
        {
            return None;
        }

        let new_y = self.1.checked_sub(&n)?;
        Some(Coord(self.0, new_y))
    }

    pub fn down_n(&self, n: Option<T>, max_y: Option<T>) -> Option<Self> {
        let n = n.unwrap_or(T::from(1u8));

        if let Some(max) = max_y
            && self.1 + n > max
        {
            return None;
        }

        let new_y = self.1.checked_add(&n)?;
        Some(Coord(self.0, new_y))
    }

    pub fn left_n(&self, n: Option<T>, min_x: Option<T>) -> Option<Self> {
        let n = n.unwrap_or(T::from(1u8));

        if let Some(min) = min_x
            && (self.0 < n || self.0 - n < min)
        {
            return None;
        }

        let new_x = self.0.checked_sub(&n)?;
        Some(Coord(new_x, self.1))
    }

    pub fn right_n(&self, n: Option<T>, max_x: Option<T>) -> Option<Self> {
        let n = n.unwrap_or(T::from(1u8));

        if let Some(max) = max_x
            && self.0 + n > max
        {
            return None;
        }

        let new_x = self.0.checked_add(&n)?;
        Some(Coord(new_x, self.1))
    }
    pub fn move_up(&mut self) {
        *self = self.up_n(None, None).unwrap()
    }
    pub fn move_down(&mut self) {
        *self = self.down_n(None, None).unwrap()
    }
    pub fn move_left(&mut self) {
        *self = self.left_n(None, None).unwrap()
    }
    pub fn move_right(&mut self) {
        *self = self.right_n(None, None).unwrap()
    }
    pub fn up(&self, range: Option<T>) -> Option<Self> {
        self.up_n(None, range)
    }
    pub fn down(&self, range: Option<T>) -> Option<Self> {
        self.down_n(None, range)
    }
    pub fn left(&self, range: Option<T>) -> Option<Self> {
        self.left_n(None, range)
    }
    pub fn right(&self, range: Option<T>) -> Option<Self> {
        self.right_n(None, range)
    }
    pub fn up_right(&self, range_x: Option<T>, range_y: Option<T>) -> Option<Self> {
        let new_point = self.up(range_y)?;
        new_point.right(range_x)
    }
    pub fn up_left(&self, range_x: Option<T>, range_y: Option<T>) -> Option<Self> {
        let new_point = self.up(range_y)?;
        new_point.left(range_x)
    }
    pub fn down_right(&self, range_x: Option<T>, range_y: Option<T>) -> Option<Self> {
        let new_point = self.down(range_y)?;
        new_point.right(range_x)
    }
    pub fn down_left(&self, range_x: Option<T>, range_y: Option<T>) -> Option<Self> {
        let new_point = self.down(range_y)?;
        new_point.left(range_x)
    }
    pub fn udlr(&self, udlr: [T; 4]) -> Vec<Self> {
        self.udlr_unfiltered(udlr)
            .iter()
            .filter_map(|p| *p)
            .collect()
    }
    pub fn udlr_unfiltered(&self, udlr: [T; 4]) -> [Option<Self>; 4] {
        [
            self.up(Some(udlr[0])),
            self.down(Some(udlr[1])),
            self.left(Some(udlr[2])),
            self.right(Some(udlr[3])),
        ]
    }

    pub fn points_are_linear(coords: &[Coord<T>]) -> bool {
        // Fewer than 3 points are always collinear
        if coords.len() < 3 {
            return true;
        }

        let (x1, y1) = (coords[0].x(), coords[0].y());
        let (x2, y2) = (coords[1].x(), coords[1].y());
        let ref_dx = x2 - x1;
        let ref_dy = y2 - y1;

        for &coord in &coords[2..] {
            let dx = coord.x() - x1;
            let dy = coord.y() - y1;

            if ref_dx * dy != ref_dy * dx {
                return false;
            }
        }

        true
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
        assert_eq!(c.up(None), Some(Coord::new(5, 4)));
        assert_eq!(c.down(None), Some(Coord::new(5, 6)));
        assert_eq!(c.left(None), Some(Coord::new(4, 5)));
        assert_eq!(c.right(None), Some(Coord::new(6, 5)));
    }

    #[test]
    fn test_up_n_down_n_left_n_right_n() {
        let c = Coord::new(10u32, 10u32);
        assert_eq!(c.up_n(Some(3), None), Some(Coord::new(10, 7)));
        assert_eq!(c.down_n(Some(2), None), Some(Coord::new(10, 12)));
        assert_eq!(c.left_n(Some(4), None), Some(Coord::new(6, 10)));
        assert_eq!(c.right_n(Some(5), None), Some(Coord::new(15, 10)));
    }

    #[test]
    fn test_checked_bounds() {
        let c = Coord::new(0u32, 0u32);
        assert_eq!(c.up(None), None);
        assert_eq!(c.left(None), None);
        assert_eq!(c.up_n(Some(1), None), None);
        assert_eq!(c.left_n(Some(1), None), None);
    }

    #[test]
    fn test_tuple_conversion() {
        let c = Coord::from((7u32, 8u32));
        let t: (u32, u32) = c.into();
        assert_eq!(t, (7, 8));
    }

    #[test]
    fn test_points_are_lineaer() {
        let p1 = Coord::new(0u8, 0u8);
        let p2 = Coord::new(1, 1);
        let p3 = Coord::new(2, 2);

        let coords = [p1, p2, p3];

        assert!(Coord::points_are_linear(&coords));
    }
}
