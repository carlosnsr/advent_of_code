use crate::point::Point;
use crate::common::Height;

pub trait Newable {
    fn new(value: Height) -> Self;
}

pub trait Valuable {
    fn value(&self) -> &Height;
}

#[derive(Debug)]
pub struct Node {
    pub value: Height,
    pub visited: bool,
    pub parent: Option<Point>,
}

impl Newable for Node {
    fn new(value: Height) -> Self {
        Self {
            value,
            visited: false,
            parent: None,
        }
    }
}

impl Valuable for Node {
    fn value(&self) -> &Height {
        &self.value
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T> where T: Newable + Valuable {
    pub fn new() -> Self {
        Self { grid: Vec::new() }
    }

    pub fn push(&mut self, line: &String) {
        let row: Vec<T> = line.chars().map(|c| T::new(c)).collect();
        self.grid.push(row);
    }

    pub fn find(&self, value: Height) -> Option<Point> {
        for y in 0..self.len_y() {
            for x in 0..self.len_x() {
                if *self.grid[y][x].value() == value {
                    return Some(Point::new(x, y));
                }
            }
        }
        None
    }

    pub fn get_mut(&mut self, target: &Point) -> Option<&mut T> {
        let (x, y) = (target.x, target.y);
        Some(&mut self.grid[y][x])
    }

    pub fn get(&self, target: &Point) -> Option<&T> {
        let (x, y) = (target.x, target.y);
        Some(&self.grid[y][x])
    }

    pub fn len_x(&self) -> usize {
        self.grid[0].len()
    }

    pub fn len_y(&self) -> usize {
        self.grid.len()
    }
}
