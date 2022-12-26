use std::cmp::{Ord, Ordering};

pub type Points = Vec<Point>;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self{ x, y }
    }

    pub fn distance(&self, other: &Point) -> f32 {
        let (dx, dy): (f32, f32) = (
            (other.x as isize - self.x as isize) as f32,
            (other.y as isize - self.y as isize) as f32
        );

        (dx * dx + dy * dy).sqrt()
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let (x, y) = (self.x, self.y);
        let (x2, y2) = (other.x, other.y);
        if x == x2 && y == y2 {
            Ordering::Equal
        } else if self.distance(other) < 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ord() {
        assert!(Point::new(12, 13) == Point::new(12, 13));
        assert!(Point::new(10, 10) < Point::new(12, 13));
        assert!(Point::new(10, 10) > Point::new(5, 5));
    }
}
