use crate::point::{Point, Points};

pub trait Newable {
    fn new(value: Cell) -> Self;
}

pub trait Valuable {
    fn value(&self) -> &Cell;
}

#[derive(Debug, PartialEq)]
pub enum Cell {
    Start,
    End,
    Area(u8),
}

impl Cell {
    pub fn height(&self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => 25, // i.e. b'z' - b'a'
            Cell::Area(h) => *h,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: Cell,
    pub visited: bool,
    pub parent: Option<Point>,
}

impl Newable for Node {
    fn new(value: Cell) -> Self {
        Self {
            value,
            visited: false,
            parent: None,
        }
    }
}

impl Valuable for Node {
    fn value(&self) -> &Cell {
        &self.value
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> where T: Newable + Valuable {
    pub fn new() -> Self {
        Self { grid: Vec::new(), width: 0, height: 0 }
    }

    pub fn push(&mut self, line: &String) {
        let row: Vec<T> = line
            .chars()
            .map(|c| match c {
                'S' => Cell::Start,
                'E' => Cell::End,
                'a'..='z' => Cell::Area(c as u8 - b'a'),
                _ => panic!("Invalid character *{}*", c),
            })
            .map(|cell| T::new(cell))
            .collect();
        self.width = row.len();
        self.height += 1;
        self.grid.push(row);
    }

    pub fn find(&self, value: Cell) -> Option<Point> {
        for y in 0..self.height {
            for x in 0..self.width {
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

    pub fn get_walkable_neighbours(&self, target: &Point) -> Points {
        let target_height = self.get(target).unwrap().value().height();
        let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas
            .into_iter()
            .filter_map(move |(dx, dy)|
                Some(Point {
                    x: target.x.checked_add_signed(dx)?,
                    y: target.y.checked_add_signed(dy)?,
                })
            )
            .filter(|point| self.in_bounds(&point))
            .filter(|point| {
                let height = self.get(&point).unwrap().value().height();
                height <= target_height + 1
            })
        .collect()
    }

    pub fn in_bounds(&self, point: &Point) -> bool {
        point.x < self.width && point.y < self.height
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}
