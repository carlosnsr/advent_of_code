#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
