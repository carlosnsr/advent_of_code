use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "input";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut grid: Grid = Grid::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(&line);
    }

    println!("The grid is {:?}", grid);
}

type Height = char;
type Heights = Vec<Height>;

#[derive(Debug)]
struct Grid {
    grid: Vec<Heights>,
}

impl Grid {
    fn new() -> Self {
        Self { grid: Vec::new() }
    }

    fn push(&mut self, line: &String) {
        let heights = line.chars().collect::<Heights>();
        self.grid.push(heights);
    }

    fn find(&self, value: Height) -> Option<Point> {
        for y in 0..self.len_y() {
            for x in 0..self.len_x() {
                if self.grid[y][x] == value {
                    return Some(Point::new(x, y));
                }
            }
        }
        None
    }

    fn get(&self, target: &Point) -> Option<Height> {
        let (x, y) = (target.x, target.y);
        Some(self.grid[y][x])
    }

    fn len_x(&self) -> usize {
        self.grid[0].len()
    }

    fn len_y(&self) -> usize {
        self.grid.len()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self{ x, y }
    }
}

fn climb_to_top(grid: &Grid) -> Option<Vec<Point>> {
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    let mut visited: HashSet<Point> = HashSet::new();

    match hill_climb(&grid, &start, &end, &mut visited) {
        Some(mut path) => {
            path.push(start.clone());
            path.reverse();
            Some(path)
        },
        None => None,
    }
}

fn hill_climb(grid: &Grid, current: &Point, end: &Point, visited: &mut HashSet<Point>) -> Option<Vec<Point>> {
    visited.insert(current.clone());
    if current == end {
        return Some(vec![]);
    } else {
        let neighbours = find_neighbours(grid, current, &visited);
        for i in 0..neighbours.len() {
            match hill_climb(&grid, &neighbours[i], &end, visited) {
                Some(mut path) => {
                    path.push(neighbours[i].clone());
                    return Some(path);
                },
                None => (),
            }
        }
    }

    None
}

fn find_neighbours(grid: &Grid, center: &Point, visited: &HashSet<Point>) -> Vec<Point> {
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
        if is_vetted(&grid, &center, &neighbour, &visited) {
            vetted.push(neighbour)
        }
    }
    vetted
}

fn is_vetted(grid: &Grid, center: &Point, neighbour: &Point, visited: &HashSet<Point>) -> bool {
    if visited.contains(&neighbour) {
        return false;
    }

    let center_height = height(grid.get(&center).unwrap()).unwrap();
    let neighbour_height = match height(grid.get(&neighbour).unwrap()) {
        Some(value) => value,
        None => center_height + 1,
    };
    let diff = if center_height > neighbour_height {
        center_height - neighbour_height
    } else {
        neighbour_height - center_height
    };

    if diff > 1 {
        false
    } else {
        true
    }
}

fn height(value: Height) -> Option<usize> {
    match value {
        'S' => Some(0), // TODO: should match 'a'
        'E' => None, // TODO: should match 'z'
        _ => Some(value as usize - 'a' as usize),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_grid(lines: &Vec<String>) -> Grid {
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

    /*
    #[ignore]
    #[test]
    fn test_hill_climb() {
        let expected: Vec<String> = vec![
            "v..v<<<<".into(),
            ">v.vv<<^".into(),
            ".>vv>E^^".into(),
            "..v>>>^^".into(),
            "..>>>>>^".into(),
        ];

        let paths = climb(&input_grid());
        println!("{:?}", paths);
        assert!(false);
        // assert_eq!(climb(input_grid), expected);
    }
    */

    #[test]
    fn test_little_hill_climbs() {
        let mut input = vec![
            "Sa".into(),
            "bE".into(),
        ];
        let mut grid = make_grid(&input);
        let mut path = climb_to_top(&grid);
        // println!("{:?}", path);
        assert_eq!(path, Some(vec![Point::new(0, 0), Point::new(1, 0), Point::new(1, 1)]));

        input = vec![
            "Sac".into(),
            "bbd".into(),
            "dcE".into(),
        ];
        grid = make_grid(&input);
        path = climb_to_top(&grid);
        // println!("{:?}", path);
        assert_eq!(
            path,
            Some(vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(1, 1),
                Point::new(1, 2),
                Point::new(2, 2),
            ])
        );
    }
}
