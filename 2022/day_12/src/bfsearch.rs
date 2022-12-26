use std::collections::VecDeque;
use crate::point::Point;
use crate::common::{Height, height};

pub type Points = Vec<Point>;

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
    parent: Option<Point>,
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

impl<T: Newable + Valuable> Grid<T> {
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


pub fn bfsearch(grid: &mut Grid<Node>) -> Option<Points> {
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();

    let mut root = grid.get_mut(&start).unwrap();
    let mut queue: VecDeque<Point> = VecDeque::new();
    root.visited = true;
    queue.push_back(start.clone());
    while !queue.is_empty() {
        if let Some(current) = queue.pop_front() {
            if current == end {
                let mut path: Points = Vec::new();
                path.push(current.clone());
                let mut node = grid.get(&current).unwrap();
                while let Some(parent) = &node.parent {
                    path.push(parent.clone());
                    node = grid.get(&parent).unwrap();
                }
                path.reverse();
                return Some(path);
            }

            let mut neighbours = find_neighbours(&grid, &current);
            for neighbour in neighbours.drain(0..) {
                if let Some(node) = grid.get_mut(&neighbour) {
                    if node.visited {
                        continue;
                    }
                    node.visited = true;
                    node.parent = Some(current.clone());
                    queue.push_back(neighbour);
                }
            }
        }
    }

    None
}

fn find_neighbours(grid: &Grid<Node>, center: &Point) -> Points {
    let (x, y) = (center.x, center.y);
    let mut neighbours = Vec::new();
    if x + 1 < grid.len_x() { // right
        neighbours.push(Point::new(x + 1, y));
    }
    if x > 0 { // left
        neighbours.push(Point::new(x - 1, y));
    }
    if y > 0 { // up
        neighbours.push(Point::new(x, y - 1));
    }
    if y + 1 < grid.len_y() { // down
        neighbours.push(Point::new(x, y + 1));
    }

    let mut vetted = Vec::new();
    for neighbour in neighbours.drain(..) {
        let center_height = height(grid.get(&center).unwrap().value) as isize;
        if is_vetted(&grid, center_height, &neighbour) {
            vetted.push(neighbour)
        }
    }
    // println!("   Vetted Neighbours {:?}", &vetted);
    vetted
}

fn is_vetted(grid: &Grid<Node>, center_height: isize, neighbour: &Point) -> bool {
    if let Some(node) = grid.get(neighbour) {
        if node.visited {
            return false;
        }

        let neighbour_height = height(node.value) as isize;
        let diff = (center_height - neighbour_height).abs();
        // println!("   Comparing with {:?} (height: {}): {}", neighbour, neighbour_height, diff);
        if diff > 1 {
            false
        } else {
            true
        }
    } else {
        false
    }
}

#[cfg(test)]
mod bfs_tests {
    use super::*;

    fn make_grid(lines: &Vec<String>) -> Grid<Node> {
        let mut grid = Grid::new();
        for line in lines.iter() {
            grid.push(&line);
        }
        grid
    }

    fn example_input() -> Vec<String> {
        vec![
            "Sabqponm".into(),
            "abcryxxl".into(),
            "accszExk".into(),
            "acctuvwj".into(),
            "abdefghi".into(),
        ]
    }

    fn print(title: &str, path: &Option<Points>) {
        if let Some(path) = path {
            println!("{} ({}):", title, path.len());
            for point in path.iter() {
                println!("   {:?}", point);
            }
        } else {
            println!("{}: None", title);
        }
    }

    #[test]
    fn test_on_simple_grid() {
        let input = vec![
            "Sabcde".into(),
            "yzEdgf".into(),
            "xwvutg".into(),
            "opqrsh".into(),
            "nmlkji".into(),
        ];
        let mut grid = make_grid(&input);
        let path = bfsearch(&mut grid);
        assert_eq!(path, Some(vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 0),
                Point::new(5, 0),
                Point::new(5, 1),
                Point::new(5, 2),
                Point::new(5, 3),
                Point::new(5, 4),

                Point::new(4, 4),
                Point::new(3, 4),
                Point::new(2, 4),
                Point::new(1, 4),
                Point::new(0, 4),

                Point::new(0, 3),
                Point::new(1, 3),
                Point::new(2, 3),
                Point::new(3, 3),
                Point::new(4, 3),

                Point::new(4, 2),
                Point::new(3, 2),
                Point::new(2, 2),
                Point::new(1, 2),
                Point::new(0, 2),

                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
                ]));
    }

    #[test]
    fn test_on_example() {
        // let expected: Vec<String> = vec![
        //     "v..v<<<<".into(),
        //     ">v.vv<<^".into(),
        //     ".>vv>E^^".into(),
        //     "..v>>>^^".into(),
        //     "..>>>>>^".into(),
        // ];
        let expected = Some(vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(1, 2),
            Point::new(2, 2),
            Point::new(2, 3),
            Point::new(2, 4),
            Point::new(3, 4),
            Point::new(4, 4),
            Point::new(5, 4),
            Point::new(6, 4),
            Point::new(7, 4),
            Point::new(7, 3),
            Point::new(7, 2),
            Point::new(7, 1),
            Point::new(7, 0),
            Point::new(6, 0),
            Point::new(5, 0),
            Point::new(4, 0),
            Point::new(3, 0),
            Point::new(3, 1),
            Point::new(3, 2),
            Point::new(3, 3),
            Point::new(4, 3),
            Point::new(5, 3),
            Point::new(6, 3),
            Point::new(6, 2),
            Point::new(6, 1),
            Point::new(5, 1),
            Point::new(4, 1),
            Point::new(4, 2),
            Point::new(5, 2),
            ]);
        let mut grid = make_grid(&example_input());
        let path = bfsearch(&mut grid);
        print("Path", &path);
        print("Expected", &expected);
        // assert_eq!(path, expected);
        if let Some(path) = path {
            if let Some(expected) = expected {
                assert_eq!(path.len(), expected.len());
            } else {
                assert!(false, "Expected is None")
            }
        } else {
            assert!(false, "Path is None")
        }
    }
}
