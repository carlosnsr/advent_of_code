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

    let mut path = match hill_climb(&grid, &start, &end, &mut visited) {
        Some(mut path) => {
            path.push(start.clone());
            path.reverse();
            Some(path)
        },
        None => None,
    };

    path
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
    if x + 1 < grid.len_x() {
        let neighbour = Point::new(x + 1, y);
        if !visited.contains(&neighbour) {
            neighbours.push(neighbour); // right
        }
    }
    if x > 0 {
        let neighbour = Point::new(x - 1, y);
        if !visited.contains(&neighbour) {
            neighbours.push(neighbour); // left
        }
    }
    if y > 0 {
        let neighbour = Point::new(x, y - 1);
        if !visited.contains(&neighbour) {
            neighbours.push(neighbour); // up
        }
    }
    if y + 1 < grid.len_y() {
        let neighbour = Point::new(x, y + 1);
        if !visited.contains(&neighbour) {
            neighbours.push(neighbour); // down
        }
    }

    neighbours
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
